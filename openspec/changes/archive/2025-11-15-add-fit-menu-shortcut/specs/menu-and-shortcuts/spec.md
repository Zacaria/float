## MODIFIED Requirements

### Requirement: Menu and keyboard shortcuts
The application MUST include a View menu item "Fit to Image Now" with shortcut Cmd+F that triggers the manual fit action.

#### Scenario: View → Fit Now
- Given an image is displayed
- When the user chooses View → Fit to Image Now or presses Cmd+F
- Then the window resizes to the image (clamped to the visible screen)

