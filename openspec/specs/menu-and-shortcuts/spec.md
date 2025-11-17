# menu-and-shortcuts Specification

## Purpose
TBD - created by archiving change create-macos-keep-on-top-app. Update Purpose after archive.
## Requirements
### Requirement: Menu and keyboard shortcuts
The Fit to Image Now action MUST be available via menu and a platform-appropriate shortcut: Cmd+F on macOS and Ctrl+F on Windows.

#### Scenario: Fit Now shortcut on macOS
- Given an image is displayed
- When the user chooses View → Fit to Image Now or presses Cmd+F
- Then the window resizes to the image (clamped to the visible screen)

#### Scenario: Fit Now shortcut on Windows
- Given an image is displayed
- When the user presses Ctrl+F
- Then the window resizes to the image (clamped to the visible screen)

