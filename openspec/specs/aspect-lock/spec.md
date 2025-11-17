# aspect-lock Specification

## Purpose
TBD - created by archiving change add-settings-fit-window-and-aspect-lock. Update Purpose after archive.
## Requirements
### Requirement: Lock aspect ratio on resize
When enabled, the application MUST constrain window resizing to maintain the image’s aspect ratio.

#### Scenario: Aspect lock enabled
- Given the Lock aspect ratio on resize setting is enabled
- And an image is currently displayed
- When the user resizes the window
- Then the window’s width and height change in proportion to preserve the image’s aspect ratio

#### Scenario: Aspect lock disabled
- Given the Lock aspect ratio on resize setting is disabled
- When the user resizes the window
- Then the window resizes freely without preserving aspect ratio

