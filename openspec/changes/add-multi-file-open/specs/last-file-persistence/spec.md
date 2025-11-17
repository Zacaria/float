# Last File Persistence (Change Delta)

## MODIFIED Requirements

### Requirement: Persist last opened file
The application MUST persist the absolute path of the active file—even when chosen from a multi-file selection—and restore it on startup if the file still exists.

#### Scenario: Persist active file from multi-selection
- Given the user selected multiple files in one dialog
- And the user navigated to a specific file from that selection
- When the application saves settings or closes
- Then the persisted last file path is the currently active file
- And when the application starts again and that file exists, it becomes the active selection without requiring a new dialog
- And if the stored file is missing, the app starts without a selection as before
