#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use base64::{engine::general_purpose, Engine};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use tauri::menu::{
    CheckMenuItem, CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder,
};
use tauri::{async_runtime, AppHandle, Emitter, Manager, WebviewWindow, WindowEvent, Wry};
use tauri_plugin_dialog::DialogExt;
use tokio::time::sleep;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct PersistedState {
    last_file: Option<String>,
    fit_window: bool,
    aspect_lock: bool,
    window_w: Option<f64>,
    window_h: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
struct FileSelectedPayload {
    path: Option<String>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("no config dir available")]
    NoConfigDir,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Default)]
struct AppState {
    // Cached settings in memory
    settings: Mutex<PersistedState>,
    // Current aspect ratio (w / h) based on last selected image
    aspect_ratio: Mutex<Option<f64>>,
    // Guard to avoid resize recursion when enforcing aspect lock
    adjusting_resize: AtomicBool,
    // Menu toggle handles for syncing check states
    fit_toggle: Mutex<Option<CheckMenuItem<Wry>>>,
    aspect_toggle: Mutex<Option<CheckMenuItem<Wry>>>,
    // Debounced save handle for window size persistence
    pending_save: Mutex<Option<async_runtime::JoinHandle<()>>>,
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
        } else {
            let mut s = PersistedState::default();
            s.fit_window = true; // default to auto-fit when unset
            return s;
        }
    }
    PersistedState::default()
}

fn save_state(app: &AppHandle, win: &WebviewWindow, mut st: PersistedState) -> Result<(), Error> {
    if let Ok(size) = win.outer_size() {
        st.window_w = Some(size.width as f64);
        st.window_h = Some(size.height as f64);
    }
    let path = config_path(app)?;
    fs::write(path, serde_json::to_vec_pretty(&st)?)?;
    Ok(())
}

#[tauri::command]
async fn choose_file(app: AppHandle) -> Option<String> {
    // For automation, allow bypassing the native dialog with a predefined path.
    if let Ok(test_path) = std::env::var("AOT_TEST_PATH") {
        dbg!("Using test path: {}", &test_path);
        if test_path.is_empty() {
            return None;
        }
        return handle_selected_path(app, test_path);
    }

    // Use Tauri dialog plugin for native file picking; prefer parenting to the main window when present.
    let picker = if let Some(win) = app.get_webview_window("main") {
        app.dialog().file().set_parent(&win)
    } else {
        app.dialog().file()
    };

    let picked = picker.blocking_pick_file();
    dbg!("Picked file: {picked:?}");

    if let Some(file_path) = picked {
        if let Ok(path) = file_path.into_path() {
            let path_str = path.to_string_lossy().to_string();
            dbg!("Selected path: {}", &path_str);
            handle_selected_path(app, path_str)
        } else {
            None
        }
    } else {
        None
    }
}

fn handle_selected_path(app: AppHandle, path_str: String) -> Option<String> {
    let path = PathBuf::from(&path_str);
    // Update title
    if let Some(win) = app.get_webview_window("main") {
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            let _ = win.set_title(&format!("Always On Top — {}", name));
        }
    }
    // Cache aspect ratio
    if let Ok((w, h)) = image::image_dimensions(&path) {
        if h > 0 {
            if let Some(state) = app.try_state::<AppState>() {
                *state.aspect_ratio.lock() = Some(w as f64 / h as f64);
            }
        }
    }
    // Persist settings and auto-fit if enabled
    if let Some(win) = app.get_webview_window("main") {
        let fit_enabled;
        if let Some(state) = app.try_state::<AppState>() {
            let mut s = state.settings.lock().clone();
            fit_enabled = s.fit_window;
            s.last_file = Some(path_str.clone());
            let _ = save_state(&app, &win, s.clone());
            *state.settings.lock() = s;
        } else {
            let mut st = load_state(&app);
            fit_enabled = st.fit_window;
            st.last_file = Some(path_str.clone());
            let _ = save_state(&app, &win, st);
        }
        if fit_enabled {
            let _ = fit_now(app.clone(), win.clone());
        }
    }
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.emit(
            "file-selected",
            FileSelectedPayload {
                path: Some(path_str.clone()),
            },
        );
        let _ = win.eval("window.location.reload()");
    }
    Some(path_str)
}

