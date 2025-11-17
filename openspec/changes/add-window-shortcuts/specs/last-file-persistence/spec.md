# Last File Persistence (Change Delta)

## MODIFIED Requirements

### Requirement: Persist last opened file
The application MUST persist the absolute path of the active file from the last focused window when quitting and restore it on startup if the file still exists.

#### Scenario: Persist last focused window on quit
- Given multiple windows are open
- And one window is focused with an active file
- When the user quits the application (for example, Cmd/Ctrl+Q or closing the last window)
- Then the persisted last file path matches the focused window’s active file
- And when the application starts again and that file exists, it becomes the active selection without requiring a new dialog
- And if the stored file is missing, the app starts without a selection as before
