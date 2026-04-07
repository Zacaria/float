#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use base64::{engine::general_purpose, Engine};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
    sync::atomic::AtomicUsize,
    time::Duration,
};
use tauri::menu::{
    CheckMenuItem, CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder,
};
use tauri::{
    async_runtime,
    window::{Effect, EffectsBuilder},
    AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WindowEvent, Wry,
};
use tauri_plugin_dialog::DialogExt;
use tokio::time::sleep;

#[cfg(target_os = "macos")]
use objc2_app_kit::NSWindow;
#[cfg(target_os = "windows")]
use windows_sys::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{LWA_ALPHA, SetLayeredWindowAttributes},
};

const SETTINGS_WINDOW_LABEL: &str = "settings";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum WindowSizeUnits {
    Logical,
    Physical,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
struct PersistedState {
    aspect_lock: bool,
    click_through: bool,
    slideshow_enabled: bool,
    slideshow_interval_ms: u64,
    opacity_percent: u8,
    blur_enabled: bool,
    window_w: Option<f64>,
    window_h: Option<f64>,
    window_size_units: Option<WindowSizeUnits>,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            aspect_lock: false,
            click_through: false,
            slideshow_enabled: false,
            slideshow_interval_ms: 5000,
            opacity_percent: 100,
            blur_enabled: false,
            window_w: None,
            window_h: None,
            window_size_units: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct ActiveFilePayload {
    path: Option<String>,
    index: Option<usize>,
    total: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
struct SettingsPayload {
    aspect_lock: bool,
    click_through: bool,
    slideshow_enabled: bool,
    slideshow_interval_ms: u64,
    opacity_percent: u8,
    blur_enabled: bool,
    blur_supported: bool,
}

#[derive(Clone, Debug)]
struct SelectionState {
    files: Vec<String>,
    active: usize,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("no config dir available")]
    NoConfigDir,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("tauri: {0}")]
    Tauri(#[from] tauri::Error),
}

struct AppState {
    settings: Mutex<PersistedState>,
    aspect_ratio: Mutex<HashMap<String, f64>>, // per-window aspect ratio
    adjusting_resize: Mutex<HashSet<String>>,  // per-window resize guard
    aspect_toggle: Mutex<Option<CheckMenuItem<Wry>>>,
    click_through_toggle: Mutex<Option<CheckMenuItem<Wry>>>,
    slideshow_toggle: Mutex<Option<CheckMenuItem<Wry>>>,
    pending_save: Mutex<HashMap<String, async_runtime::JoinHandle<()>>>,
    selections: Mutex<HashMap<String, SelectionState>>, // per-window selections
    last_focused_window: Mutex<Option<String>>,         // label of last focused viewer window
    window_counter: AtomicUsize,
}

fn is_image_path(path: &str) -> bool {
    let ext = PathBuf::from(path)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase());
    matches!(
        ext.as_deref(),
        Some("png")
            | Some("jpg")
            | Some("jpeg")
            | Some("gif")
            | Some("webp")
            | Some("bmp")
            | Some("tif")
            | Some("tiff")
            | Some("heic")
    )
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: Mutex::new(PersistedState::default()),
            aspect_ratio: Mutex::new(HashMap::new()),
            adjusting_resize: Mutex::new(HashSet::new()),
            aspect_toggle: Mutex::new(None),
            click_through_toggle: Mutex::new(None),
            slideshow_toggle: Mutex::new(None),
            pending_save: Mutex::new(HashMap::new()),
            selections: Mutex::new(HashMap::new()),
            last_focused_window: Mutex::new(None),
            window_counter: AtomicUsize::new(0),
        }
    }
}

fn config_path(app: &AppHandle) -> Result<PathBuf, Error> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|_| Error::NoConfigDir)?;
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir.join("settings.json"))
}

fn load_state(app: &AppHandle) -> PersistedState {
    if let Ok(path) = config_path(app) {
        if path.exists() {
            if let Ok(bytes) = fs::read(path) {
                if let Ok(s) = serde_json::from_slice::<PersistedState>(&bytes) {
                    return s;
                }
            }
        }
    }
    PersistedState::default()
}

fn logical_outer_size(win: &WebviewWindow) -> Option<(f64, f64)> {
    if let (Ok(size), Ok(scale_factor)) = (win.outer_size(), win.scale_factor()) {
        let safe_scale = if scale_factor > 0.0 {
            scale_factor
        } else {
            1.0
        };
        return Some((
            (size.width as f64) / safe_scale,
            (size.height as f64) / safe_scale,
        ));
    }
    None
}

