# window-mode Specification

## Purpose
TBD - created by archiving change remove-quick-look-and-fullscreen. Update Purpose after archive.
## Requirements
### Requirement: Windowed-only mode
The application MUST remain in a windowed, always-on-top mode by default and MUST NOT expose app-provided menus or shortcuts to enter full-screen. OS-level window affordances remain unchanged.

#### Scenario: App full-screen affordances are absent
- Given the app is running on macOS or Windows
- When the user inspects the app menus/shortcuts for full-screen options
- Then no app-provided full-screen command is available and the window remains in its windowed, always-on-top state (system-level full-screen controls behave per OS defaults)

