# last-file-persistence (Change Delta)

## MODIFIED Requirements

### Requirement: Persist last opened file
The application MUST persist an ordered list of the absolute paths for all open files in the same JSON file and restore them on startup when they still exist. Single-window usage remains supported by storing one entry.

#### Scenario: Restore last file
- Given the user previously opened one or more files and they still exist
- When the application starts
- Then each existing file from the saved list is reopened (order preserved)
- And at least the first available file updates the main window title and content without requiring a new selection

#### Scenario: Missing file
- Given the saved list includes a path that is now missing
- When the application starts
- Then the application ignores missing entries
- And continues restoring any remaining existing files (or starts with no selection if none exist)
