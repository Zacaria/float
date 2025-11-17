## ADDED Requirements

### Requirement: Native file selection triggers auto-fit
When the user selects a file using the native file dialog and the Fit window to image setting is enabled, the window MUST auto-fit to the selected image.

#### Scenario: Select file → auto-fit
- Given the user selects an image via the file dialog
- And Fit window to image is enabled
- Then the window resizes to fit the image within visible screen bounds
