# settings-persistence (Change Delta)

## MODIFIED Requirements

### Requirement: Persist settings to JSON
The application MUST persist the values of Fit window to image, Lock aspect ratio on resize, window opacity (0–100%), and blur enabled to a JSON file alongside existing entries. If no persisted value exists, Fit window MUST default to enabled, opacity MUST default to fully opaque (100%), and blur MUST default to disabled.

#### Scenario: Default appearance when settings are missing
- Given no settings file is present
- When the app loads settings at startup
- Then Fit window defaults to enabled
- And window opacity defaults to fully opaque with blur disabled

#### Scenario: Restore settings on startup
- Given the settings file contains values for Fit window, Aspect lock, opacity, and blur
- When the application starts
- Then those values are applied before the window appears
- And the Settings modal controls reflect those values when opened

#### Scenario: Persist updates from Settings
- Given the user changes Fit window, Aspect lock, opacity, or blur from the Settings modal
- When the change is applied
- Then the settings file is updated with the new values for subsequent launches
