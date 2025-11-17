## ADDED Requirements

### Requirement: Settings panel
The application MUST present a settings panel accessible from the app menu with at least the following options:
- Fit window to image
- Lock aspect ratio on resize

#### Scenario: Open settings
- Given the app is running on macOS
- When the user chooses Settings… from the app menu (Cmd+,)
- Then a panel opens with the listed options and their current states

#### Scenario: Toggle options
- Given the settings panel is open
- When the user toggles an option
- Then the setting updates immediately and applies to the current window behavior

