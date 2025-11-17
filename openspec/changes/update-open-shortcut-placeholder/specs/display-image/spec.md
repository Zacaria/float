## MODIFIED Requirements
### Requirement: Display selected image in main frame
The application MUST render the selected image in the window content and keep it in view until a different selection replaces it. When no file is selected, the placeholder MUST invite opening a file using the platform shortcut (⌘+O on macOS, Ctrl+O on Windows).

#### Scenario: Show newly selected image
- Given the user selects an image via the native file dialog
- When the selection completes
- Then the image appears in the main window content, scaled to fit within the window while preserving its aspect ratio
- And when another image is selected later, the displayed image updates to the new selection

#### Scenario: Startup with last file
- Given a previously selected image still exists on disk
- When the application starts
- Then the last image appears in the main window content without requiring a new selection

#### Scenario: Missing selection shows placeholder
- Given no image is selected or the remembered file is missing
- When the application is running
- Then the main window shows a neutral placeholder state instead of a broken or stale image
- And the placeholder text invites opening a file with the platform shortcut (⌘+O on macOS, Ctrl+O on Windows)
