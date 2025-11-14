use std::path::PathBuf;
use std::process::Command;
use std::thread;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
use winit::window::{WindowBuilder, WindowLevel};

use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Clone, Debug)]
enum UserEvent {
    OpenFile,
    QuickLook,
    RestoreTop,
    OpenSettings,
    ApplySettings,
}

static EVENT_PROXY: Lazy<Mutex<Option<EventLoopProxy<UserEvent>>>> = Lazy::new(|| Mutex::new(None));

#[derive(Clone, Copy, Debug)]
struct Settings {
    fit_window: bool,
    aspect_lock: bool,
}

static SETTINGS: Lazy<Mutex<Settings>> = Lazy::new(|| Mutex::new(Settings {
    fit_window: false,
    aspect_lock: false,
}));

static IMAGE_ASPECT: Lazy<Mutex<Option<f64>>> = Lazy::new(|| Mutex::new(None));
static RESIZE_GUARD: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[cfg(target_os = "macos")]
mod macos_menu {
    use super::{UserEvent, EVENT_PROXY};
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy, NSMenu, NSMenuItem};
    use cocoa::base::nil;
    use cocoa::foundation::{NSAutoreleasePool, NSString};
    use objc::declare::ClassDecl;
    use objc::runtime::{Object, Sel};
    use objc::{class, msg_send, sel, sel_impl};
    use std::sync::Once;

    static mut HANDLER_INSTANCE: *mut Object = std::ptr::null_mut();
    static INIT: Once = Once::new();

    extern "C" fn rust_open_file(_this: &Object, _sel: Sel, _sender: *mut Object) {
        if let Some(proxy) = EVENT_PROXY.lock().ok().and_then(|g| g.as_ref().cloned()) {
            let _ = proxy.send_event(UserEvent::OpenFile);
        }
    }

    extern "C" fn rust_quick_look(_this: &Object, _sel: Sel, _sender: *mut Object) {
        if let Some(proxy) = EVENT_PROXY.lock().ok().and_then(|g| g.as_ref().cloned()) {
            let _ = proxy.send_event(UserEvent::QuickLook);
        }
    }

    extern "C" fn rust_open_settings(_this: &Object, _sel: Sel, _sender: *mut Object) {
        if let Some(proxy) = EVENT_PROXY.lock().ok().and_then(|g| g.as_ref().cloned()) {
            let _ = proxy.send_event(UserEvent::OpenSettings);
        }
    }

    unsafe fn get_handler_instance() -> *mut Object {
        INIT.call_once(|| {
            // Register class
            let superclass = class!(NSObject);
            let mut decl = ClassDecl::new("RustMenuHandler", superclass).unwrap();
            decl.add_method(sel!(rustOpenFile:), rust_open_file as extern "C" fn(&Object, Sel, *mut Object));
            decl.add_method(sel!(rustQuickLook:), rust_quick_look as extern "C" fn(&Object, Sel, *mut Object));
            decl.add_method(sel!(rustOpenSettings:), rust_open_settings as extern "C" fn(&Object, Sel, *mut Object));
            let cls = decl.register();
            let obj: *mut Object = msg_send![cls, new];
            HANDLER_INSTANCE = obj;
        });
        HANDLER_INSTANCE
    }

    pub unsafe fn install_menubar() {
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);

        // Main menubar with three items: App, File, View
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        let file_menu_item = NSMenuItem::new(nil).autorelease();
        let view_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        menubar.addItem_(file_menu_item);
        menubar.addItem_(view_menu_item);
        app.setMainMenu_(menubar);

        // App menu (Settings…, Quit)
        let app_menu = NSMenu::new(nil).autorelease();
        let handler = get_handler_instance();
        let settings_title = NSString::alloc(nil).init_str("Settings…");
        let comma_key = NSString::alloc(nil).init_str(",");
        let settings_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(settings_title, sel!(rustOpenSettings:), comma_key)
            .autorelease();
        let _: () = msg_send![settings_item, setTarget: handler];
        app_menu.addItem_(settings_item);

        let quit_title = NSString::alloc(nil).init_str("Quit Always On Top");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, sel!(terminate:), quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        // File menu (Open... Cmd+O)
        let file_menu = NSMenu::new(nil).autorelease();
        let open_title = NSString::alloc(nil).init_str("Open…");
        let o_key = NSString::alloc(nil).init_str("o");
        let open_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(open_title, sel!(rustOpenFile:), o_key)
            .autorelease();
        // Target our handler
        let _: () = msg_send![open_item, setTarget: handler];
        file_menu.addItem_(open_item);
        file_menu_item.setSubmenu_(file_menu);

        // View menu (Quick Look Cmd+Y)
        let view_menu = NSMenu::new(nil).autorelease();
        let ql_title = NSString::alloc(nil).init_str("Quick Look");
        let y_key = NSString::alloc(nil).init_str("y");
        let ql_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(ql_title, sel!(rustQuickLook:), y_key)
            .autorelease();
        let _: () = msg_send![ql_item, setTarget: handler];
        view_menu.addItem_(ql_item);
        view_menu_item.setSubmenu_(view_menu);
    }
}

