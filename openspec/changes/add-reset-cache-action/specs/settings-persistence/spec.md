# settings-persistence (Change Delta)

## MODIFIED Requirements

### Requirement: Persist settings to JSON
- The application MUST persist user settings in a JSON file under the OS-specific config directory and reload them at startup.

#### Scenario: Reset clears persisted settings
- Given settings have been saved previously
- When the user triggers Reset Cache
- Then the settings JSON is deleted or overwritten to defaults
- And the next app launch starts with default settings applied
