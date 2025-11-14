## ADDED Requirements

### Requirement: Window stays always on top
The application window MUST remain above other application windows while it is open.

#### Scenario: Window is created
- Given the app creates its main window
- When the window appears
- Then the window is configured as always-on-top by default

#### Scenario: Always-on-top persists
- Given the window is configured as always-on-top
- When the user switches focus to other applications
- Then the app window remains visible above other windows until closed
