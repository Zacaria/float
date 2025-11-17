## Why
- Users need a quick way to clear persisted settings/state (e.g., cached window sizes and last-file list) and return the app to a clean slate when troubleshooting or starting fresh.
- Providing a menu item + shortcut keeps the action discoverable without exposing in-window UI clutter.

## What Changes
- Add a Reset Cache command in the app menu bar with a keyboard shortcut that clears persisted app state and closes all open windows.
- Ensure the app restarts in a clean state (no remembered files, default settings) after invoking Reset Cache.
- Keep platform parity for menu/shortcut exposure.

## Impact
- Affected specs: menu-and-shortcuts (new menu item + shortcut), settings-persistence/last-file-persistence/window-size (behavior when reset), settings-panel (may reference shortcut list if shown).
- Affected code: Tauri shell menu wiring, persistence files handling, multi-window lifecycle.
- Validation: Confirm shortcut/menu fires, clears persisted JSON/state, closes windows, and next launch starts fresh on macOS/Windows.
