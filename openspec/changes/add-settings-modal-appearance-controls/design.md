# Design: Settings modal with appearance controls

## Overview
Add a Settings modal to the Tauri shell that surfaces operational toggles (auto-fit, aspect lock), shows shortcuts, and introduces window opacity and blur controls. Values must persist in the existing `settings.json` and apply immediately to the main window.

## UI/UX
- Entry: App menu item + Cmd+, shortcut opens the modal.
- Layout: Two tabs (General, Shortcuts). General holds toggles for Fit window and Aspect lock plus an opacity slider (0–100%) and blur toggle; Shortcuts lists active keybindings with platform naming (Cmd vs Ctrl).
- Defaults: Fully opaque window (100%), blur disabled. Opacity slider bounded to 0–100% but never hides the window entirely (0% still renders the surface).

## Data & Persistence
- Extend persisted state with `opacity` (float 0–1 or percent) and `blur_enabled` (bool); keep existing fields intact for compatibility.
- Load persisted values on startup to apply window appearance and populate the modal; write on change and window resize.

- ## Application behavior
- Apply opacity and blur via Tauri window APIs when settings change and on startup. If blur is unsupported on the host OS (e.g., disabled on Windows when API differs), disable the control in the UI and keep the window unblurred while persisting the requested state safely.
- Settings updates should also refresh menu toggle checkmarks for auto-fit/aspect when those values change in the modal.
