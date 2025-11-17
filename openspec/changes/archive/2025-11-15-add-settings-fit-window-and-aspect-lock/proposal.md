# Proposal: Add settings panel for image fit and aspect lock

- Change ID: add-settings-fit-window-and-aspect-lock
- Summary: Provide a simple settings panel with two options: (1) fit the window to the image size when an image is selected; (2) lock the window’s resize to keep the image’s aspect ratio.
- Motivation: Users want the pinned window to size itself to the image for optimal view, and to preserve aspect ratio when manually resizing for consistent presentation.

## Goals
- Add a visible settings UI reachable from the app menu.
- Fit window to image size when enabled.
- Lock resize to image aspect ratio when enabled.

## Non-Goals
- Persisting settings across launches (runtime only for now).
- Cross-platform settings UI (macOS only for this change).

