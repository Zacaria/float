## Why
- The current window keeps standard OS decorations, which are visually distracting for a frameless always-on-top viewer.
- Removing the title bar requires an alternate drag handle so users can still reposition the window while viewing an image.

## What Changes
- Make the main window borderless on macOS and Windows while keeping resize/always-on-top behavior intact.
- Allow moving the window by dragging the displayed image/placeholder frame without interfering with overlay controls.
- Capture the borderless/drag behavior in a new window-frame capability spec.

## Impact
- Specs: new window-frame capability describing borderless window behavior and drag regions.
- Code: Tauri window configuration (decorations off) plus frontend markup/styles to mark draggable content and non-draggable controls.
