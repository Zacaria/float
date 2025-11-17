# quick-look (Change Delta)

## REMOVED Requirements

### Requirement: Quick Look preview
The application MUST provide a Quick Look preview of the selected file on macOS. Windows has no Quick Look capability.

#### Scenario: Quick Look command no longer available
- Given the app is running on macOS or Windows
- When the user looks for the Quick Look menu item or shortcut (Cmd+Y)
- Then no Quick Look option is available
- And triggering the old shortcut does nothing visible
