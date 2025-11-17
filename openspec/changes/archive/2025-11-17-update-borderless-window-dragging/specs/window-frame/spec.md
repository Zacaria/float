## ADDED Requirements
### Requirement: Borderless draggable image frame
The application MUST present the main window without OS window decorations while remaining resizable, and the image/placeholder area MUST function as the drag region so the window can be moved.

#### Scenario: Drag window via image or placeholder
- Given the window is borderless and an image or placeholder is visible
- When the user clicks and drags anywhere on the displayed frame (excluding overlay controls)
- Then the window moves with the drag

#### Scenario: Controls remain interactive
- Given overlay controls (HUD text, Previous/Next buttons) appear over the borderless window
- When the user clicks those controls
- Then the controls respond normally and do not move the window
