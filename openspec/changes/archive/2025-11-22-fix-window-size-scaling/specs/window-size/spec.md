## MODIFIED Requirements
### Requirement: Persist window size
- The application MUST persist the window size to JSON and restore it on startup.
- Saving MUST occur after a brief quiet period instead of every resize event to capture the final size while limiting disk writes.
- Persisted dimensions MUST represent the logical (DPI-adjusted) window size so restoring on any display yields the same on-screen size.
- When loading legacy saves that stored physical dimensions, the application MUST convert them to logical units using the current scale factor.

#### Scenario: Reset clears saved sizes
- Given window size has been saved previously
- When the user triggers Reset Cache
- Then any persisted window size data is removed or reset
- And the fresh window opened after reset (and on next launch) uses defaults until resized again

#### Scenario: Restored size uses logical units
- Given the user resized the window while on a high-DPI display
- And the window size was saved
- When the application restarts
- Then the window opens at the same apparent size on screen (not doubled or halved due to scale factor)
