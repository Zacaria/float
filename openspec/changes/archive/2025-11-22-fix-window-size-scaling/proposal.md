## Why
Window size persistence saves physical pixels on high-DPI displays, so on relaunch the window restores at roughly 2× the visible size of the image.

## What Changes
- Persist and restore window size in logical units so relaunch matches the on-screen size the user set.
- Update the window-size spec with a scenario covering logical size persistence to prevent DPI scaling regressions.

## Impact
- Affected specs: window-size
- Affected code: Tauri window persistence and restore logic
