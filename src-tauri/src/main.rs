#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use base64::{engine::general_purpose, Engine};
use directories::{BaseDirs, ProjectDirs};
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
    async_runtime, AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WindowEvent, Wry,
};
use tauri_plugin_dialog::DialogExt;
use tokio::time::sleep;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum WindowSizeUnits {
    Logical,
    Physical,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct PersistedState {
    last_file: Option<String>,
    aspect_lock: bool,
    window_w: Option<f64>,
    window_h: Option<f64>,
    window_size_units: Option<WindowSizeUnits>,
}

#[derive(Clone, Debug, Serialize)]
struct ActiveFilePayload {
    path: Option<String>,
    index: Option<usize>,
    total: Option<usize>,
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
    pending_save: Mutex<HashMap<String, async_runtime::JoinHandle<()>>>,
    selections: Mutex<HashMap<String, SelectionState>>, // per-window selections
    last_focused_window: Mutex<Option<String>>,         // label of last focused window
    window_counter: AtomicUsize,
}

const LEGACY_APP_NAME: &str = "Always On Top";
const LEGACY_IDENTIFIER: &str = "com.example.always-on-top";

fn legacy_settings_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(proj) = ProjectDirs::from("com", "example", LEGACY_APP_NAME) {
        candidates.push(proj.config_dir().to_path_buf().join("settings.json"));
    }
    if let Some(base) = BaseDirs::new() {
        candidates.push(
            base.config_dir()
                .join(LEGACY_IDENTIFIER)
                .join("settings.json"),
        );
        candidates.push(
            base.config_dir()
                .join(LEGACY_APP_NAME)
                .join("settings.json"),
        );
    }
    candidates
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
    let dest = dir.join("settings.json");
    if !dest.exists() {
        for candidate in legacy_settings_candidates() {
            if candidate.exists() {
                let _ = fs::copy(&candidate, &dest);
                break;
            }
        }
    }
    Ok(dest)
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
        let safe_scale = if scale_factor > 0.0 { scale_factor } else { 1.0 };
        return Some((
            (size.width as f64) / safe_scale,
            (size.height as f64) / safe_scale,
        ));
    }
    None
}

fn save_state(app: &AppHandle, win: &WebviewWindow, mut st: PersistedState) -> Result<(), Error> {
    if let Some((logical_w, logical_h)) = logical_outer_size(win) {
        st.window_w = Some(logical_w);
        st.window_h = Some(logical_h);
        st.window_size_units = Some(WindowSizeUnits::Logical);
    } else if let Ok(size) = win.outer_size() {
        st.window_w = Some(size.width as f64);
        st.window_h = Some(size.height as f64);
        st.window_size_units = Some(WindowSizeUnits::Physical);
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
                let _ = save_state(&app_for_task, &win_for_task, st);
            } else {
                let st = load_state(&app_for_task);
                let _ = save_state(&app_for_task, &win_for_task, st);
            }
            if let Some(state) = app_for_task.try_state::<AppState>() {
                state.pending_save.lock().remove(&label_for_task);
            }
        });
        pending.insert(label, handle);
    }
}

