# settings-persistence Specification

## Purpose
TBD - created by archiving change persist-settings-and-last-file. Update Purpose after archive.
## Requirements
### Requirement: Persist settings to JSON
The application MUST persist the values of “Fit window to image” and “Lock aspect ratio on resize” to a JSON file under the user’s configuration directory and load them on startup.

#### Scenario: Load settings on startup
- Given the user has previously changed settings
- When the application starts
- Then the settings are loaded from the JSON file and applied immediately

#### Scenario: Save settings on change
- Given the settings panel is open
- When the user toggles an option and confirms
- Then the JSON file is updated to reflect the new values

