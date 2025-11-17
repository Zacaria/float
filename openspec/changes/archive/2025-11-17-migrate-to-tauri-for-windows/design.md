# Design: Tauri migration (cross-platform shell)

## Architecture
- Shell: Tauri (Rust core + WebView frontend)
- Frontend: Minimal HTML/JS/CSS to render selected image; no framework required
- Backend: Tauri commands for:
  - Opening native file dialog and returning selected path
  - Persisting settings/state (window size, fit/aspect) to config dir
  - Optional platform hooks (macOS Quick Look only)

## Windowing
- Single main window, always-on-top enabled on both macOS and Windows
- Restore last window size on startup; clamp to visible screen if needed

## Data Flow
- User triggers Open → Tauri dialog returns path → frontend displays image via `file://` or an object URL granted by Tauri FS permission
- Fit Now resizes window to the image’s dimensions within screen bounds
- Aspect lock constrains manual resizes (CSS natural ratio or JS listener)

## Platform
- macOS: Keep Quick Look command; no-op on Windows
- Windows: Map shortcuts to Ctrl variants; ensure file path handling (UTF-16) works through Tauri APIs

## Packaging
- Use `tauri build` to generate macOS `.app` and Windows installer (default NSIS)
- Replace `cargo-bundle` target and update docs/Justfile accordingly

## Risks / Trade-offs
- WebView UI replaces Cocoa layer; keep frontend minimal to reduce complexity
- Quick Look remains macOS-only to avoid scope creep; Windows feature parity intentionally limited in v1

