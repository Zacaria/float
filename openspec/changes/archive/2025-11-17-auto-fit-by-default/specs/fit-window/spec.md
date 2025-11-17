## MODIFIED Requirements

### Requirement: Fit window to image
The application MUST auto-fit the window to the newly selected image by default, clamped to the visible screen area. A manual Fit Now action MUST remain available.

#### Scenario: Auto-fit on selection
- Given the Fit window to image setting is enabled (default)
- And the user selects an image
- Then the window resizes to fit the image within visible screen bounds

#### Scenario: Manual fit remains available
- Given an image is displayed
- When the user triggers the manual Fit Now action
- Then the window resizes to match the image within reasonable screen bounds
