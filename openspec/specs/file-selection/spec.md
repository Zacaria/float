# file-selection Specification

## Purpose
TBD - created by archiving change create-macos-keep-on-top-app. Update Purpose after archive.
## Requirements
### Requirement: Native file selection via Tauri dialog plugin
The application MUST use the Tauri dialog plugin to present the operating system’s native file picker when selecting a file, preserving current title updates and cancellation behavior.

#### Scenario: Select file using Tauri dialog plugin
- Given the user triggers file selection from the app
- When the Tauri dialog plugin opens the native picker and the user chooses a file
- Then the app records the absolute path of the selected file
- And the window title updates to include the selected file name

#### Scenario: Cancel file selection via Tauri dialog plugin
- Given the Tauri dialog plugin has opened the native picker
- When the user cancels the dialog
- Then the app keeps running without a selected file
- And the window title remains unchanged

### Requirement: Native file selection triggers auto-fit
When the user selects a file using the native file dialog and the Fit window to image setting is enabled, the window MUST auto-fit to the selected image.

#### Scenario: Select file → auto-fit
- Given the user selects an image via the file dialog
- And Fit window to image is enabled
- Then the window resizes to fit the image within visible screen bounds

