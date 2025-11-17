# settings-panel (Change Delta)

## MODIFIED Requirements

### Requirement: Settings panel
The Settings panel MUST be reachable from the app menu/shortcut (Cmd+, on macOS, Ctrl+, on Windows) and present General and Shortcuts tabs. The General tab MUST show current operational options (Fit window to image, Lock aspect ratio) plus window appearance controls (opacity 0–100% and blur) that reflect the persisted values and allow edits with live preview. The Shortcuts tab MUST list the active shortcuts for commands that remain available on the platform.

#### Scenario: Shortcuts tab lists active shortcuts
- Given the Settings modal is open
- When the user selects the Shortcuts tab
- Then it lists the active shortcuts for available commands on the platform (e.g., Settings, Open File, Fit to Image Now)
- And the key labels follow platform conventions (Cmd on macOS, Ctrl on Windows)