fn save_state(
    app: &AppHandle,
    win: Option<&WebviewWindow>,
    mut st: PersistedState,
) -> Result<(), Error> {
    if let Some(win) = win.filter(|win| is_viewer_window_label(win.label())) {
        if let Some((logical_w, logical_h)) = logical_outer_size(win) {
            st.window_w = Some(logical_w);
            st.window_h = Some(logical_h);
            st.window_size_units = Some(WindowSizeUnits::Logical);
        } else if let Ok(size) = win.outer_size() {
            st.window_w = Some(size.width as f64);
            st.window_h = Some(size.height as f64);
            st.window_size_units = Some(WindowSizeUnits::Physical);
        }
    }
    let path = config_path(app)?;
    fs::write(path, serde_json::to_vec_pretty(&st)?)?;
    Ok(())
}

fn schedule_size_save(app: AppHandle, label: String, win: WebviewWindow) {
    if let Some(state) = app.try_state::<AppState>() {
        let mut pending = state.pending_save.lock();
        if let Some(handle) = pending.remove(&label) {
            handle.abort();
        }
        let app_for_task = app.clone();
        let win_for_task = win.clone();
        let label_for_task = label.clone();
        let handle = async_runtime::spawn(async move {
            sleep(Duration::from_millis(500)).await;
            if let Some(state) = app_for_task.try_state::<AppState>() {
                let st = state.settings.lock().clone();
                let _ = save_state(&app_for_task, Some(&win_for_task), st);
            } else {
                let st = load_state(&app_for_task);
                let _ = save_state(&app_for_task, Some(&win_for_task), st);
            }
            if let Some(state) = app_for_task.try_state::<AppState>() {
                state.pending_save.lock().remove(&label_for_task);
            }
        });
        pending.insert(label, handle);
    }
}

fn blur_supported() -> bool {
    cfg!(target_os = "windows")
}

fn settings_payload(settings: &PersistedState) -> SettingsPayload {
    SettingsPayload {
        aspect_lock: settings.aspect_lock,
        click_through: settings.click_through,
        slideshow_enabled: settings.slideshow_enabled,
        slideshow_interval_ms: settings.slideshow_interval_ms,
        opacity_percent: settings.opacity_percent,
        blur_enabled: settings.blur_enabled,
        blur_supported: blur_supported(),
    }
}

fn emit_settings_changed(app: &AppHandle, settings: &PersistedState) {
    let payload = settings_payload(settings);
    for (_, window) in app.webview_windows() {
        let _ = window.emit("settings-changed", payload.clone());
    }
}

fn is_settings_window_label(label: &str) -> bool {
    label == SETTINGS_WINDOW_LABEL
}

fn is_viewer_window_label(label: &str) -> bool {
    !is_settings_window_label(label)
}

fn viewer_windows(app: &AppHandle) -> Vec<WebviewWindow> {
    app.webview_windows()
        .into_iter()
        .filter_map(|(label, window)| is_viewer_window_label(&label).then_some(window))
        .collect()
}

fn opacity_factor(settings: &PersistedState) -> f64 {
    (settings.opacity_percent.clamp(35, 100) as f64) / 100.0
}

#[cfg(target_os = "macos")]
fn apply_native_window_opacity(window: &WebviewWindow, settings: &PersistedState) {
    if let Ok(ns_window) = window.ns_window() {
        unsafe {
            let window: &NSWindow = &*ns_window.cast();
            window.setAlphaValue(opacity_factor(settings));
        }
    }
}

