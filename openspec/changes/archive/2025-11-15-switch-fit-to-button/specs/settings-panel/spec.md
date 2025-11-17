## MODIFIED Requirements

### Requirement: Settings panel
The Settings panel MUST include a button labeled “Fit Window to Image Now” which triggers an immediate fit-to-image action when pressed. The previous automatic toggle for fit-on-selection is removed.

#### Scenario: Fit Now button
- Given an image is currently displayed
- When the user opens Settings and presses “Fit Window to Image Now”
- Then the window resizes to the image’s dimensions (clamped to the visible screen)

