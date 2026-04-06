# window-appearance (Change Delta)

## ADDED Requirements

### Requirement: Adjustable window opacity
The main window MUST support user-adjustable opacity from a bounded visible range that keeps the app usable while staying always on top. This control MUST affect the native window transparency rather than only dimming the app's own rendered content.

#### Scenario: Change opacity from Settings
- Given the app window is visible
- When the user changes the opacity control from the Settings surface
- Then the window updates to the new opacity immediately
- And the content behind the Float window becomes correspondingly visible
- And the window remains visible and interactive

#### Scenario: Missing opacity preference uses a safe default
- Given there is no persisted opacity value
- When the app starts
- Then the window uses a fully visible default opacity

### Requirement: Optional background blur
The main window MUST support an optional blur effect when the host platform can apply it safely. If the host platform does not support blur for this shell, the Settings surface MUST present that limitation gracefully without blocking other settings.

#### Scenario: Enable blur on a supported platform
- Given the platform supports the blur effect for the active shell
- When the user enables blur from Settings
- Then the window shows the blur effect behind the app content
- And disabling blur restores the normal background

#### Scenario: Unsupported blur degrades gracefully
- Given the platform does not support blur for the active shell
- When the user opens Settings
- Then the blur control is disabled or clearly marked unavailable
- And changing other settings still works normally