#[cfg(target_os = "macos")]
mod macos_image {
    use cocoa::appkit::{NSImage, NSImageView, NSView, NSScreen};
    use cocoa::base::nil;
    use cocoa::foundation::{NSAutoreleasePool, NSString};
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl};
    use winit::platform::macos::WindowExtMacOS;

    const IMAGE_TAG: i64 = 4242;

    unsafe fn get_root_view(window: &winit::window::Window) -> *mut Object {
        window.ns_view() as *mut Object
    }

    static mut LAST_SIZE: (f64, f64) = (0.0, 0.0);

    pub unsafe fn set_image(window: &winit::window::Window, path: &std::path::Path) -> Option<(f64, f64)> {
        let ns_view = get_root_view(window);
        if ns_view.is_null() {
            return None;
        }

        // Create or find the image view.
        let existing: *mut Object = msg_send![ns_view, viewWithTag: IMAGE_TAG];
        let image_view: *mut Object = if existing.is_null() {
            let bounds = NSView::bounds(ns_view);
            let iv: *mut Object = msg_send![class!(NSImageView), alloc];
            let iv: *mut Object = msg_send![iv, initWithFrame: bounds];
            let _: () = msg_send![iv, setTag: IMAGE_TAG];
            // Scale proportionally up or down (3)
            let _: () = msg_send![iv, setImageScaling: 3_u64];
            let _: () = msg_send![ns_view, addSubview: iv];
            iv
        } else {
            existing
        };

        // Load NSImage from file
        let ns_path = NSString::alloc(nil).init_str(&path.to_string_lossy());
        let image_cls = class!(NSImage);
        let img_alloc: *mut Object = msg_send![image_cls, alloc];
        let ns_image: *mut Object = msg_send![img_alloc, initWithContentsOfFile: ns_path];
        if ns_image.is_null() {
            return None;
        }
        let _: () = msg_send![image_view, setImage: ns_image];
        // Extract image size in points
        let size: cocoa::foundation::NSSize = msg_send![ns_image, size];
        LAST_SIZE = (size.width as f64, size.height as f64);
        layout_image_view(window);
        Some(LAST_SIZE)
    }

    pub unsafe fn layout_image_view(window: &winit::window::Window) {
        let ns_view = get_root_view(window);
        if ns_view.is_null() {
            return;
        }
        let bounds = NSView::bounds(ns_view);
        let sub: *mut Object = msg_send![ns_view, viewWithTag: IMAGE_TAG];
        if !sub.is_null() {
            let _: () = msg_send![sub, setFrame: bounds];
        }
    }

    pub unsafe fn last_image_size() -> Option<(f64, f64)> {
        if LAST_SIZE.0 > 0.0 && LAST_SIZE.1 > 0.0 { Some(LAST_SIZE) } else { None }
    }

    pub unsafe fn clamp_to_screen(window: &winit::window::Window, mut w: f64, mut h: f64) -> (f64, f64) {
        // Use main screen visible frame as a simple bound.
        let screen = NSScreen::mainScreen(nil);
        if !screen.is_null() {
            let frame: cocoa::foundation::NSRect = msg_send![screen, visibleFrame];
            let max_w = frame.size.width as f64 * 0.9;
            let max_h = frame.size.height as f64 * 0.9;
            let scale_w = max_w / w;
            let scale_h = max_h / h;
            let scale = scale_w.min(scale_h).min(1.0);
            w *= scale;
            h *= scale;
        }
        (w, h)
    }
}