#[cfg(target_os = "windows")]
fn apply_native_window_opacity(window: &WebviewWindow, settings: &PersistedState) {
    if let Ok(hwnd) = window.hwnd() {
        unsafe {
            let alpha = (opacity_factor(settings) * 255.0).round() as u8;
            let _ = SetLayeredWindowAttributes(hwnd.0 as HWND, 0, alpha, LWA_ALPHA);
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn apply_native_window_opacity(_window: &WebviewWindow, _settings: &PersistedState) {}

fn apply_window_appearance(window: &WebviewWindow, settings: &PersistedState) {
    apply_native_window_opacity(window, settings);

    let result = if blur_supported() && settings.blur_enabled {
        window.set_effects(EffectsBuilder::new().effect(Effect::Blur).build())
    } else {
        window.set_effects(None::<tauri::utils::config::WindowEffectsConfig>)
    };

    if let Err(err) = result {
        eprintln!("window effects update failed: {err}");
    }
}

fn apply_window_appearance_to_all_windows(app: &AppHandle, settings: &PersistedState) {
    for window in viewer_windows(app) {
        apply_window_appearance(&window, settings);
    }
}

fn open_settings_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(SETTINGS_WINDOW_LABEL) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    let builder = tauri::WebviewWindowBuilder::new(
        app,
        SETTINGS_WINDOW_LABEL,
        WebviewUrl::App("settings.html".into()),
    )
    .title("Float Settings")
    .visible(true)
    .focused(true)
    .resizable(true)
    .decorations(true)
    .inner_size(540.0, 680.0)
    .always_on_top(false);

    match builder.build() {
        Ok(window) => {
            let _ = window.set_focus();
            if let Some(state) = app.try_state::<AppState>() {
                let payload = settings_payload(&state.settings.lock().clone());
                let _ = window.emit("settings-changed", payload);
            }
        }
        Err(err) => eprintln!("failed to open settings window: {err}"),
    }
}

fn create_viewer_window(app: &AppHandle) -> Result<WebviewWindow, Error> {
    let builder = tauri::WebviewWindowBuilder::new(
        app,
        next_window_label(app),
        WebviewUrl::App("index.html".into()),
    )
    .title("Float")
    .visible(true)
    .focused(true)
    .resizable(true)
    .decorations(false)
    .inner_size(400.0, 400.0);

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let builder = builder.transparent(true);
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos"
    )))]
    let builder = builder;

    let window = builder.build()?;
    let label = window.label().to_string();

    apply_initial_window_state(app, &window);
    wire_window_events(app, &window);
    if let Some(state) = app.try_state::<AppState>() {
        state.selections.lock().remove(&label);
        state.aspect_ratio.lock().remove(&label);
        state.adjusting_resize.lock().remove(&label);
        state
            .last_focused_window
            .lock()
            .replace(label);
    }
    let _ = window.set_focus();
    Ok(window)
}

fn spawn_empty_window(app: &AppHandle) -> Result<(), Error> {
    create_viewer_window(app).map(|_| ())
}

fn reset_cache(app: &AppHandle) -> Result<(), Error> {
    if let Some(state) = app.try_state::<AppState>() {
        // Cancel pending saves to avoid rewriting the file after deletion.
        for (_label, handle) in state.pending_save.lock().drain() {
            handle.abort();
        }
        *state.settings.lock() = PersistedState::default();
        state.aspect_ratio.lock().clear();
        state.adjusting_resize.lock().clear();
        state.selections.lock().clear();
        state.last_focused_window.lock().take();
        // Sync menu toggles to defaults
        if let Some(toggle) = state.aspect_toggle.lock().clone() {
            let _ = toggle.set_checked(false);
        }
        if let Some(toggle) = state.click_through_toggle.lock().clone() {
            let _ = toggle.set_checked(false);
        }
        if let Some(toggle) = state.slideshow_toggle.lock().clone() {
            let _ = toggle.set_checked(false);
        }
    }
    if let Ok(path) = config_path(app) {
        if path.exists() {
            fs::remove_file(path)?;
        }
    }
    for (_, window) in app.webview_windows() {
        let _ = window.close();
    }
    spawn_empty_window(app)?;
    Ok(())
}

fn focused_any_window(app: &AppHandle) -> Option<WebviewWindow> {
    let mut focused = None;
    for (_label, window) in app.webview_windows() {
        if let Ok(true) = window.is_focused() {
            focused = Some(window);
            break;
        }
    }
    focused.or_else(|| app.get_webview_window("main"))
}

fn focused_viewer_window(app: &AppHandle) -> Option<WebviewWindow> {
    if let Some(window) = focused_any_window(app).filter(|win| is_viewer_window_label(win.label())) {
        return Some(window);
    }

    if let Some(state) = app.try_state::<AppState>() {
        if let Some(label) = state.last_focused_window.lock().clone() {
            if let Some(window) = app.get_webview_window(&label) {
                return Some(window);
            }
        }
    }

    for (label, window) in app.webview_windows() {
        if is_viewer_window_label(&label) && matches!(window.is_focused(), Ok(true)) {
            return Some(window);
        }
    }

    app.get_webview_window("main")
}

fn active_file_for_window(app: &AppHandle, label: &str) -> Option<String> {
    if let Some(state) = app.try_state::<AppState>() {
        let selections = state.selections.lock();
        if let Some(sel) = selections.get(label) {
            return sel.files.get(sel.active).cloned();
        }
    }
    None
}

fn active_file_event_name(label: &str) -> String {
    format!("active-file-changed:{label}")
}

fn file_selected_event_name(label: &str) -> String {
    format!("file-selected:{label}")
}

fn emit_active_file(window: &WebviewWindow, payload: ActiveFilePayload) {
    let active_event = active_file_event_name(window.label());
    let file_selected_event = file_selected_event_name(window.label());
    let _ = window.emit(&active_event, payload.clone());
    let _ = window.emit(
        &file_selected_event,
        ActiveFilePayload {
            path: payload.path.clone(),
            index: None,
            total: None,
        },
    );
}

