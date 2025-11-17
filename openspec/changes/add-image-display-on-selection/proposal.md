## Why
- Selecting an image currently only updates window metadata; the Tauri webview still shows a placeholder, so users cannot see the chosen file.
- Other behaviors (fit-to-image, aspect lock, last-file restore) assume an image is visible, so aligning the UI prevents regressions and matches the product intent.

## What Changes
- Render the selected (or restored) image inside the main window content, replacing the static placeholder.
- Update the display when a new selection is made and fall back to a neutral placeholder if no image is available.
- Keep image rendering lightweight and platform-neutral within the existing Tauri shell.

## Impact
- Touches the Tauri frontend and the selection-to-display wiring; no menu or settings changes.
- Requires manual validation on macOS and Windows for image display and fallback behavior.
