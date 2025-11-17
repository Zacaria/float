# settings-persistence Specification

## Purpose
TBD - created by archiving change persist-settings-and-last-file. Update Purpose after archive.
## Requirements
### Requirement: Persist settings to JSON
The application MUST persist the values of “Fit window to image” and “Lock aspect ratio on resize” to a JSON file. If no persisted value exists, “Fit window to image” MUST default to enabled.

#### Scenario: Default to auto-fit when unset
- Given the app starts with no settings file present
- When the first image is selected
- Then auto-fit occurs because the default for Fit window to image is enabled

