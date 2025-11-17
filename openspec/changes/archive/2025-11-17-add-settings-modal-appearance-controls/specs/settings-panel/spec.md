# settings-panel (Change Delta)

## MODIFIED Requirements

### Requirement: Settings panel
The Settings panel MUST be reachable from the app menu/shortcut (Cmd+, on macOS, Ctrl+, on Windows) and present General and Shortcuts tabs. The General tab MUST show current operational options (Fit window to image, Lock aspect ratio) plus window appearance controls (opacity 0–100% and blur) that reflect the persisted values and allow edits with live preview. The Shortcuts tab MUST list the active shortcuts for available commands.

#### Scenario: Open Settings to view current parameters
- Given the app is running
- When the user opens Settings from the app menu or presses the platform shortcut
- Then the modal opens to the General tab showing the current states for Fit window to image and Lock aspect ratio
- And the current opacity and blur settings are shown using bounded controls

#### Scenario: Shortcuts tab lists active shortcuts
- Given the Settings modal is open
- When the user selects the Shortcuts tab
- Then it lists the active shortcuts for commands available on the platform (Settings, Open File, Fit to Image Now, Quick Look on macOS)
- And the key labels follow platform conventions (Cmd on macOS, Ctrl on Windows)

#### Scenario: Adjust appearance from Settings
- Given the Settings modal is open on the General tab
- When the user changes the opacity or blur control
- Then the main window updates immediately while the modal remains open
