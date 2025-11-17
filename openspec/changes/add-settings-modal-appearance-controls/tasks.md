## 1. Implementation
- [ ] 1.1 Extend persisted settings to include window opacity (bounded default) and blur enabled, with migration-safe defaults.
- [ ] 1.2 Add Tauri-side commands/state to read/apply opacity and blur on startup and when updated.
- [ ] 1.3 Build a Settings modal in the WebView with General + Shortcuts tabs; General shows current fit/aspect states and appearance controls with live preview.
- [ ] 1.4 Populate the Shortcuts tab with the active platform-specific shortcuts for available commands (Settings, Open, Fit Now, Quick Look on macOS).
- [ ] 1.5 Wire Settings menu/shortcut (Cmd+,) to open the modal and ensure changes persist and update menu checkmarks/state.

## 2. Validation
- [ ] 2.1 `openspec validate add-settings-modal-appearance-controls --strict`.
- [ ] 2.2 `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and `cargo check` (root and `src-tauri`).
- [ ] 2.3 Manual: launch Tauri dev build, open Settings, verify current states visible, adjust opacity/blur and see immediate effect and persistence across restart, confirm shortcuts tab matches menu shortcuts.
