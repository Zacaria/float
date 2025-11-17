## ADDED Requirements

### Requirement: Persist window size
The application MUST persist the window size to JSON and restore it on startup.

#### Scenario: Restore size on startup
- Given the user resized the window in a previous session
- When the application starts
- Then the window size is restored before any optional image fit adjustment

#### Scenario: Save on resize
- Given the application is running
- When the window is resized
- Then the new size is saved to the JSON file

