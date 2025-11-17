## Why
- The placeholder currently says “No file selected” and doesn’t guide users toward the keyboard shortcut to open an image.
- Showing the platform-appropriate shortcut (⌘+O on macOS, Ctrl+O on Windows) makes the empty state actionable and discoverable.

## What Changes
- Update the empty-state placeholder text to invite opening a file with the correct platform shortcut.
- Capture this behavior in the display-image capability spec.

## Impact
- Specs: display-image placeholder requirement updated.
- Code: frontend placeholder text/logic updated to show platform-specific shortcut hint.
