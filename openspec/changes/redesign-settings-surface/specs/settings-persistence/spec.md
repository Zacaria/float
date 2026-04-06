# settings-persistence (Change Delta)

## MODIFIED Requirements

### Requirement: Persist settings to JSON
The application MUST persist user settings in a JSON file under the OS-specific config directory and reload them at startup. Persisted settings MUST include the durable viewer preferences exposed by the active Tauri Settings surface, including Aspect Lock, Click-through overlay, slideshow timing, and supported window appearance values. The persisted settings contract MUST NOT include an auto-fit toggle because Fit is a manual action. Viewer session state such as the active file in a given window MUST NOT be shared through this settings payload.

#### Scenario: Reset clears persisted settings
- Given settings have been saved previously
- When the user triggers Reset Cache
- Then the settings JSON is deleted or overwritten to defaults
- And the app continues with default settings applied in the fresh window that opens after reset (and on next launch)

#### Scenario: Restore supported preferences on startup
- Given the settings file contains supported viewer preference values
- When the application starts
- Then those values are loaded before the user interacts with the app
- And the Settings surface reflects those values when opened

#### Scenario: Missing appearance settings use safe defaults
- Given no persisted appearance values are present
- When the app loads settings at startup
- Then window opacity defaults to a fully visible state
- And blur defaults to disabled

#### Scenario: Auto-fit toggle is not persisted
- Given the user opens Settings or restarts the app
- When the persisted settings are loaded
- Then no stored auto-fit preference affects image selection behavior
- And Fit remains available only as a manual action

#### Scenario: Active image state stays isolated per window
- Given two Float windows are open with different images selected
- When the user changes the active image in one window
- Then the other window keeps showing its own image
- And no shared settings payload overwrites that viewer state
