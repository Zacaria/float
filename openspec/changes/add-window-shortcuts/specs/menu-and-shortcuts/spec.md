# Menu and Shortcuts (Change Delta)

## MODIFIED Requirements

### Requirement: Menu and keyboard shortcuts
The application MUST provide platform-appropriate shortcuts for window management and file opening: Cmd/Ctrl+T (new window with file), Cmd/Ctrl+W (close focused window), Cmd/Ctrl+Q (close all and quit), while keeping Cmd/Ctrl+O as the Open command for the focused window.

#### Scenario: New window shortcut on macOS
- Given the app is running on macOS
- When the user presses Cmd+T
- Then the app opens a native file picker for creating a new window
- And after selection, a new window opens showing the chosen file and becomes focused

#### Scenario: New window shortcut on Windows
- Given the app is running on Windows
- When the user presses Ctrl+T
- Then the app opens a native file picker for creating a new window
- And after selection, a new window opens showing the chosen file and becomes focused

#### Scenario: Close focused window via shortcut
- Given multiple app windows are open
- When the user presses Cmd/Ctrl+W in the focused window
- Then that focused window closes
- And other windows remain open

#### Scenario: Quit all windows via shortcut
- Given one or more app windows are open
- When the user presses Cmd/Ctrl+Q
- Then all windows close and the app quits

#### Scenario: Open in focused window remains Cmd/Ctrl+O
- Given a window is focused
- When the user presses Cmd/Ctrl+O
- Then the native file picker opens for that window
- And the selection loads into that same window instead of creating a new one