#[tauri::command]
fn load_image_data(path: String) -> Result<String, String> {
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
    // Read last_file, if present get image dimensions and resize window.
    // Prefer in-memory settings for freshness
    let st = if let Some(state) = app.try_state::<AppState>() {
        state.settings.lock().clone()
    } else {
        load_state(&app)
    };
    let path = match st.last_file {
        Some(p) => PathBuf::from(p),
        None => return Ok(()),
    };
    let img = image::image_dimensions(&path)
        .map_err(|e| format!("failed to read image dimensions: {e}"))?;
    let (w, h) = (img.0 as f64, img.1 as f64);
    // Clamp to a reasonable maximum (e.g., 90% of current screen working area).
    if let Ok(monitor) = window.current_monitor() {
        if let Some(m) = monitor {
            let size = m.size();
            let max_w = (size.width as f64 * 0.9).max(200.0);
            let max_h = (size.height as f64 * 0.9).max(200.0);
            let scale = (max_w / w).min(max_h / h).min(1.0);
            let new_w = (w * scale).round() as u32;
            let new_h = (h * scale).round() as u32;
            let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                width: new_w as f64,
                height: new_h as f64,
            }));
            // Update aspect ratio cache
            if let Some(state) = app.try_state::<AppState>() {
                *state.aspect_ratio.lock() = Some(w / h);
            }
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
#[tauri::command]
fn quick_look(_app: AppHandle) -> Result<(), String> {
    // Placeholder: No-op for now; future change may implement.
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
fn quick_look(_app: AppHandle) -> Result<(), String> {
    Ok(())
}

fn apply_initial_window_state(app: &AppHandle, window: &WebviewWindow) {
    // Always-on-top
    let _ = window.set_always_on_top(true);

    // Restore last size
    let st = load_state(app);
    if let (Some(w), Some(h)) = (st.window_w, st.window_h) {
        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: w,
            height: h,
        }));
    }

    // Title reflects last file if present
    if let Some(p) = st.last_file.clone() {
        let path = PathBuf::from(p);
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let _ = window.set_title(&format!("Always On Top — {}", name));
        // Initialize aspect ratio cache
        if let Ok((w, h)) = image::image_dimensions(&path) {
            if h > 0 {
                if let Some(state) = app.try_state::<AppState>() {
                    *state.aspect_ratio.lock() = Some(w as f64 / h as f64);
                }
            }
        }
    }

    // Auto-fit on startup if enabled and last file exists
    if st.fit_window && st.last_file.is_some() {
        let _ = fit_now(app.clone(), window.clone());
    }
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
    fit_window: Option<bool>,
    aspect_lock: Option<bool>,
}

