# Proposal: Switch image fit to manual button action

- Change ID: switch-fit-to-button
- Summary: Replace the automatic “fit window to image” toggle with a manual “Fit Window to Image Now” button in the Settings panel. Remove auto-fit-on-selection behavior.

## Why
Auto-fitting on every selection can be disruptive; users requested explicit control to fit only when desired.

## What Changes
- Settings panel adds a button “Fit Window to Image Now”.
- Automatic fit on image selection is disabled.
- A programmatic action (“Fit Now”) resizes the window to the current image dimensions (clamped to screen).

