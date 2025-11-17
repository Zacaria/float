# Proposal: Enable auto-fit by default

- Change ID: auto-fit-by-default
- Summary: Change behavior so the window auto-fits to the selected image by default when a new file is chosen. Keep the manual “Fit Now” action available and persist the user preference.

## Why
- Users expect the window to size itself to the chosen image without extra steps.
- The existing manual-only flow creates friction for the common case.

## What Changes
- Fit window to image is ON by default (configurable via persisted setting).
- On image selection, the window auto-fits (with screen clamp) when the setting is enabled.
- Manual Fit Now action remains available.