#[tauri::command]
fn set_settings(app: AppHandle, update: SettingsUpdate) -> Result<PersistedState, String> {
    let win = app
        .get_webview_window("main")
        .ok_or("missing main window")?;
    let mut st = if let Some(state) = app.try_state::<AppState>() {
        state.settings.lock().clone()
    } else {
        load_state(&app)
    };
    if let Some(v) = update.fit_window {
        st.fit_window = v;
        if let Some(state) = app.try_state::<AppState>() {
            if let Some(toggle) = state.fit_toggle.lock().clone() {
                let _ = toggle.set_checked(v);
            }
        }
    }
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

fn schedule_size_save(app: AppHandle, win: WebviewWindow) {
    if let Some(state) = app.try_state::<AppState>() {
        let mut pending = state.pending_save.lock();
        if let Some(handle) = pending.take() {
            handle.abort();
        }
        let app_for_task = app.clone();
        let win_for_task = win.clone();
        let handle = async_runtime::spawn(async move {
            sleep(Duration::from_millis(1000)).await;
            if let Some(state) = app_for_task.try_state::<AppState>() {
                let st = state.settings.lock().clone();
                let _ = save_state(&app_for_task, &win_for_task, st);
            } else {
                let st = load_state(&app_for_task);
                let _ = save_state(&app_for_task, &win_for_task, st);
            }
        });
        *pending = Some(handle);
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
                    &MenuItemBuilder::with_id("open", "Open…")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Cmd+O"
                        } else {
                            "Ctrl+O"
                        })
                        .build(&app_handle)?,
                )
                .build()?;
            let st_for_menu = load_state(&app_handle);
            let fit_toggle =
                CheckMenuItemBuilder::with_id("fit_window_toggle", "Auto-fit on selection")
                    .checked(st_for_menu.fit_window)
                    .build(&app_handle)?;
            let aspect_toggle =
                CheckMenuItemBuilder::with_id("aspect_lock_toggle", "Lock aspect ratio on resize")
                    .checked(st_for_menu.aspect_lock)
                    .build(&app_handle)?;
            let mut view_menu = SubmenuBuilder::new(&app_handle, "View")
                .item(
                    &MenuItemBuilder::with_id("fit_now", "Fit to Image Now")
                        .accelerator(if cfg!(target_os = "macos") {
                            "Cmd+F"
                        } else {
                            "Ctrl+F"
                        })
                        .build(&app_handle)?,
                )
                .item(&fit_toggle)
                .item(&aspect_toggle);
            view_menu = view_menu.item(
                &MenuItemBuilder::with_id("inspect", "Toggle Inspector")
                    .accelerator(if cfg!(target_os = "macos") {
                        "Alt+Cmd+I"
                    } else {
                        "Ctrl+Shift+I"
                    })
                    .build(&app_handle)?,
            );
            #[cfg(target_os = "macos")]
            {
                view_menu = view_menu.item(
                    &MenuItemBuilder::with_id("quick_look", "Quick Look")
                        .accelerator("Cmd+Y")
                        .build(&app_handle)?,
                );
            }
            let app_menu = MenuBuilder::new(&app_handle)
                .item(&file_menu)
                .item(&view_menu.build()?)
                .build()?;
            app.set_menu(app_menu)?;
            if let Some(state) = app_handle.try_state::<AppState>() {
                *state.fit_toggle.lock() = Some(fit_toggle.clone());
                *state.aspect_toggle.lock() = Some(aspect_toggle.clone());
            }

            // Initialize in-memory settings before applying window state
            if let Some(state) = app_handle.try_state::<AppState>() {
                *state.settings.lock() = load_state(&app_handle);
            }
            // Apply initial window state when ready
            let win = app_handle
                .get_webview_window("main")
                .expect("main window exists");
            apply_initial_window_state(&app_handle, &win);

            // Persist on resize
            win.on_window_event(move |e| {
                if let WindowEvent::Resized(size) = e {
                    let win = app_handle.get_webview_window("main").unwrap();
                    // Enforce aspect lock if enabled
                    if let Some(state) = app_handle.try_state::<AppState>() {
                        if state.adjusting_resize.load(Ordering::Relaxed) {
                            return;
                        }
                        let st = state.settings.lock().clone();
                        if st.aspect_lock {
                            if let Some(r) = *state.aspect_ratio.lock() {
                                if r.is_finite() && r > 0.0 {
                                    // Width drives; compute height = width / r
                                    let new_w = size.width as f64;
                                    let new_h = (new_w / r).round().max(1.0);
                                    state.adjusting_resize.store(true, Ordering::Relaxed);
                                    let _ =
                                        win.set_size(tauri::Size::Logical(tauri::LogicalSize {
                                            width: new_w,
                                            height: new_h,
                                        }));
                                    state.adjusting_resize.store(false, Ordering::Relaxed);
                                }
                            }
                        }
                        // Debounced persist of latest window size (respecting any adjustment)
                        schedule_size_save(app_handle.clone(), win.clone());
                    } else {
                        schedule_size_save(app_handle.clone(), win.clone());
                    }
                }
            });
            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => {
                let handle = app.clone();
                async_runtime::spawn(async move {
                    let _ = choose_file(handle).await;
                });
            }
            "fit_now" => {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = fit_now(app.clone(), win);
                }
            }
            "quick_look" => {
                let _ = quick_look(app.clone());
            }
            "fit_window_toggle" => {
                if let Some(state) = app.try_state::<AppState>() {
                    let mut s = state.settings.lock().clone();
                    let new_state = if let Some(toggle) = state.fit_toggle.lock().clone() {
                        if let Ok(current) = toggle.is_checked() {
                            let desired = !current;
                            let _ = toggle.set_checked(desired);
                            desired
                        } else {
                            !s.fit_window
                        }
                    } else {
                        !s.fit_window
                    };
                    s.fit_window = new_state;
                    if let Some(win) = app.get_webview_window("main") {
                        let _ = save_state(app, &win, s.clone());
                    }
                    *state.settings.lock() = s;
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
                    if let Some(win) = app.get_webview_window("main") {
                        let _ = save_state(app, &win, s.clone());
                    }
                    *state.settings.lock() = s;
                }
            }
            "inspect" => {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.open_devtools();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            choose_file,
            fit_now,
            quick_look,
            get_settings,
            set_settings,
            load_image_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
