## MODIFIED Requirements

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
