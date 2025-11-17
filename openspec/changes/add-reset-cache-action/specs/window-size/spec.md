# window-size (Change Delta)

## MODIFIED Requirements

### Requirement: Persist window size
- The application MUST persist the window size to JSON and restore it on startup.
- Saving MUST occur after a brief quiet period instead of every resize event to capture the final size while limiting disk writes.

#### Scenario: Reset clears saved sizes
- Given window size has been saved previously
- When the user triggers Reset Cache
- Then any persisted window size data is removed or reset
- And on next launch, the window uses defaults until resized again