#[cfg(target_os = "macos")]
mod macos_settings {
    use super::{UserEvent, EVENT_PROXY, SETTINGS};
    use cocoa::appkit::{NSButton, NSView};
    use cocoa::base::nil;
    use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl};

    pub unsafe fn show_settings_modal() {
        let alert: *mut Object = msg_send![class!(NSAlert), new];
        let title = NSString::alloc(nil).init_str("Settings");
        let _: () = msg_send![alert, setMessageText: title];
        let ok = NSString::alloc(nil).init_str("OK");
        let _: () = msg_send![alert, addButtonWithTitle: ok];

        // Accessory view with two checkboxes
        let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(280.0, 70.0));
        let accessory: *mut Object = msg_send![class!(NSView), alloc];
        let accessory: *mut Object = msg_send![accessory, initWithFrame: frame];

        // Fit window to image checkbox
        let cb1: *mut Object = msg_send![class!(NSButton), alloc];
        let cb1: *mut Object = msg_send![cb1, initWithFrame: NSRect::new(NSPoint::new(0.0, 40.0), NSSize::new(280.0, 24.0))];
        let t1 = NSString::alloc(nil).init_str("Fit window to image");
        let _: () = msg_send![cb1, setTitle: t1];
        // NSButtonTypeSwitch = 3
        let _: () = msg_send![cb1, setButtonType: 3_i64];

        // Lock aspect ratio checkbox
        let cb2: *mut Object = msg_send![class!(NSButton), alloc];
        let cb2: *mut Object = msg_send![cb2, initWithFrame: NSRect::new(NSPoint::new(0.0, 12.0), NSSize::new(280.0, 24.0))];
        let t2 = NSString::alloc(nil).init_str("Lock aspect ratio on resize");
        let _: () = msg_send![cb2, setTitle: t2];
        let _: () = msg_send![cb2, setButtonType: 3_i64];

        // Initialize states from current settings
        if let Ok(s) = SETTINGS.lock() {
            let fit_state: i64 = if s.fit_window { 1 } else { 0 };
            let aspect_state: i64 = if s.aspect_lock { 1 } else { 0 };
            let _: () = msg_send![cb1, setState: fit_state];
            let _: () = msg_send![cb2, setState: aspect_state];
        }

        let _: () = msg_send![accessory, addSubview: cb1];
        let _: () = msg_send![accessory, addSubview: cb2];
        let _: () = msg_send![alert, setAccessoryView: accessory];

        let _: i64 = msg_send![alert, runModal];

        // Read states back and apply
        let s1: i64 = msg_send![cb1, state];
        let s2: i64 = msg_send![cb2, state];
        if let Ok(mut s) = SETTINGS.lock() {
            s.fit_window = s1 != 0;
            s.aspect_lock = s2 != 0;
        }

        // Notify app to apply settings (e.g., fit window now)
        if let Some(proxy) = EVENT_PROXY.lock().ok().and_then(|g| g.as_ref().cloned()) {
            let _ = proxy.send_event(UserEvent::ApplySettings);
        }
    }
}

