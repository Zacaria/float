## ADDED Requirements

### Requirement: Quick Look preview
The application MUST provide a Quick Look preview of the selected file on macOS.

#### Scenario: Quick Look via menu or shortcut
- Given a file is selected
- When the user chooses Quick Look from the menu
- Or presses the shortcut (Cmd+Y)
- Then the system Quick Look panel opens to preview the file

#### Scenario: No file selected
- Given no file is selected
- When the user triggers Quick Look
- Then no preview opens and the app continues running
