# menu-and-shortcuts Specification

## Purpose
TBD - created by archiving change create-macos-keep-on-top-app. Update Purpose after archive.
## Requirements
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

### Requirement: Commands available via menu bar (no in-window toolbar duplicates)
The File/Open and View/Fit to Image commands MUST be available from the macOS/Windows menu bar (with existing shortcuts) and MUST NOT be duplicated as in-window buttons. Quick Look MUST NOT appear in menus or shortcuts, and the app MUST NOT add a full-screen menu item or shortcut.

#### Scenario: Open via menu or shortcut only
- Given the app is running
- When the user wants to open a file
- Then the action is available via the File → Open menu item and platform shortcut (Cmd/Ctrl+O)
- And there is no Open button inside the WebView content

#### Scenario: Fit via menu or shortcut only
- Given the app is running
- When the user wants to Fit to Image
- Then the action is available via menu/shortcut
- And there is no Fit button inside the WebView content

#### Scenario: No Quick Look or app full-screen commands
- Given the app is running on macOS or Windows
- When the user inspects the menu bar or tries Cmd+Y or any app-provided full-screen shortcut
- Then no Quick Look or app-provided full-screen command is exposed
- And triggering those app shortcuts does nothing (OS/system full-screen affordances remain unchanged)

### Requirement: Reset Cache command in menu bar
The app MUST provide a Reset Cache command accessible via the app menu and a platform-appropriate shortcut. Triggering Reset Cache clears persisted app state, closes all open windows, and reopens a fresh window with empty state.

#### Scenario: Reset Cache via menu
- Given the app is running with one or more windows open
- When the user selects the Reset Cache menu item
- Then persisted state (settings, last opened files list, window sizes) is deleted or reset to defaults
- And all app windows close as part of the reset
- And a new window opens in a clean state

#### Scenario: Reset Cache via shortcut
- Given the app is running
- When the user presses the Reset Cache shortcut (Cmd+Shift+Backspace on macOS, Ctrl+Shift+Backspace on Windows)
- Then the Reset Cache behavior occurs identically to the menu invocation, including reopening a clean window

### Requirement: Toggle commands surfaced via menu bar
Aspect Lock and Auto-fit toggles MUST be exposed via menu items (with checkmarks) so users can enable/disable them without in-window checkboxes.

#### Scenario: Toggle Fit window via menu
- Given the app is running
- When the user toggles the Auto-fit option from the menu
- Then the option state is persisted and reflected in behavior without needing an in-window checkbox

#### Scenario: Toggle Aspect Lock via menu
- Given the app is running
- When the user toggles Aspect Lock from the menu
- Then the option state is persisted and enforced without needing an in-window checkbox