fn apply_active_file(
    app: &AppHandle,
    window: &WebviewWindow,
    selection: &SelectionState,
) -> Option<String> {
    let path_str = selection.files.get(selection.active)?.clone();
    if !is_image_path(&path_str) {
        return None;
    }
    let path = PathBuf::from(&path_str);
    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
        let _ = window.set_title(&format!("Float — {}", name));
    }

    // Cache aspect ratio per window
    if let Ok((w, h)) = image::image_dimensions(&path) {
        if h > 0 {
            if let Some(state) = app.try_state::<AppState>() {
                state
                    .aspect_ratio
                    .lock()
                    .insert(window.label().to_string(), w as f64 / h as f64);
            }
        }
    }

    // Persist viewer size without treating the active file as shared settings state.
    if let Some(state) = app.try_state::<AppState>() {
        let st = state.settings.lock().clone();
        let _ = save_state(app, Some(window), st);
    } else {
        let st = load_state(app);
        let _ = save_state(app, Some(window), st);
    }

    emit_active_file(
        window,
        ActiveFilePayload {
            path: Some(path_str.clone()),
            index: Some(selection.active),
            total: Some(selection.files.len()),
        },
    );

    Some(path_str)
}

fn apply_selection(app: &AppHandle, window: &WebviewWindow, files: Vec<String>) -> Option<String> {
    let files: Vec<String> = files.into_iter().filter(|p| is_image_path(p)).collect();
    if files.is_empty() {
        emit_active_file(
            window,
            ActiveFilePayload {
                path: None,
                index: None,
                total: Some(0),
            },
        );
        return None;
    }
    let selection = SelectionState { files, active: 0 };
    if let Some(state) = app.try_state::<AppState>() {
        state
            .selections
            .lock()
            .insert(window.label().to_string(), selection.clone());
    }
    apply_active_file(app, window, &selection)
}

fn wrapped_selection_index(current: usize, len: usize, delta: isize) -> Option<usize> {
    if len == 0 {
        return None;
    }

    let len = len as isize;
    let next = (current as isize + delta).rem_euclid(len);
    Some(next as usize)
}

fn navigate_selection(app: &AppHandle, window: &WebviewWindow, delta: isize) -> Option<String> {
    if let Some(state) = app.try_state::<AppState>() {
        let mut selections = state.selections.lock();
        if let Some(sel) = selections.get_mut(window.label()) {
            let len = sel.files.len();
            if let Some(next_index) = wrapped_selection_index(sel.active, len, delta) {
                if next_index != sel.active {
                    sel.active = next_index;
                    return apply_active_file(app, window, sel);
                }
                return None;
            }
            return None;
        }
    }
    None
}

fn apply_click_through_to_window(window: &WebviewWindow, enabled: bool) {
    #[allow(deprecated)]
    let _ = window.set_ignore_cursor_events(enabled);
}

fn apply_click_through_to_all_windows(app: &AppHandle, enabled: bool) {
    for win in viewer_windows(app) {
        apply_click_through_to_window(&win, enabled);
    }
}

fn apply_initial_window_state(app: &AppHandle, window: &WebviewWindow) {
    let _ = window.set_always_on_top(true);

    let st = load_state(app);
    if let (Some(w), Some(h)) = (st.window_w, st.window_h) {
        let logical_size = match st
            .window_size_units
            .clone()
            .unwrap_or(WindowSizeUnits::Physical)
        {
            WindowSizeUnits::Logical => Some((w, h)),
            WindowSizeUnits::Physical => {
                if let Ok(scale_factor) = window.scale_factor() {
                    let safe_scale = if scale_factor > 0.0 {
                        scale_factor
                    } else {
                        1.0
                    };
                    Some((w / safe_scale, h / safe_scale))
                } else {
                    None
                }
            }
        };
        if let Some((logical_w, logical_h)) = logical_size {
            let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                width: logical_w,
                height: logical_h,
            }));
        }
    }

    apply_click_through_to_window(window, st.click_through);
    apply_window_appearance(window, &st);
}

