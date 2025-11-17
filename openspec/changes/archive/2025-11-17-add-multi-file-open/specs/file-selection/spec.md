# File Selection (Change Delta)

## ADDED Requirements

### Requirement: Multi-file selection and navigation
The application MUST accept multiple files in a single selection and let the user view each selected file without reopening the dialog.

#### Scenario: Select multiple files once
- Given the app is running
- When the user selects multiple files in one native dialog confirmation
- Then the app records all selected absolute paths in selection order
- And the first selected file becomes the active file with the window title updated to that name

#### Scenario: Navigate selected files without reopening
- Given multiple files were selected in one dialog interaction
- When the user requests the next or previous file from that selection using app controls (for example, a menu item or shortcut)
- Then the active file changes to the adjacent item in that selection order when one exists
- And the window title updates to the new file name
- And auto-fit and aspect-lock behaviors apply to the newly active file when enabled
- And if there is no next or previous item, the active file remains unchanged without an error
