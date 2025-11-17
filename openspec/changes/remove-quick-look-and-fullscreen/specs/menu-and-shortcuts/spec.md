# menu-and-shortcuts (Change Delta)

## MODIFIED Requirements

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