fn wire_window_events(app_handle: &AppHandle, window: &WebviewWindow) {
    let label = window.label().to_string();
    let app_for_event = app_handle.clone();
    window.on_window_event(move |e| match e {
        WindowEvent::Resized(size) => {
            if let Some(state) = app_for_event.try_state::<AppState>() {
                let mut adjusting = state.adjusting_resize.lock();
                if adjusting.contains(&label) {
                    return;
                }
                let st = state.settings.lock().clone();
                if st.aspect_lock {
                    if let Some(r) = state.aspect_ratio.lock().get(&label).copied() {
                        if r.is_finite() && r > 0.0 {
                            let new_w = size.width as f64;
                            let new_h = (new_w / r).round().max(1.0);
                            adjusting.insert(label.clone());
                            if let Some(win) = app_for_event.get_webview_window(&label) {
                                let _ = win.set_size(tauri::Size::Logical(tauri::LogicalSize {
                                    width: new_w,
                                    height: new_h,
                                }));
                            }
                            adjusting.remove(&label);
                        }
                    }
                }
                if let Some(win) = app_for_event.get_webview_window(&label) {
                    schedule_size_save(app_for_event.clone(), label.clone(), win);
                }
            }
        }
        WindowEvent::Focused(true) => {
            if is_viewer_window_label(&label) {
                if let Some(state) = app_for_event.try_state::<AppState>() {
                    *state.last_focused_window.lock() = Some(label.clone());
                    if let Some(win) = app_for_event.get_webview_window(&label) {
                        let st = state.settings.lock().clone();
                        let _ = save_state(&app_for_event, Some(&win), st);
                    }
                }
            }
        }
        WindowEvent::Destroyed => {
            if is_viewer_window_label(&label) {
                if let Some(state) = app_for_event.try_state::<AppState>() {
                    if let Some(handle) = state.pending_save.lock().remove(&label) {
                        handle.abort();
                    }
                    state.selections.lock().remove(&label);
                    state.aspect_ratio.lock().remove(&label);
                    state.adjusting_resize.lock().remove(&label);

                    let mut last_focused = state.last_focused_window.lock();
                    if last_focused.as_deref() == Some(label.as_str()) {
                        last_focused.take();
                    }
                }
            }
        }
        _ => {}
    });
}

fn viewer_window_for_label(app: &AppHandle, label: Option<&str>) -> Option<WebviewWindow> {
    label
        .and_then(|label| app.get_webview_window(label))
        .filter(|window| is_viewer_window_label(window.label()))
}

#[tauri::command]
async fn choose_file(app: AppHandle, window: WebviewWindow) -> Option<String> {
    pick_and_apply_selection(app, Some(window.label().to_string()))
}

#[tauri::command]
fn current_window_label(window: WebviewWindow) -> String {
    window.label().to_string()
}

#[tauri::command]
fn mark_active_viewer(app: AppHandle, window: WebviewWindow) {
    if !is_viewer_window_label(window.label()) {
        return;
    }
    if let Some(state) = app.try_state::<AppState>() {
        *state.last_focused_window.lock() = Some(window.label().to_string());
    }
}

#[tauri::command]
fn previous_file(app: AppHandle) -> Option<String> {
    if let Some(win) = focused_viewer_window(&app) {
        return navigate_selection(&app, &win, -1);
    }
    None
}

#[tauri::command]
fn next_file(app: AppHandle) -> Option<String> {
    if let Some(win) = focused_viewer_window(&app) {
        return navigate_selection(&app, &win, 1);
    }
    None
}

#[tauri::command]
fn load_image_data(path: String) -> Result<String, String> {
    if !is_image_path(&path) {
        return Err("unsupported file type".into());
    }
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err("file does not exist".into());
    }
    let bytes = fs::read(&path_buf).map_err(|e| format!("read error: {e}"))?;
    let mime = match path_buf
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "application/octet-stream",
    };
    let encoded = general_purpose::STANDARD.encode(bytes);
    Ok(format!("data:{mime};base64,{encoded}"))
}

#[tauri::command]
fn fit_now(app: AppHandle, window: WebviewWindow) -> Result<(), String> {
    let path = active_file_for_window(&app, window.label())
        .map(PathBuf::from);

    let path = match path {
        Some(p) => p,
        None => return Ok(()),
    };

    let img = image::image_dimensions(&path)
        .map_err(|e| format!("failed to read image dimensions: {e}"))?;
    let (img_w, img_h) = (img.0 as f64, img.1 as f64);
    if img_w <= 0.0 || img_h <= 0.0 {
        return Ok(());
    }
    let aspect = img_w / img_h;

    // Anchor on the current larger window dimension and adjust the other down to match aspect.
    // Convert to logical units first so high-DPI windows don't double in size when resizing.
    if let (Ok(size), Ok(scale_factor)) = (window.outer_size(), window.scale_factor()) {
        let cur_w = (size.width as f64) / scale_factor;
        let cur_h = (size.height as f64) / scale_factor;
        let min_dim = 50.0_f64;
        let (mut new_w, mut new_h) = if cur_w >= cur_h {
            let mut target_w = cur_w;
            let mut target_h = target_w / aspect;
            if target_h > cur_h && target_h > 0.0 {
                let scale = cur_h / target_h;
                target_w *= scale;
                target_h = cur_h;
            }
            (target_w, target_h)
        } else {
            let mut target_h = cur_h;
            let mut target_w = target_h * aspect;
            if target_w > cur_w && target_w > 0.0 {
                let scale = cur_w / target_w;
                target_h *= scale;
                target_w = cur_w;
            }
            (target_w, target_h)
        };

        new_w = new_w.max(min_dim);
        new_h = new_h.max(min_dim);

        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: new_w,
            height: new_h,
        }));
        if let Some(state) = app.try_state::<AppState>() {
            state
                .aspect_ratio
                .lock()
                .insert(window.label().to_string(), aspect);
        }
    }
    Ok(())
}

