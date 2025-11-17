## 1. Implementation
- [x] 1.1 Add the Tauri dialog plugin dependency, register it in the Tauri app, and remove `rfd` from relevant Cargo manifests and lockfiles.
- [x] 1.2 Replace `rfd` file-picking calls in the Tauri command layer with the dialog plugin while preserving test overrides and cancel handling; adjust any front-end invocation if required.
- [x] 1.3 Confirm file selection continues to update the window title, persist the last file, and trigger fit-to-image when enabled after switching to the plugin.

## 2. Validation
- [x] 2.1 Run `cargo fmt` and `cargo clippy` for the workspace to ensure formatting and lint checks pass.
- [x] 2.2 Run `cargo check -p always-on-top-tauri` and `cargo check -p always-on-top` to verify both crates build without `rfd`.
- [x] 2.3 Manual sanity check: launch via `cargo tauri dev` (or platform equivalent) and verify selecting/canceling a file uses the native dialog and keeps title/auto-fit behavior intact.
