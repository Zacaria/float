# File Selection (Change Delta)

## MODIFIED Requirements

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
