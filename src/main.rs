#![allow(unexpected_cfgs)] // Allow objc macro cfg probes under clippy

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::thread;

use winit::dpi::LogicalSize;
use winit::event::{
    ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent,
};
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
    FitNow,
}

static EVENT_PROXY: Lazy<Mutex<Option<EventLoopProxy<UserEvent>>>> = Lazy::new(|| Mutex::new(None));

#[derive(Clone, Copy, Debug)]
struct Settings {
    fit_window: bool,
    aspect_lock: bool,
}

static SETTINGS: Lazy<Mutex<Settings>> = Lazy::new(|| {
    Mutex::new(Settings {
        fit_window: false,
        aspect_lock: false,
    })
});

static IMAGE_ASPECT: Lazy<Mutex<Option<f64>>> = Lazy::new(|| Mutex::new(None));
static RESIZE_GUARD: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistedState {
    fit_window: bool,
    aspect_lock: bool,
    last_file: Option<String>,
    window_w: Option<f64>,
    window_h: Option<f64>,
}

fn config_path() -> Option<std::path::PathBuf> {
    if let Some(proj) = directories::ProjectDirs::from("com", "example", "Always On Top") {
        let mut p = proj.config_dir().to_path_buf();
        p.push("settings.json");
        Some(p)
    } else {
        None
    }
}

fn load_persisted() -> Option<PersistedState> {
    let path = config_path()?;
    let mut s = String::new();
    if let Ok(mut f) = fs::File::open(path) {
        if f.read_to_string(&mut s).is_ok() {
            if let Ok(st) = serde_json::from_str::<PersistedState>(&s) {
                return Some(st);
            }
        }
    }
    None
}

fn save_persisted(selected: Option<&PathBuf>, window: &winit::window::Window) {
    let path = match config_path() {
        Some(p) => p,
        None => return,
    };
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    let (fit_window, aspect_lock) = {
        let s = SETTINGS.lock().ok();
        if let Some(s) = s {
            (s.fit_window, s.aspect_lock)
        } else {
            (false, false)
        }
    };
    let last_file = selected.and_then(|p| p.to_str().map(|s| s.to_string()));
    // Persist current window logical size
    let sf = window.scale_factor();
    let size = window.inner_size();
    let window_w = Some(size.width as f64 / sf);
    let window_h = Some(size.height as f64 / sf);
    let state = PersistedState {
        fit_window,
        aspect_lock,
        last_file,
        window_w,
        window_h,
    };
    if let Ok(data) = serde_json::to_string_pretty(&state) {
        if let Ok(mut f) = fs::File::create(path) {
            let _ = f.write_all(data.as_bytes());
        }
    }
}

