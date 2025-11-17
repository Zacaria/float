# Proposal: Migrate to Tauri for Windows distribution

- Change ID: migrate-to-tauri-for-windows
- Summary: Migrate the app from a macOS-focused winit/Cocoa shell to Tauri to preserve core behavior and enable packaging and distribution on Windows.

## Why
- Deliver a Windows build without maintaining a second codepath.
- Leverage Tauri’s cross-platform windowing, dialogs, and bundler.
- Simplify packaging steps across macOS and Windows.

## What Changes
- Adopt Tauri as the application shell (Rust backend + WebView frontend).
- Replicate core features: always-on-top window, native file dialog, window size persistence, manual Fit Now, and menu shortcuts.
- Keep Quick Look as a macOS-only affordance; on Windows it is omitted.
- Replace `cargo-bundle` with Tauri bundler for packaging; produce macOS `.app` and Windows installer.

## Out of Scope (initial)
- Embedded Quick Look view or advanced previews.
- App signing/notarization and Windows code signing.
- CI pipelines for release publishing.

