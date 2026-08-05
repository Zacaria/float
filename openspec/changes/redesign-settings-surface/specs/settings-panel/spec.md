# settings-panel (Change Delta)

## MODIFIED Requirements

### Requirement: Settings panel
The Settings panel MUST be reachable from the app menu or platform shortcut (`Cmd+,` on macOS, `Ctrl+,` on Windows) and present Behavior and Appearance tabs in a dedicated separate settings window. The Behavior tab MUST surface persistent viewer preferences including Lock aspect ratio, Click-through overlay, and slideshow timing. The Appearance tab MUST surface supported native window appearance controls and clearly mark unavailable capabilities. The panel MUST remain lightweight and visually polished, with clear grouping, labels, and hierarchy.

#### Scenario: Open Settings to view current preferences
- Given the app is running
- When the user opens Settings from the app menu or presses the platform shortcut
- Then the Settings surface opens or focuses a dedicated settings window on the Behavior tab
- And it shows the current persisted states for the viewer preferences supported by that shell
- And the Appearance tab reflects the current window settings and platform capabilities

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
