# settings-panel Specification

## Purpose
TBD - created by archiving change add-settings-fit-window-and-aspect-lock. Update Purpose after archive.
## Requirements
### Requirement: Settings panel
The Settings panel MUST include two tabs: a General tab for operational options and a Shortcuts tab that lists the current keyboard shortcuts (read-only).

#### Scenario: Shortcuts tab
- Given the app is running on macOS
- When the user opens Settings and selects the Shortcuts tab
- Then a list shows the active shortcuts (e.g., Cmd+, for Settings, Cmd+O for Open, Cmd+Y for Quick Look)

