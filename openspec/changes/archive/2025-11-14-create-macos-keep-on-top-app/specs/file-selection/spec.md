## ADDED Requirements

### Requirement: Native file selection
The application MUST let the user choose a file using the macOS native file dialog.

#### Scenario: User opens the app and selects a file
- Given the app has launched successfully
- When the app presents a file dialog
- And the user selects a file and confirms
- Then the app records the absolute path of the selected file
- And the app updates the window title to include the selected file name

#### Scenario: User cancels file selection
- Given the app presents a file dialog
- When the user cancels the dialog
- Then the app keeps running without a selected file
- And the window title remains the default
