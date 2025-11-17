# last-file-persistence Specification

## Purpose
TBD - created by archiving change persist-settings-and-last-file. Update Purpose after archive.
## Requirements
### Requirement: Persist last opened file
The application MUST persist the absolute path of the last opened file in the same JSON file and restore it on startup if the file still exists.

#### Scenario: Restore last file
- Given the user previously opened a file
- And the file still exists
- When the application starts
- Then the window title updates to include the file name
- And the image is displayed without requiring a new selection

#### Scenario: Missing file
- Given the last file path points to a missing file
- When the application starts
- Then the application ignores the missing path and continues without a selection

