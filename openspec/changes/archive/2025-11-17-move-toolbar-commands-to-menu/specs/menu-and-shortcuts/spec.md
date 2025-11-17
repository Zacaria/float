# Menu and Shortcuts (Change Delta)

## MODIFIED Requirements

### Requirement: Commands available via menu bar (no in-window toolbar duplicates)
The File/Open, View/Fit to Image, and Quick Look commands MUST be available from the macOS/Windows menu bar (with existing shortcuts) and MUST NOT be duplicated as in-window buttons.

#### Scenario: Open via menu or shortcut only
- Given the app is running
- When the user wants to open a file
- Then the action is available via the File → Open menu item and platform shortcut (Cmd/Ctrl+O)
- And there is no Open button inside the WebView content

#### Scenario: Fit/Quick Look via menu or shortcut only
- Given the app is running
- When the user wants to Fit to Image or trigger Quick Look (macOS only)
- Then the actions are available via menu/shortcut
- And there are no Fit/Quick Look buttons inside the WebView content

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
