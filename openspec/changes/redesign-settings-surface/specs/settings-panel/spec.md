# settings-panel (Change Delta)

## MODIFIED Requirements

### Requirement: Settings panel
The Settings panel MUST be reachable from the app menu or platform shortcut (`Cmd+,` on macOS, `Ctrl+,` on Windows) and present General and Shortcuts tabs in a dedicated separate settings window. The General tab MUST surface the persistent viewer preferences supported by the Tauri shell, including Lock aspect ratio, Click-through overlay, slideshow timing, and any supported window appearance controls. The panel MUST remain lightweight and visually polished, with clear grouping, labels, and hierarchy. The Shortcuts tab MUST list the active shortcuts for commands that remain available on the current platform.

#### Scenario: Open Settings to view current preferences
- Given the app is running
- When the user opens Settings from the app menu or presses the platform shortcut
- Then the Settings surface opens or focuses a dedicated settings window on the General tab
- And it shows the current persisted states for the viewer preferences supported by that shell
- And any appearance controls reflect the current window settings

#### Scenario: Shortcuts tab lists active shortcuts
- Given the Settings panel is open
- When the user selects the Shortcuts tab
- Then it lists the active shortcuts for commands available on the current platform, including Settings, Open File, New Window, Fit to Image Now, Previous File, Next File, and Reset Cache when those commands are present
- And the key labels follow platform conventions (`Cmd` on macOS, `Ctrl` on Windows)

#### Scenario: Settings omits obsolete auto-fit toggle
- Given the Settings panel is open
- When the user inspects fit-related controls
- Then the panel exposes the manual Fit action only through the app's existing menu or shortcut surfaces
- And no persisted auto-fit toggle is shown

#### Scenario: Settings does not cover the active image
- Given a viewer window is displaying an image
- When the user opens Settings
- Then the image remains visible in its own viewer window
- And the settings UI does not render on top of that viewport
