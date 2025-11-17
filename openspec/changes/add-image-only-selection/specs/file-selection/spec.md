# File Selection (Change Delta)

## MODIFIED Requirements

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
