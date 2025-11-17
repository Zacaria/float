## Overview
- Introduce multi-window support driven by standard shortcuts: new window with file (Cmd/Ctrl+T), close focused window (Cmd/Ctrl+W), close all and quit (Cmd/Ctrl+Q), keep Cmd/Ctrl+O scoped to the focused window.
- Each window hosts its own webview but reuses shared state for persisted settings (fit/aspect toggles) and persists the active file from the last focused window on quit.

## Considerations
- **Window creation**: decide whether to clone window size/state from the focused window or use defaults; always set always-on-top and apply persisted fit/aspect toggles.
- **Dialog parenting**: new window creation needs a dialog parent (focused window) when asking for a file path.
- **Persistence**: track only the most recently focused window’s active file at shutdown; restored on next launch if present.
- **Platform shortcuts**: prefer Cmd on macOS and mirror as Ctrl on Windows; behavior on other platforms is likely no-op or disabled.
- **Quit semantics**: Cmd/Ctrl+Q should close all windows and exit without leaving orphaned processes.