#[tauri::command]
fn get_settings(app: AppHandle) -> SettingsPayload {
    if let Some(state) = app.try_state::<AppState>() {
        settings_payload(&state.settings.lock().clone())
    } else {
        settings_payload(&load_state(&app))
    }
}

#[derive(Deserialize)]
struct SettingsUpdate {
    aspect_lock: Option<bool>,
    click_through: Option<bool>,
    slideshow_enabled: Option<bool>,
    slideshow_interval_ms: Option<u64>,
    opacity_percent: Option<u8>,
    blur_enabled: Option<bool>,
}

#[tauri::command]
fn set_settings(app: AppHandle, update: SettingsUpdate) -> Result<SettingsPayload, String> {
    let maybe_viewer = focused_viewer_window(&app);
    let mut st = if let Some(state) = app.try_state::<AppState>() {
        state.settings.lock().clone()
    } else {
        load_state(&app)
    };
    if let Some(v) = update.aspect_lock {
        st.aspect_lock = v;
        if let Some(state) = app.try_state::<AppState>() {
            if let Some(toggle) = state.aspect_toggle.lock().clone() {
                let _ = toggle.set_checked(v);
            }
        }
    }
    if let Some(v) = update.click_through {
        st.click_through = v;
        apply_click_through_to_all_windows(&app, v);
        if let Some(state) = app.try_state::<AppState>() {
            if let Some(toggle) = state.click_through_toggle.lock().clone() {
                let _ = toggle.set_checked(v);
            }
        }
    }
    if let Some(v) = update.slideshow_enabled {
        st.slideshow_enabled = v;
        if let Some(state) = app.try_state::<AppState>() {
            if let Some(toggle) = state.slideshow_toggle.lock().clone() {
                let _ = toggle.set_checked(v);
            }
        }
    }
    if let Some(v) = update.slideshow_interval_ms {
        st.slideshow_interval_ms = v.clamp(1000, 60000);
    }
    if let Some(v) = update.opacity_percent {
        st.opacity_percent = v.clamp(35, 100);
        apply_window_appearance_to_all_windows(&app, &st);
    }
    if let Some(v) = update.blur_enabled {
        st.blur_enabled = v;
        apply_window_appearance_to_all_windows(&app, &st);
    }
    save_state(&app, maybe_viewer.as_ref(), st.clone()).map_err(|e| e.to_string())?;
    if let Some(state) = app.try_state::<AppState>() {
        *state.settings.lock() = st.clone();
    }
    emit_settings_changed(&app, &st);
    Ok(settings_payload(&st))
}

fn pick_files(app: &AppHandle, parent: Option<&WebviewWindow>) -> Vec<String> {
    let make_picker = || {
        if let Some(win) = parent {
            app.dialog().file().set_parent(win)
        } else {
            app.dialog().file()
        }
    };

    let mut paths = Vec::new();
    if let Some(files) = make_picker().blocking_pick_files() {
        for file in files {
            if let Ok(path) = file.into_path() {
                let path_str = path.to_string_lossy().to_string();
                if is_image_path(&path_str) {
                    paths.push(path_str);
                }
            }
        }
    }

    if paths.is_empty() {
        if let Some(file) = make_picker().blocking_pick_file() {
            if let Ok(path) = file.into_path() {
                let path_str = path.to_string_lossy().to_string();
                if is_image_path(&path_str) {
                    return vec![path_str];
                }
            }
        }
    }

    paths
}

fn pick_and_apply_selection(app: AppHandle, target_label: Option<String>) -> Option<String> {
    // For automation, allow bypassing the native dialog with a predefined path.
    if let Ok(test_path) =
        std::env::var("FLOAT_TEST_PATH").or_else(|_| std::env::var("AOT_TEST_PATH"))
    {
        if !test_path.is_empty() {
            if let Some(win) = viewer_window_for_label(&app, target_label.as_deref())
                .or_else(|| focused_viewer_window(&app))
            {
                return apply_selection(&app, &win, vec![test_path]);
            }
        } else {
            return None;
        }
    }

    let focus =
        viewer_window_for_label(&app, target_label.as_deref()).or_else(|| focused_viewer_window(&app));
    let parent = focus.as_ref();
    let files = pick_files(&app, parent);
    if files.is_empty() {
        return None;
    }

    if let Some(win) = focus.or_else(|| app.get_webview_window("main")) {
        apply_selection(&app, &win, files)
    } else {
        None
    }
}

