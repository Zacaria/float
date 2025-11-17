# file-selection Specification

## Purpose
TBD - created by archiving change create-macos-keep-on-top-app. Update Purpose after archive.
## Requirements
### Requirement: Native file selection
The application MUST accept only image files when selecting paths (single or multi-select), ignoring or rejecting videos and other non-image types.

#### Scenario: Filter to images on selection
- Given the app presents the native file dialog
- When the user selects files that include any non-image types (for example, videos)
- Then only image files (commonly used picture extensions) are accepted into the selection
- And if no images are chosen, the app reports no valid selection instead of loading unsupported files

#### Scenario: Multi-file navigation respects image-only filter
- Given the user selects multiple files in one dialog
- And some are not images
- When the selection is applied
- Then only the image files are loaded into the selection order for navigation
- And title/auto-fit/aspect behaviors apply only to those valid images

### Requirement: Native file selection accepts only images
The application MUST reject non-image files during native selection (single or multi-select) and only accept common picture extensions.

#### Scenario: Filter to images on selection
- Given the app presents the native file dialog
- When the user selects files that include any non-image types (for example, videos)
- Then only image files are accepted into the selection
- And if no images are chosen, the app reports no valid selection instead of loading unsupported files

#### Scenario: Multi-file navigation respects image-only filter
- Given the user selects multiple files in one dialog
- And some are not images
- When the selection is applied
- Then only the image files are loaded into the selection order for navigation
- And title/auto-fit/aspect behaviors apply only to those valid images

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

