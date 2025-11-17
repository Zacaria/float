# window-appearance (Change Delta)

## ADDED Requirements

### Requirement: Adjustable window opacity
The main window MUST support user-adjustable opacity from 0–100% within a safe bound that keeps the app visible while staying always-on-top.

#### Scenario: Change opacity from Settings
- Given the app window is visible
- When the user sets opacity to a new value within the allowed range from the Settings modal
- Then the window updates to that opacity immediately while remaining interactive and always on top

### Requirement: Optional background blur
The main window MUST support an optional blur effect applied to its background when supported by the host platform. On Windows, blur MUST be disabled if the platform API differs from macOS and would degrade behavior.

#### Scenario: Enable blur on supported platforms
- Given the platform supports rendering a blurred/translucent window background
- When the user enables blur in Settings
- Then the window shows a blur effect behind the app content
- And disabling blur restores the normal background

#### Scenario: Handle unsupported blur gracefully
- Given the platform does not support the blur effect
- When the user opens Settings
- Then the blur control is disabled or clearly marked unavailable
- And leaving it unchanged does not affect other settings
