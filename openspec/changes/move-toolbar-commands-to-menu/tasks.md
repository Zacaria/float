## 1. Implementation
- [x] 1.1 Remove toolbar command controls from the WebView (`dist/index.html`), keeping any status display minimal.
- [x] 1.2 Add menu entries (with checkmarks) for Auto-fit and Aspect Lock toggles, wiring them to existing settings persistence/state.
- [x] 1.3 Keep Open, Fit to Image, and Quick Look commands available via menu/shortcuts only and ensure they still invoke the same backend commands.

## 2. Validation
- [x] 2.1 Run `cargo fmt` and `cargo clippy` for both crates.
- [x] 2.2 Run `cargo check` (workspace) and `cargo check --manifest-path src-tauri/Cargo.toml`.
- [ ] 2.3 Manual sanity: launch via `cargo tauri dev`, confirm no in-window command buttons/checkboxes, menu items perform Open/Fit/Quick Look, and menu toggles for Auto-fit/Aspect Lock update/reflect state.
