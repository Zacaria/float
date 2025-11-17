# quick-look Specification

## Purpose
TBD - created by archiving change create-macos-keep-on-top-app. Update Purpose after archive.
## Requirements
### Requirement: Quick Look preview
The application MUST provide a Quick Look preview of the selected file on macOS. Windows has no Quick Look capability.

#### Scenario: Quick Look via menu or shortcut on macOS
- Given a file is selected on macOS
- When the user chooses Quick Look from the menu
- Or presses the shortcut (Cmd+Y)
- Then the system Quick Look panel opens to preview the file

#### Scenario: No file selected on macOS
- Given no file is selected on macOS
- When the user triggers Quick Look
- Then no preview opens and the app continues running

#### Scenario: Quick Look on Windows is unavailable
- Given the app is running on Windows
- When the user triggers Quick Look
- Then no preview opens and the app continues running without error