#[cfg(target_os = "macos")]
mod macos_menu {
    use super::{UserEvent, EVENT_PROXY};
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy, NSMenu, NSMenuItem};
    use cocoa::base::{nil, YES};
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

    extern "C" fn rust_fit_now(_this: &Object, _sel: Sel, _sender: *mut Object) {
        if let Some(proxy) = EVENT_PROXY.lock().ok().and_then(|g| g.as_ref().cloned()) {
            let _ = proxy.send_event(UserEvent::FitNow);
        }
    }

    pub unsafe fn get_handler_instance() -> *mut Object {
        INIT.call_once(|| {
            // Register class
            let superclass = class!(NSObject);
            let mut decl = ClassDecl::new("RustMenuHandler", superclass).unwrap();
            decl.add_method(
                sel!(rustOpenFile:),
                rust_open_file as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(rustQuickLook:),
                rust_quick_look as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(rustFitNow:),
                rust_fit_now as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(rustOpenSettings:),
                rust_open_settings as extern "C" fn(&Object, Sel, *mut Object),
            );
            let cls = decl.register();
            let obj: *mut Object = msg_send![cls, new];
            HANDLER_INSTANCE = obj;
        });
        HANDLER_INSTANCE
    }

    pub unsafe fn install_menubar() {
        let app = NSApp();
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        );

        // Main menubar with three items: App, File, View
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        let file_menu_item = NSMenuItem::new(nil).autorelease();
        let view_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        menubar.addItem_(file_menu_item);
        menubar.addItem_(view_menu_item);
        app.setMainMenu_(menubar);
        // Ensure app becomes active so menu is accessible
        app.activateIgnoringOtherApps_(YES);

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

        // View menu (Fit Now Cmd+F, Quick Look Cmd+Y)
        let view_menu = NSMenu::new(nil).autorelease();
        // Fit to Image Now
        let fit_title = NSString::alloc(nil).init_str("Fit to Image Now");
        let f_key = NSString::alloc(nil).init_str("f");
        let fit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(fit_title, sel!(rustFitNow:), f_key)
            .autorelease();
        let _: () = msg_send![fit_item, setTarget: handler];
        view_menu.addItem_(fit_item);
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
    use cocoa::appkit::{NSScreen, NSView};
    use cocoa::base::nil;
    use cocoa::foundation::NSString;
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl};
    use once_cell::sync::Lazy;
    use std::sync::Mutex;
    use winit::platform::macos::WindowExtMacOS;

    const IMAGE_TAG: i64 = 4242;
    static LAST_SIZE: Lazy<Mutex<Option<(f64, f64)>>> = Lazy::new(|| Mutex::new(None));

    fn get_root_view(window: &winit::window::Window) -> *mut Object {
        // winit guarantees the NSView pointer is valid for the window lifetime.
        window.ns_view() as *mut Object
    }

    pub fn set_image(window: &winit::window::Window, path: &std::path::Path) -> Option<(f64, f64)> {
        let ns_view = get_root_view(window);
        if ns_view.is_null() {
            return None;
        }

        // Create or find the image view.
        let image_view: *mut Object = unsafe {
            let existing: *mut Object = msg_send![ns_view, viewWithTag: IMAGE_TAG];
            if existing.is_null() {
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
            }
        };

        // Load NSImage from file
        let last_size = unsafe {
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
            (size.width as f64, size.height as f64)
        };

        if let Ok(mut guard) = LAST_SIZE.lock() {
            *guard = Some(last_size);
        }
        layout_image_view(window);
        Some(last_size)
    }

    pub fn layout_image_view(window: &winit::window::Window) {
        let ns_view = get_root_view(window);
        if ns_view.is_null() {
            return;
        }
        let bounds = unsafe { NSView::bounds(ns_view) };
        let sub: *mut Object = unsafe { msg_send![ns_view, viewWithTag: IMAGE_TAG] };
        if !sub.is_null() {
            unsafe {
                let _: () = msg_send![sub, setFrame: bounds];
            }
        }
    }

    pub fn last_image_size() -> Option<(f64, f64)> {
        LAST_SIZE.lock().ok().and_then(|g| (*g).as_ref().copied())
    }

    pub fn clamp_to_screen(mut w: f64, mut h: f64) -> (f64, f64) {
        // Use main screen visible frame as a simple bound.
        let screen = unsafe { NSScreen::mainScreen(nil) };
        if !screen.is_null() {
            let frame: cocoa::foundation::NSRect = unsafe { msg_send![screen, visibleFrame] };
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
    use super::macos_menu::get_handler_instance;
    use super::{UserEvent, EVENT_PROXY, SETTINGS};
    use cocoa::base::nil;
    use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl};

    pub fn show_settings_modal() {
        unsafe {
            let alert: *mut Object = msg_send![class!(NSAlert), new];
            let title = NSString::alloc(nil).init_str("Settings");
            let _: () = msg_send![alert, setMessageText: title];
            let ok = NSString::alloc(nil).init_str("OK");
            let _: *mut Object = msg_send![alert, addButtonWithTitle: ok];
            // Add a Cancel button with Escape key equivalent
            let cancel = NSString::alloc(nil).init_str("Cancel");
            let cancel_btn: *mut Object = msg_send![alert, addButtonWithTitle: cancel];
            let esc = NSString::alloc(nil).init_str("\u{1b}");
            let _: () = msg_send![cancel_btn, setKeyEquivalent: esc];

            // Accessory view with a tabbed interface: General and Shortcuts
            let acc_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(420.0, 180.0));
            let accessory: *mut Object = msg_send![class!(NSView), alloc];
            let accessory: *mut Object = msg_send![accessory, initWithFrame: acc_frame];

            // Create NSTabView
            let tab: *mut Object = msg_send![class!(NSTabView), alloc];
            let tab: *mut Object = msg_send![tab, initWithFrame: acc_frame];

            // General tab content
            let general_view: *mut Object = msg_send![class!(NSView), alloc];
            let general_view: *mut Object = msg_send![general_view, initWithFrame: NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(420.0, 150.0))];

            // Fit Now button
            let btn: *mut Object = msg_send![class!(NSButton), alloc];
            let btn: *mut Object = msg_send![btn, initWithFrame: NSRect::new(NSPoint::new(16.0, 92.0), NSSize::new(380.0, 28.0))];
            let t1 = NSString::alloc(nil).init_str("Fit Window to Image Now");
            let _: () = msg_send![btn, setTitle: t1];
            let _: () = msg_send![btn, setButtonType: 1_i64]; // momentary push
            let handler = get_handler_instance();
            let _: () = msg_send![btn, setTarget: handler];
            let _: () = msg_send![btn, setAction: sel!(rustFitNow:)];
            let _: () = msg_send![general_view, addSubview: btn];

            // Lock aspect ratio checkbox
            let cb2: *mut Object = msg_send![class!(NSButton), alloc];
            let cb2: *mut Object = msg_send![cb2, initWithFrame: NSRect::new(NSPoint::new(16.0, 56.0), NSSize::new(380.0, 24.0))];
            let t2 = NSString::alloc(nil).init_str("Lock aspect ratio on resize");
            let _: () = msg_send![cb2, setTitle: t2];
            let _: () = msg_send![cb2, setButtonType: 3_i64]; // switch
                                                              // Tag for later lookup when applying settings
            let _: () = msg_send![cb2, setTag: 42_i64];
            if let Ok(s) = SETTINGS.lock() {
                let aspect_state: i64 = if s.aspect_lock { 1 } else { 0 };
                let _: () = msg_send![cb2, setState: aspect_state];
            }
            let _: () = msg_send![general_view, addSubview: cb2];

            // Build "General" tab item
            let general_item: *mut Object = msg_send![class!(NSTabViewItem), alloc];
            let general_item: *mut Object = msg_send![general_item, initWithIdentifier: nil];
            let label1 = NSString::alloc(nil).init_str("General");
            let _: () = msg_send![general_item, setLabel: label1];
            let _: () = msg_send![general_item, setView: general_view];

            // Shortcuts tab content
            let sc_view: *mut Object = msg_send![class!(NSView), alloc];
            let sc_view: *mut Object = msg_send![sc_view, initWithFrame: NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(420.0, 150.0))];

            // Helper to add a label
            let make_label = |y: f64, text: &str| -> *mut Object {
                let tf: *mut Object = msg_send![class!(NSTextField), alloc];
                let tf: *mut Object = msg_send![tf, initWithFrame: NSRect::new(NSPoint::new(16.0, y), NSSize::new(380.0, 20.0))];
                let s = NSString::alloc(nil).init_str(text);
                let _: () = msg_send![tf, setStringValue: s];
                let _: () = msg_send![tf, setBezeled: 0_i32];
                let _: () = msg_send![tf, setDrawsBackground: 0_i32];
                let _: () = msg_send![tf, setEditable: 0_i32];
                let _: () = msg_send![tf, setSelectable: 0_i32];
                tf
            };

            // Add shortcut rows
            let row1 = make_label(110.0, "Cmd+,  — Open Settings");
            let row2 = make_label(88.0, "Cmd+O  — Open File…");
            let row3 = make_label(66.0, "Cmd+Y  — Quick Look");
            let _: () = msg_send![sc_view, addSubview: row1];
            let _: () = msg_send![sc_view, addSubview: row2];
            let _: () = msg_send![sc_view, addSubview: row3];

            // Build "Shortcuts" tab item
            let sc_item: *mut Object = msg_send![class!(NSTabViewItem), alloc];
            let sc_item: *mut Object = msg_send![sc_item, initWithIdentifier: nil];
            let label2 = NSString::alloc(nil).init_str("Shortcuts");
            let _: () = msg_send![sc_item, setLabel: label2];
            let _: () = msg_send![sc_item, setView: sc_view];

            // Add items to tab view
            let _: () = msg_send![tab, addTabViewItem: general_item];
            let _: () = msg_send![tab, addTabViewItem: sc_item];
            let _: () = msg_send![accessory, addSubview: tab];
            let _: () = msg_send![alert, setAccessoryView: accessory];

            let response: i64 = msg_send![alert, runModal];

            // Only apply settings if OK (first button) was pressed.
            if response == 1000 {
                // Read states back and apply
                let s2: i64 = msg_send![cb2, state];
                if let Ok(mut s) = SETTINGS.lock() {
                    s.aspect_lock = s2 != 0;
                }
                // Notify app to apply settings (e.g., re-layout effects)
                if let Some(proxy) = EVENT_PROXY.lock().ok().and_then(|g| g.as_ref().cloned()) {
                    let _ = proxy.send_event(UserEvent::ApplySettings);
                }
            }
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

    // Load persisted settings and last file (if any)
    let mut selected: Option<PathBuf> = None;
    if let Some(st) = load_persisted() {
        if let Ok(mut s) = SETTINGS.lock() {
            s.fit_window = st.fit_window;
            s.aspect_lock = st.aspect_lock;
        }
        // Restore window size first (logical points)
        if let (Some(w), Some(h)) = (st.window_w, st.window_h) {
            if w > 0.0 && h > 0.0 {
                window.set_inner_size(LogicalSize::new(w, h));
            }
        }
        if let Some(p) = st.last_file.and_then(|s| {
            let pb = PathBuf::from(s);
            if pb.exists() {
                Some(pb)
            } else {
                None
            }
        }) {
            // Restore last file
            let title = match p.file_name().and_then(|n| n.to_str()) {
                Some(name) => format!("Pinned: {}", name),
                None => "Pinned: <unnamed>".to_string(),
            };
            window.set_title(&title);
            selected = Some(p);
        }
    }
    // If no file restored, ask the user to select a file immediately on launch.
    if selected.is_none() {
        selected = open_file_dialog_and_set_title(&window);
    }
    #[cfg(target_os = "macos")]
    if let Some(ref p) = selected {
        if let Some((w, h)) = macos_image::set_image(&window, p) {
            let aspect = if h > 0.0 { w / h } else { 0.0 };
            if aspect > 0.0 {
                if let Ok(mut a) = IMAGE_ASPECT.lock() {
                    *a = Some(aspect);
                }
            }
        }
    }
    // Persist initial state after potential restoration/selection.
    save_persisted(selected.as_ref(), &window);

    // Basic event loop: keep running until the user closes the window.
    let mut mods: ModifiersState = ModifiersState::empty();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::ModifiersChanged(m) => {
                    mods = m;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(key),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    // Keyboard fallbacks for menu shortcuts on macOS
                    if mods.logo() {
                        match key {
                            VirtualKeyCode::Comma => {
                                #[cfg(target_os = "macos")]
                                {
                                    macos_settings::show_settings_modal();
                                }
                                // Apply settings and persist
                                save_persisted(selected.as_ref(), &window);
                            }
                            VirtualKeyCode::O => {
                                selected = open_file_dialog_and_set_title(&window);
                                #[cfg(target_os = "macos")]
                                if let Some(ref p) = selected {
                                    if let Some((w, h)) = macos_image::set_image(&window, p) {
                                        let aspect = if h > 0.0 { w / h } else { 0.0 };
                                        if aspect > 0.0 {
                                            if let Ok(mut a) = IMAGE_ASPECT.lock() {
                                                *a = Some(aspect);
                                            }
                                        }
                                        if let Ok(s) = SETTINGS.lock() {
                                            if s.fit_window {
                                                let (cw, ch) = macos_image::clamp_to_screen(w, h);
                                                window.set_inner_size(LogicalSize::new(cw, ch));
                                            }
                                        }
                                    }
                                }
                                save_persisted(selected.as_ref(), &window);
                            }
                            VirtualKeyCode::Y => {
                                if let Some(path) = selected.as_ref() {
                                    window.set_window_level(WindowLevel::Normal);
                                    quick_look(path.clone());
                                }
                            }
                            VirtualKeyCode::F => {
                                // Trigger Fit Now action
                                #[cfg(target_os = "macos")]
                                {
                                    if let Some((w, h)) = macos_image::last_image_size() {
                                        let (cw, ch) = macos_image::clamp_to_screen(w, h);
                                        window.set_inner_size(LogicalSize::new(cw, ch));
                                    }
                                }
                                save_persisted(selected.as_ref(), &window);
                            }
                            _ => {}
                        }
                    }
                }
                WindowEvent::Resized(new_size) => {
                    #[cfg(target_os = "macos")]
                    {
                        macos_image::layout_image_view(&window);
                    }
                    // Persist new window size
                    save_persisted(selected.as_ref(), &window);
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
                    {
                        macos_image::layout_image_view(&window);
                    }
                }
                _ => {}
            },
            Event::UserEvent(UserEvent::OpenFile) => {
                selected = open_file_dialog_and_set_title(&window);
                #[cfg(target_os = "macos")]
                if let Some(ref p) = selected {
                    if let Some((w, h)) = macos_image::set_image(&window, p) {
                        let aspect = if h > 0.0 { w / h } else { 0.0 };
                        if aspect > 0.0 {
                            if let Ok(mut a) = IMAGE_ASPECT.lock() {
                                *a = Some(aspect);
                            }
                        }
                    }
                }
                save_persisted(selected.as_ref(), &window);
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
                {
                    macos_settings::show_settings_modal();
                }
            }
            Event::UserEvent(UserEvent::ApplySettings) => {
                save_persisted(selected.as_ref(), &window);
            }
            Event::UserEvent(UserEvent::FitNow) => {
                #[cfg(target_os = "macos")]
                {
                    if let Some((w, h)) = macos_image::last_image_size() {
                        let (cw, ch) = macos_image::clamp_to_screen(w, h);
                        window.set_inner_size(LogicalSize::new(cw, ch));
                    }
                }
                save_persisted(selected.as_ref(), &window);
            }
            Event::UserEvent(UserEvent::RestoreTop) => {
                window.set_window_level(WindowLevel::AlwaysOnTop);
            }
            Event::LoopDestroyed => {
                // Drop selected path if any (no-op but explicit ownership end).
                let _ = &selected;
                save_persisted(selected.as_ref(), &window);
            }
            _ => {}
        }
    });
}

fn open_file_dialog_and_set_title(window: &winit::window::Window) -> Option<PathBuf> {
    // Use macOS native prompt via AppleScript to avoid extra GUI crates.
    let script = r#"POSIX path of (choose file with prompt "Select a file to pin on top")"#;
    match Command::new("osascript").arg("-e").arg(script).output() {
        Ok(out) if out.status.success() => {
            let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if path_str.is_empty() {
                return None;
            }
            let path = PathBuf::from(path_str);
            let title = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => format!("Pinned: {}", name),
                None => "Pinned: <unnamed>".to_string(),
            };
            window.set_title(&title);
            Some(path)
        }
        _ => None,
    }
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
