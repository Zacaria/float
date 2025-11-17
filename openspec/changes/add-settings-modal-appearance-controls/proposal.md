# Proposal: Add settings modal appearance controls

## Change ID
add-settings-modal-appearance-controls

## Why
- The Tauri shell lacks the Settings UI described in specs, so users cannot see or update auto-fit/aspect options or shortcut references.
- There is no way to adjust window opacity or blur, limiting the app's utility for overlay-style use cases.

## What Changes
- Add a Settings modal (Cmd+, / menu) with General and Shortcuts tabs in the Tauri shell.
- General tab surfaces current settings (Fit window to image, Lock aspect ratio) and new window appearance controls (opacity slider, blur toggle) with live preview.
- Shortcuts tab lists the active shortcuts for the available commands, matching platform conventions.
- Persist opacity/blur alongside existing settings and apply them on startup and when changed.

## Scope
- Tauri app shell (Rust) and minimal WebView UI.

## Out of Scope
- New file handling or preview formats.
- Quick Look behavior changes beyond showing the shortcut reference.

## Open Questions
- None (resolved): opacity is adjustable 0–100% and blur is disabled on Windows when the API differs from macOS.
