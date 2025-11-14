# Proposal: Create macOS always-on-top app (Rust)

- Change ID: create-macos-keep-on-top-app
- Summary: Ship a tiny macOS GUI app written in Rust that opens a native file picker and keeps its window always on top. The window title reflects the selected file path.
- Motivation: Provide a simple, distraction-free utility to pin a small window above others while letting the user select a file to associate with it.
- Out of Scope (initial): Rendering file previews, controlling other apps’ windows.

## Goals
- Native file selection dialog.
- Window stays always on top by default and can be toggled in code if needed.
- Minimal UI and dependencies to favor reliability.

## Non-Goals
- Complex UI components or custom drawing.
- Interacting with other applications’ windows.

## Decision (2025-11-14)
- Provide Quick Look preview: Yes
- Add visible menu items and keyboard shortcuts: Yes
- Package as a macOS .app bundle: Yes

Scope and specs updated accordingly under this change.

## Why
Provide a lightweight macOS utility to pin a small window above others while quickly referencing a selected file (e.g., image or document), minimizing context switches. The app should be easy to run and distribute.

## What Changes
- Add native file selection and reflect the file name in the window title.
- Keep the window always-on-top while the app runs.
- Add macOS menu with File → Open… (Cmd+O) and View → Quick Look (Cmd+Y).
- Provide Quick Look preview of the selected file using the system panel.
- Package as a `.app` bundle with metadata in `Cargo.toml`.
