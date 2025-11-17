## Why
The current webview toolbar duplicates commands (Open, Fit, Quick Look, aspect lock, auto-fit) that are also exposed via menus/shortcuts, leading to double entry points and inconsistent behavior. The request is to move these commands to the menu bar only.

## What Changes
- Remove command UI from the in-window toolbar and rely on menu bar entries (and shortcuts) for Open, Fit to Image, and Quick Look.
- Expose command toggles (auto-fit, aspect lock) via menu items so users can still toggle them without in-window controls.
- Keep status/selection updates intact while simplifying the UI.

## Impact
- Affected specs: menu-and-shortcuts (command placement), potentially settings-panel for toggle accessibility.
- Affected code: `dist/index.html` UI, Tauri menu setup in `src-tauri/src/main.rs`, any frontend command wiring.
