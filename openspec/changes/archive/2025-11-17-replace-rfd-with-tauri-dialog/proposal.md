## Why
The app currently relies on the `rfd` crate for file selection. Tauri provides a built-in dialog plugin that offers native pickers without extra dependencies. Switching to the plugin reduces surface area while keeping the native file selection behavior the specs require.

## What Changes
- Replace all file-picking calls that use `rfd` with Tauri’s dialog plugin.
- Register the dialog plugin in the Tauri shell and remove the `rfd` dependency from the project.
- Keep the existing behaviors (record selected path, update window title, preserve cancel handling) intact while using the plugin.

## Impact
- Affected specs: file-selection.
- Affected code: `src-tauri` commands that pick files, any front-end code that triggers selection, dependency manifests (`Cargo.toml`/lockfiles) and plugin setup.