fn next_window_label(app: &AppHandle) -> String {
    let existing: std::collections::HashSet<String> =
        app.webview_windows().keys().cloned().collect();
    if !existing.contains("main") {
        return "main".to_string();
    }
    let mut idx = 1;
    loop {
        let candidate = format!("window-{idx}");
        if !existing.contains(&candidate) {
            return candidate;
        }
        idx += 1;
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Build native menu with platform shortcuts and toggles.
            let file_menu = SubmenuBuilder::new(&app_handle, "File")
                .item(
                    &MenuItemBuilder::with_id("new_window", "New Window")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Cmd+T"
                        } else {
                            "Ctrl+T"
                        })
                        .build(&app_handle)?,
                )
                .item(
                    &MenuItemBuilder::with_id("open", "Open…")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Cmd+O"
                        } else {
                            "Ctrl+O"
                        })
                        .build(&app_handle)?,
                )
                .item(
                    &MenuItemBuilder::with_id("close_window", "Close Window")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Cmd+W"
                        } else {
                            "Ctrl+W"
                        })
                        .build(&app_handle)?,
                )
                .item(
                    &MenuItemBuilder::with_id("open_settings", "Settings…")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Cmd+,"
                        } else {
                            "Ctrl+,"
                        })
                        .build(&app_handle)?,
                )
                .item(
                    &MenuItemBuilder::with_id("reset_cache", "Reset Cache")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Cmd+Shift+Backspace"
                        } else {
                            "Ctrl+Shift+Backspace"
                        })
                        .build(&app_handle)?,
                )
                .item(
                    &MenuItemBuilder::with_id("quit", "Quit")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Cmd+Q"
                        } else {
                            "Ctrl+Q"
                        })
                        .build(&app_handle)?,
                )
                .build()?;

            let initial_settings = load_state(&app_handle);
            let aspect_toggle =
                CheckMenuItemBuilder::with_id("aspect_lock_toggle", "Lock aspect ratio on resize")
                    .checked(initial_settings.aspect_lock)
                    .build(&app_handle)?;
            let click_through_toggle =
                CheckMenuItemBuilder::with_id("click_through_toggle", "Click-through overlay")
                    .checked(initial_settings.click_through)
                    .accelerator(if cfg!(target_os = "macos") {
                        "Cmd+Shift+X"
                    } else {
                        "Ctrl+Shift+X"
                    })
                    .build(&app_handle)?;
            let slideshow_toggle =
                CheckMenuItemBuilder::with_id("slideshow_toggle", "Slideshow mode")
                    .checked(initial_settings.slideshow_enabled)
                    .accelerator(if cfg!(target_os = "macos") {
                        "Cmd+Shift+S"
                    } else {
                        "Ctrl+Shift+S"
                    })
                    .build(&app_handle)?;
            let view_menu = SubmenuBuilder::new(&app_handle, "View")
                .item(
                    &MenuItemBuilder::with_id("fit_now", "Fit to Image Now")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Cmd+F"
                        } else {
                            "Ctrl+F"
                        })
                        .build(&app_handle)?,
                )
                .item(
                    &MenuItemBuilder::with_id("previous_file", "Previous File")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Alt+Cmd+["
                        } else {
                            "Ctrl+["
                        })
                        .build(&app_handle)?,
                )
                .item(
                    &MenuItemBuilder::with_id("next_file", "Next File")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Alt+Cmd+]"
                        } else {
                            "Ctrl+]"
                        })
                        .build(&app_handle)?,
                )
                .item(&aspect_toggle)
                .item(&click_through_toggle)
                .item(&slideshow_toggle);
            let app_menu = MenuBuilder::new(&app_handle)
                .item(&file_menu)
                .item(&view_menu.build()?)
                .build()?;
            app.set_menu(app_menu)?;
            if let Some(state) = app_handle.try_state::<AppState>() {
                *state.aspect_toggle.lock() = Some(aspect_toggle.clone());
                *state.click_through_toggle.lock() = Some(click_through_toggle.clone());
                *state.slideshow_toggle.lock() = Some(slideshow_toggle.clone());
            }

            if let Some(state) = app_handle.try_state::<AppState>() {
                *state.settings.lock() = load_state(&app_handle);
                state
                    .window_counter
                    .store(1, std::sync::atomic::Ordering::SeqCst);
            }

            let win = app_handle
                .get_webview_window("main")
                .expect("main window exists");
            apply_initial_window_state(&app_handle, &win);
            wire_window_events(&app_handle, &win);

            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => {
                let handle = app.clone();
                let target_label = focused_viewer_window(app).map(|window| window.label().to_string());
                async_runtime::spawn(async move {
                    let _ = pick_and_apply_selection(handle, target_label);
                });
            }
            "new_window" => {
                if let Err(err) = spawn_empty_window(app) {
                    eprintln!("new window failed: {err}");
                }
            }
            "close_window" => {
                if let Some(win) = focused_any_window(app) {
                    let _ = win.close();
                }
            }
            "open_settings" => open_settings_window(app),
            "reset_cache" => {
                if let Err(err) = reset_cache(app) {
                    eprintln!("reset cache failed: {err}");
                }
            }
            "quit" => {
                if let Some(state) = app.try_state::<AppState>() {
                    if let Some(win) = state
                        .last_focused_window
                        .lock()
                        .clone()
                        .and_then(|label| app.get_webview_window(&label))
                    {
                        let st = state.settings.lock().clone();
                        let _ = save_state(app, Some(&win), st);
                    }
                }
                app.exit(0);
            }
            "fit_now" => {
                if let Some(win) = focused_viewer_window(app) {
                    let _ = fit_now(app.clone(), win);
                }
            }
            "previous_file" => {
                if let Some(win) = focused_viewer_window(app) {
                    let _ = navigate_selection(app, &win, -1);
                }
            }
            "next_file" => {
                if let Some(win) = focused_viewer_window(app) {
                    let _ = navigate_selection(app, &win, 1);
                }
            }
            "aspect_lock_toggle" => {
                if let Some(state) = app.try_state::<AppState>() {
                    let mut s = state.settings.lock().clone();
                    let new_state = if let Some(toggle) = state.aspect_toggle.lock().clone() {
                        if let Ok(current) = toggle.is_checked() {
                            current
                        } else {
                            !s.aspect_lock
                        }
                    } else {
                        !s.aspect_lock
                    };
                    s.aspect_lock = new_state;
                    let maybe_viewer = focused_viewer_window(app);
                    let _ = save_state(app, maybe_viewer.as_ref(), s.clone());
                    *state.settings.lock() = s;
                    emit_settings_changed(app, &state.settings.lock().clone());
                }
            }
            "click_through_toggle" => {
                if let Some(state) = app.try_state::<AppState>() {
                    let mut s = state.settings.lock().clone();
                    let new_state = if let Some(toggle) = state.click_through_toggle.lock().clone()
                    {
                        if let Ok(current) = toggle.is_checked() {
                            current
                        } else {
                            !s.click_through
                        }
                    } else {
                        !s.click_through
                    };
                    s.click_through = new_state;
                    apply_click_through_to_all_windows(app, new_state);
                    let maybe_viewer = focused_viewer_window(app);
                    let _ = save_state(app, maybe_viewer.as_ref(), s.clone());
                    *state.settings.lock() = s;
                    emit_settings_changed(app, &state.settings.lock().clone());
                }
            }
            "slideshow_toggle" => {
                if let Some(state) = app.try_state::<AppState>() {
                    let mut s = state.settings.lock().clone();
                    let new_state = if let Some(toggle) = state.slideshow_toggle.lock().clone() {
                        if let Ok(current) = toggle.is_checked() {
                            current
                        } else {
                            !s.slideshow_enabled
                        }
                    } else {
                        !s.slideshow_enabled
                    };
                    s.slideshow_enabled = new_state;
                    let maybe_viewer = focused_viewer_window(app);
                    let _ = save_state(app, maybe_viewer.as_ref(), s.clone());
                    *state.settings.lock() = s;
                    emit_settings_changed(app, &state.settings.lock().clone());
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            choose_file,
            current_window_label,
            mark_active_viewer,
            fit_now,
            get_settings,
            set_settings,
            load_image_data,
            previous_file,
            next_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::wrapped_selection_index;

    #[test]
    fn wrapped_selection_moves_forward_and_wraps() {
        assert_eq!(wrapped_selection_index(0, 3, 1), Some(1));
        assert_eq!(wrapped_selection_index(2, 3, 1), Some(0));
    }

    #[test]
    fn wrapped_selection_moves_backward_and_wraps() {
        assert_eq!(wrapped_selection_index(2, 3, -1), Some(1));
        assert_eq!(wrapped_selection_index(0, 3, -1), Some(2));
    }

    #[test]
    fn wrapped_selection_handles_empty_lists() {
        assert_eq!(wrapped_selection_index(0, 0, 1), None);
    }
}
