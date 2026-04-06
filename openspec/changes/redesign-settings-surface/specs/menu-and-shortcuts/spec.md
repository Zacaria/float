# menu-and-shortcuts (Change Delta)

## MODIFIED Requirements

### Requirement: Menu and keyboard shortcuts
The application MUST provide platform-appropriate shortcuts for the current app shell: `Cmd/Ctrl+,` opens Settings in the focused window, `Cmd/Ctrl+T` creates a new window with file selection, `Cmd/Ctrl+W` closes the focused window, `Cmd/Ctrl+Q` closes all windows and quits, and `Cmd/Ctrl+O` opens a file in the focused window.

#### Scenario: Open Settings via shortcut
- Given a window is focused
- When the user presses `Cmd+,` on macOS or `Ctrl+,` on Windows
- Then the app opens the Settings surface for that focused window
- And the current viewer state behind Settings remains intact

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

### Requirement: Shared settings toggles surfaced via menu bar
The View menu MUST expose the viewer toggles that are also owned by persistent Settings, including Aspect Lock and Click-through overlay, using checked menu items that stay synchronized with the Settings surface. The app MUST NOT expose an Auto-fit toggle because Fit is manual-only.

#### Scenario: Toggle Aspect Lock via menu
- Given the app is running
- When the user toggles Aspect Lock from the View menu
- Then the option state is reflected immediately in behavior
- And the Settings surface shows the same state when opened

#### Scenario: Toggle Click-through via menu
- Given the app is running
- When the user toggles Click-through overlay from the View menu
- Then the option state is reflected immediately in behavior
- And the Settings surface shows the same state when opened

#### Scenario: No Auto-fit toggle in View menu
- Given the app is running
- When the user inspects the View menu
- Then the manual Fit action is present
- And no Auto-fit toggle is exposed
