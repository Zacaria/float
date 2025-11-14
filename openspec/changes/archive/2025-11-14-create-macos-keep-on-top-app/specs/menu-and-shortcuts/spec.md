## ADDED Requirements

### Requirement: Menu and keyboard shortcuts
The application MUST expose a visible macOS menu with items to open a file and trigger Quick Look, with matching keyboard shortcuts.

#### Scenario: Menu presence
- Given the app is launched on macOS
- Then the menu bar includes a File menu with "Open…"
- And a View (or equivalent) menu with "Quick Look"

#### Scenario: Shortcuts
- Given the app is active
- When the user presses Cmd+O
- Then a native file dialog opens
- When the user presses Cmd+Y
- Then Quick Look is triggered for the selected file
