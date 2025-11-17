## ADDED Requirements

### Requirement: Fit window to image
When enabled, the application MUST resize the window to match the selected image’s size, within reasonable screen bounds, whenever a new image is selected.

#### Scenario: Fit on image selection
- Given the Fit window to image setting is enabled
- And a user selects an image file
- Then the window resizes to the image’s dimensions (clamped to the visible screen)

#### Scenario: Disabled behavior
- Given the Fit window to image setting is disabled
- When a user selects an image file
- Then the window size remains unchanged

