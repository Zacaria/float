# window-management Specification

## Purpose
TBD - created by archiving change add-window-shortcuts. Update Purpose after archive.
## Requirements
### Requirement: Multiple always-on-top windows with focused actions
The application MUST support multiple always-on-top windows, each able to display its own selected file, and honor shortcut actions relative to the focused window.

#### Scenario: Create a new window with a selected file
- Given a window is focused
- When the user triggers the new window action (Cmd/Ctrl+T or equivalent menu item)
- Then the app opens a native file picker parented to the focused window
- And upon selection, a new always-on-top window is created showing the chosen file and becomes focused

#### Scenario: Close only the focused window
- Given multiple windows are open
- When the user triggers the close-window action (Cmd/Ctrl+W or equivalent menu item)
- Then the focused window closes
- And other windows remain open and focused behavior follows platform defaults

#### Scenario: Quit closes all windows
- Given one or more windows are open
- When the user triggers the quit action (Cmd/Ctrl+Q or equivalent menu item)
- Then all windows close and the application process exits

