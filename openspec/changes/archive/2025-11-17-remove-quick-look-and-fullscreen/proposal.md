## Why
- Quick Look is a macOS-only flow that increases maintenance surface and is currently a no-op in the Tauri shell; we want to simplify the viewer and avoid promising platform-specific previews.
- The app should remain a lightweight always-on-top window without full-screen behavior that can obscure other windows or conflict with the always-on-top affordance.
- Removing both features clarifies the supported surface and reduces menu/shortcut clutter, especially for Windows where Quick Look is inapplicable.

## What Changes
- Remove the Quick Look capability and any menu/shortcut references to it across platforms.
- Remove app-provided full-screen entry points (menus/shortcuts) so the app stays windowed by default while leaving OS-level affordances unchanged.
- Update settings/shortcut surfaces to reflect the reduced command set.

## Impact
- Affected specs: quick-look (removal), menu-and-shortcuts (menu coverage), settings-panel (shortcut listing), new window-mode constraint to keep the app windowed.
- Affected code: Tauri menu wiring and command handlers; any legacy Quick Look hooks; potential window configuration to prevent full-screen toggles.
- Validation: Confirm menus/shortcuts no longer expose Quick Look or full-screen and the app stays windowed on both macOS and Windows.
