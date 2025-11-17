## MODIFIED Requirements
### Requirement: Persist window size
- The application MUST persist the window size to JSON and restore it on startup.
- Saving MUST occur after a brief quiet period instead of every resize event to capture the final size while limiting disk writes.

#### Scenario: Debounced save on resize
- Given the user is resizing the window
- When they stop resizing
- Then the window size is written to the JSON file within about one second using the final dimensions

#### Scenario: Restore size on startup
- Given the user previously resized the window and the size was saved
- When the application starts
- Then the window size is restored before any optional image fit adjustment
