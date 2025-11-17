## Tasks
1. [ ] Map current single-window flow (menu wiring, window state restoration, persistence) and identify gaps for multiple window instances and focus handling.
2. [ ] Define shortcut behavior per platform (Cmd/Ctrl variants) and how new windows inherit settings (always-on-top, fit/aspect toggles) while keeping Cmd/Ctrl+O scoped to the focused window.
3. [ ] Draft spec deltas for menu shortcuts and window lifecycle (new window creation, closing focused window, close-all/quit) including edge cases like last window close.
4. [ ] Validate the change with `openspec validate add-window-shortcuts --strict` and capture any open questions about Windows parity or window persistence.
