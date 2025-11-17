# fit-window Specification

## Purpose
TBD - created by archiving change add-settings-fit-window-and-aspect-lock. Update Purpose after archive.
## Requirements
### Requirement: Manual fit window action
The application MUST provide a manual Fit action (button/menu/shortcut) that adjusts the current window based on the displayed image’s aspect ratio without relying on persisted settings.

#### Scenario: Manual fit adjusts by current window size
- Given an image is displayed
- And the user triggers the Fit action
- Then the window keeps its larger dimension unchanged (width or height, whichever is greater)
- And the other dimension is reduced to match the image’s aspect ratio without upscaling

#### Scenario: Auto-fit toggle removed
- Given the user selects a new image
- Then the window does not auto-fit based on a stored toggle
- And the user can invoke the manual Fit action to resize as needed

