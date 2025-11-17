## MODIFIED Requirements

### Requirement: Fit window to image
The application MUST provide a manual action to fit the window to the current image. It MUST NOT auto-fit on each new image selection anymore.

#### Scenario: Manual fit
- Given an image is displayed
- When the user triggers the manual fit action
- Then the window resizes to match the image within reasonable screen bounds