fn main() {
    // Create the event loop and the window configured to stay on top.
    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::with_user_event().build();
    let proxy = event_loop.create_proxy();
    {
        if let Ok(mut slot) = EVENT_PROXY.lock() {
            *slot = Some(proxy.clone());
        }
    }
    let window = WindowBuilder::new()
        .with_title("Always On Top")
        .with_inner_size(LogicalSize::new(420.0, 120.0))
        .build(&event_loop)
        .expect("Failed to create window");
    window.set_window_level(WindowLevel::AlwaysOnTop);

    // Install a minimal macOS menubar with File->Open and View->Quick Look
    #[cfg(target_os = "macos")]
    unsafe {
        macos_menu::install_menubar();
    }

    // Ask the user to select a file immediately on launch.
    let mut selected: Option<PathBuf> = open_file_dialog_and_set_title(&window);
    #[cfg(target_os = "macos")]
    if let Some(ref p) = selected {
        unsafe {
            if let Some((w, h)) = macos_image::set_image(&window, p) {
                let aspect = if h > 0.0 { w / h } else { 0.0 };
                if aspect > 0.0 {
                    if let Ok(mut a) = IMAGE_ASPECT.lock() { *a = Some(aspect); }
                }
                if let Ok(s) = SETTINGS.lock() { if s.fit_window {
                    let (cw, ch) = macos_image::clamp_to_screen(&window, w, h);
                    window.set_inner_size(LogicalSize::new(cw, ch));
                }}
            }
        }
    }

    // Basic event loop: keep running until the user closes the window.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(new_size) => {
                    #[cfg(target_os = "macos")]
                    unsafe {
                        macos_image::layout_image_view(&window);
                    }
                    // Enforce aspect ratio if enabled
                    if let Ok(s) = SETTINGS.lock() {
                        if s.aspect_lock {
                            if let Ok(aspect) = IMAGE_ASPECT.lock() {
                                if let Some(r) = *aspect {
                                    if let Ok(mut guard) = RESIZE_GUARD.lock() {
                                        if !*guard {
                                            let scale = window.scale_factor();
                                            let mut lw = new_size.width as f64 / scale;
                                            let mut lh = new_size.height as f64 / scale;
                                            let current = if lh > 0.0 { lw / lh } else { r };
                                            if (current - r).abs() > 0.001 {
                                                // Adjust the dimension that deviates the most
                                                let adj_w = lh * r;
                                                let adj_h = lw / r;
                                                if (lw - adj_w).abs() > (lh - adj_h).abs() {
                                                    lw = adj_w;
                                                } else {
                                                    lh = adj_h;
                                                }
                                                *guard = true;
                                                window.set_inner_size(LogicalSize::new(lw, lh));
                                                *guard = false;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    #[cfg(target_os = "macos")]
                    unsafe {
                        macos_image::layout_image_view(&window);
                    }
                }
                _ => {}
            },
            Event::UserEvent(UserEvent::OpenFile) => {
                selected = open_file_dialog_and_set_title(&window);
                #[cfg(target_os = "macos")]
                if let Some(ref p) = selected {
                    unsafe {
                        if let Some((w, h)) = macos_image::set_image(&window, p) {
                            let aspect = if h > 0.0 { w / h } else { 0.0 };
                            if aspect > 0.0 {
                                if let Ok(mut a) = IMAGE_ASPECT.lock() { *a = Some(aspect); }
                            }
                            if let Ok(s) = SETTINGS.lock() { if s.fit_window {
                                let (cw, ch) = macos_image::clamp_to_screen(&window, w, h);
                                window.set_inner_size(LogicalSize::new(cw, ch));
                            }}
                        }
                    }
                }
            }
            Event::UserEvent(UserEvent::QuickLook) => {
                if let Some(path) = selected.as_ref() {
                    // Temporarily drop window to normal level so Quick Look isn't obscured.
                    window.set_window_level(WindowLevel::Normal);
                    quick_look(path.clone());
                }
            }
            Event::UserEvent(UserEvent::OpenSettings) => {
                #[cfg(target_os = "macos")]
                unsafe { macos_settings::show_settings_modal(); }
            }
            Event::UserEvent(UserEvent::ApplySettings) => {
                #[cfg(target_os = "macos")]
                unsafe {
                    if let Ok(s) = SETTINGS.lock() {
                        if s.fit_window {
                            if let Some((w, h)) = macos_image::last_image_size() {
                                let (cw, ch) = macos_image::clamp_to_screen(&window, w, h);
                                window.set_inner_size(LogicalSize::new(cw, ch));
                            }
                        }
                    }
                }
            }
            Event::UserEvent(UserEvent::RestoreTop) => {
                window.set_window_level(WindowLevel::AlwaysOnTop);
            }
            Event::LoopDestroyed => {
                // Drop selected path if any (no-op but explicit ownership end).
                let _ = &selected;
            }
            _ => {}
        }
    });
}

fn open_file_dialog_and_set_title(window: &winit::window::Window) -> Option<PathBuf> {
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Select a file to pin on top")
        .pick_file()
    {
        let title = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => format!("Pinned: {}", name),
            None => "Pinned: <unnamed>".to_string(),
        };
        window.set_title(&title);
        return Some(path);
    }
    None
}

#[cfg(target_os = "macos")]
fn quick_look(path: PathBuf) {
    // Run Quick Look in a background thread; when it finishes, restore topmost.
    thread::spawn(move || {
        let _ = Command::new("qlmanage").arg("-p").arg(&path).status();
        if let Some(proxy) = EVENT_PROXY.lock().ok().and_then(|g| g.as_ref().cloned()) {
            let _ = proxy.send_event(UserEvent::RestoreTop);
        }
    });
}

#[cfg(not(target_os = "macos"))]
fn quick_look(_path: PathBuf) {
    // No-op on non-macOS platforms.
}
