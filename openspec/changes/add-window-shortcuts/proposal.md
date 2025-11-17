## Why
- Users currently have only one window and must reuse the same Open command; there is no quick way to spawn or close windows from the keyboard.
- Standard macOS shortcuts (Cmd+T/W/Q with Cmd+O unchanged) align the app with user expectations for windowed viewers and reduce friction when handling multiple files.

## What Changes
- Add platform-appropriate shortcuts to open a new file in a new window (Cmd/Ctrl+T), close the focused window (Cmd/Ctrl+W), close all windows and quit (Cmd/Ctrl+Q), while keeping Cmd/Ctrl+O bound to opening in the focused window.
- Define how new windows launch (always-on-top, inherit settings as appropriate), how focus is handled, and how closing/quitting interacts with persisted settings.
- Update menus to surface these actions and ensure Tauri command handling routes to the correct window context.

## Impact
- Affected specs: menu-and-shortcuts (shortcut coverage), new window-management capability for multi-window lifecycle and persistence expectations.
- Affected code: Tauri shell window creation/teardown, menu wiring, and settings/state persistence.