fn spawn_empty_window(app: &AppHandle) -> Result<(), Error> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        next_window_label(app),
        WebviewUrl::App("index.html".into()),
    )
    .title("Float")
    .visible(true)
    .resizable(true)
    .decorations(false)
    .inner_size(400.0, 400.0)
    .build()?;

    apply_initial_window_state(app, &window, false);
    wire_window_events(app, &window);
    if let Some(state) = app.try_state::<AppState>() {
        state
            .last_focused_window
            .lock()
            .replace(window.label().to_string());
    }
    Ok(())
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
        // Sync menu toggle to defaults
        if let Some(toggle) = state.aspect_toggle.lock().clone() {
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

fn focused_window(app: &AppHandle) -> Option<WebviewWindow> {
    let mut focused: Option<WebviewWindow> = None;
    for (_label, window) in app.webview_windows() {
        if let Ok(true) = window.is_focused() {
            focused = Some(window);
            break;
        }
    }
    focused.or_else(|| app.get_webview_window("main"))
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

fn emit_active_file(window: &WebviewWindow, payload: ActiveFilePayload) {
    let _ = window.emit("active-file-changed", payload.clone());
    // Backward compatibility with the previous event name
    let _ = window.emit(
        "file-selected",
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

    // Persist active file and window size
    if let Some(state) = app.try_state::<AppState>() {
        let mut st = state.settings.lock().clone();
        st.last_file = Some(path_str.clone());
        let _ = save_state(app, window, st.clone());
        *state.settings.lock() = st;
    } else {
        let mut st = load_state(app);
        st.last_file = Some(path_str.clone());
        let _ = save_state(app, window, st.clone());
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

fn navigate_selection(app: &AppHandle, window: &WebviewWindow, delta: isize) -> Option<String> {
    if let Some(state) = app.try_state::<AppState>() {
        let mut selections = state.selections.lock();
        if let Some(sel) = selections.get_mut(window.label()) {
            let len = sel.files.len();
            if len == 0 {
                return None;
            }
            let current = sel.active as isize;
            let next = current.saturating_add(delta);
            let bounded = next.clamp(0, (len as isize) - 1) as usize;
            if bounded != sel.active {
                sel.active = bounded;
                return apply_active_file(app, window, sel);
            }
        }
    }
    None
}

fn apply_initial_window_state(app: &AppHandle, window: &WebviewWindow, load_last_file: bool) {
    let _ = window.set_always_on_top(true);

    let st = load_state(app);
    if let (Some(w), Some(h)) = (st.window_w, st.window_h) {
        let logical_size = match st.window_size_units.unwrap_or(WindowSizeUnits::Physical) {
            WindowSizeUnits::Logical => Some((w, h)),
            WindowSizeUnits::Physical => {
                if let Ok(scale_factor) = window.scale_factor() {
                    let safe_scale = if scale_factor > 0.0 { scale_factor } else { 1.0 };
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

    if load_last_file {
        if let Some(p) = st.last_file.clone() {
            if is_image_path(&p) && PathBuf::from(&p).exists() {
                let _ = apply_selection(app, window, vec![p]);
            }
        }
    }
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
            if let Some(state) = app_for_event.try_state::<AppState>() {
                *state.last_focused_window.lock() = Some(label.clone());
            }
            if let Some(win) = app_for_event.get_webview_window(&label) {
                if let Some(path) = active_file_for_window(&app_for_event, &label) {
                    if let Some(state) = app_for_event.try_state::<AppState>() {
                        let mut st = state.settings.lock().clone();
                        st.last_file = Some(path);
                        let _ = save_state(&app_for_event, &win, st.clone());
                        *state.settings.lock() = st;
                    }
                }
            }
        }
        _ => {}
    });
}

#[tauri::command]
async fn choose_file(app: AppHandle) -> Option<String> {
    pick_and_apply_selection(app, SelectionTarget::CurrentWindow)
}

#[tauri::command]
fn previous_file(app: AppHandle) -> Option<String> {
    if let Some(win) = focused_window(&app) {
        return navigate_selection(&app, &win, -1);
    }
    None
}

#[tauri::command]
fn next_file(app: AppHandle) -> Option<String> {
    if let Some(win) = focused_window(&app) {
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
        .map(PathBuf::from)
        .or_else(|| {
            let st = if let Some(state) = app.try_state::<AppState>() {
                state.settings.lock().clone()
            } else {
                load_state(&app)
            };
            st.last_file.map(PathBuf::from)
        });

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
fn get_settings(app: AppHandle) -> PersistedState {
    if let Some(state) = app.try_state::<AppState>() {
        state.settings.lock().clone()
    } else {
        load_state(&app)
    }
}

#[derive(Deserialize)]
struct SettingsUpdate {
    aspect_lock: Option<bool>,
}

#[tauri::command]
fn set_settings(app: AppHandle, update: SettingsUpdate) -> Result<PersistedState, String> {
    let win = focused_window(&app).ok_or("missing window")?;
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
    save_state(&app, &win, st.clone()).map_err(|e| e.to_string())?;
    if let Some(state) = app.try_state::<AppState>() {
        *state.settings.lock() = st.clone();
    }
    Ok(st)
}

enum SelectionTarget {
    CurrentWindow,
    NewWindow,
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

fn pick_and_apply_selection(app: AppHandle, target: SelectionTarget) -> Option<String> {
    // For automation, allow bypassing the native dialog with a predefined path.
    if let Ok(test_path) =
        std::env::var("FLOAT_TEST_PATH").or_else(|_| std::env::var("AOT_TEST_PATH"))
    {
        if !test_path.is_empty() {
            match target {
                SelectionTarget::CurrentWindow => {
                    if let Some(win) = focused_window(&app) {
                        return apply_selection(&app, &win, vec![test_path]);
                    }
                }
                SelectionTarget::NewWindow => {
                    return spawn_new_window_with_files(&app, vec![test_path]);
                }
            }
        } else {
            return None;
        }
    }

    let focus = focused_window(&app);
    let parent = focus.as_ref();
    let files = pick_files(&app, parent);
    if files.is_empty() {
        return None;
    }

    match target {
        SelectionTarget::CurrentWindow => {
            if let Some(win) = focus.or_else(|| app.get_webview_window("main")) {
                apply_selection(&app, &win, files)
            } else {
                None
            }
        }
        SelectionTarget::NewWindow => spawn_new_window_with_files(&app, files),
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

fn spawn_new_window_with_files(app: &AppHandle, files: Vec<String>) -> Option<String> {
    if files.is_empty() {
        return None;
    }
    let label = next_window_label(app);
    let window =
        tauri::WebviewWindowBuilder::new(app, &label, WebviewUrl::App("index.html".into()))
            .title("Float")
            .visible(true)
            .resizable(true)
            .decorations(false)
            .inner_size(400.0, 400.0)
            .build()
            .ok()?;

    apply_initial_window_state(app, &window, false);
    wire_window_events(app, &window);
    if let Some(state) = app.try_state::<AppState>() {
        state
            .last_focused_window
            .lock()
            .replace(window.label().to_string());
    }
    apply_selection(app, &window, files)
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
                    &MenuItemBuilder::with_id("new_window", "New Window…")
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

            let aspect_toggle =
                CheckMenuItemBuilder::with_id("aspect_lock_toggle", "Lock aspect ratio on resize")
                    .checked(load_state(&app_handle).aspect_lock)
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
                .item(&aspect_toggle);
            let app_menu = MenuBuilder::new(&app_handle)
                .item(&file_menu)
                .item(&view_menu.build()?)
                .build()?;
            app.set_menu(app_menu)?;
            if let Some(state) = app_handle.try_state::<AppState>() {
                *state.aspect_toggle.lock() = Some(aspect_toggle.clone());
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
            apply_initial_window_state(&app_handle, &win, true);
            wire_window_events(&app_handle, &win);

            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => {
                let handle = app.clone();
                async_runtime::spawn(async move {
                    let _ = choose_file(handle).await;
                });
            }
            "new_window" => {
                let handle = app.clone();
                async_runtime::spawn(async move {
                    let _ = pick_and_apply_selection(handle, SelectionTarget::NewWindow);
                });
            }
            "close_window" => {
                if let Some(win) = focused_window(app) {
                    let _ = win.close();
                }
            }
            "reset_cache" => {
                if let Err(err) = reset_cache(app) {
                    eprintln!("reset cache failed: {err}");
                }
            }
            "quit" => {
                if let Some(state) = app.try_state::<AppState>() {
                    if let Some(label) = state.last_focused_window.lock().clone() {
                        if let Some(win) = app.get_webview_window(&label) {
                            if let Some(path) = active_file_for_window(app, &label) {
                                let mut st = state.settings.lock().clone();
                                st.last_file = Some(path);
                                let _ = save_state(app, &win, st.clone());
                                *state.settings.lock() = st;
                            }
                        }
                    }
                }
                app.exit(0);
            }
            "fit_now" => {
                if let Some(win) = focused_window(app) {
                    let _ = fit_now(app.clone(), win);
                }
            }
            "previous_file" => {
                if let Some(win) = focused_window(app) {
                    let _ = navigate_selection(app, &win, -1);
                }
            }
            "next_file" => {
                if let Some(win) = focused_window(app) {
                    let _ = navigate_selection(app, &win, 1);
                }
            }
            "aspect_lock_toggle" => {
                if let Some(state) = app.try_state::<AppState>() {
                    let mut s = state.settings.lock().clone();
                    let new_state = if let Some(toggle) = state.aspect_toggle.lock().clone() {
                        if let Ok(current) = toggle.is_checked() {
                            let desired = !current;
                            let _ = toggle.set_checked(desired);
                            desired
                        } else {
                            !s.aspect_lock
                        }
                    } else {
                        !s.aspect_lock
                    };
                    s.aspect_lock = new_state;
                    if let Some(win) = focused_window(app) {
                        let _ = save_state(app, &win, s.clone());
                    }
                    *state.settings.lock() = s;
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            choose_file,
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
