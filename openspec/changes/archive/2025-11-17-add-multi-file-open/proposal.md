## Why
- Today the app only opens one file at a time; browsing a set of images requires repeating Open… for each file.
- Allowing multiple files per selection keeps the always-on-top viewer useful for quick reviews and comparisons without constant dialog usage.

## What Changes
- Enable picking multiple files in one native dialog interaction and keep their absolute paths in order.
- Load the first selected file immediately and provide simple next/previous controls to move through the same selection without reopening the dialog.
- Apply existing behaviors (title updates, auto-fit, aspect lock, and persistence of the active file) to whichever file is currently active from the selection.

## Impact
- Affected specs: file-selection, last-file-persistence (to clarify multi-selection persistence behavior).
- Affected code: Tauri dialog invocation and state management, menu/shortcut wiring for navigation, frontend display of the active file status.
