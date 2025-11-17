# file-selection Specification

## Purpose
TBD - created by archiving change create-macos-keep-on-top-app. Update Purpose after archive.
## Requirements
### Requirement: Native file selection
The application MUST let the user choose a file using the operating system’s native file dialog on macOS and Windows.

#### Scenario: User selects a file on macOS
- Given the app is running on macOS
- And the app presents a file dialog
- When the user selects a file and confirms
- Then the app records the absolute path of the selected file
- And the app updates the window title to include the selected file name

#### Scenario: User selects a file on Windows
- Given the app presents a file dialog on Windows
- When the user selects a file and confirms
- Then the app records the absolute path of the selected file
- And the app updates the window title to include the selected file name

#### Scenario: User cancels file selection
- Given the app presents a file dialog
- When the user cancels the dialog
- Then the app keeps running without a selected file
- And the window title remains the default

### Requirement: Native file selection triggers auto-fit
When the user selects a file using the native file dialog and the Fit window to image setting is enabled, the window MUST auto-fit to the selected image.

#### Scenario: Select file → auto-fit
- Given the user selects an image via the file dialog
- And Fit window to image is enabled
- Then the window resizes to fit the image within visible screen bounds

