## Tasks
1. [ ] Map current Quick Look and app-provided full-screen affordances (menus, shortcuts, commands, window config) to confirm what is exposed per platform and where behavior exists/no-ops.
2. [ ] Draft spec deltas removing Quick Look (capability removal plus menu/settings references) and defining the new windowed-only constraint (no app-provided full-screen entry points).
3. [ ] Validate the change with `openspec validate remove-quick-look-and-fullscreen --strict` and call out any open questions (e.g., OS-level shortcut suppression vs. menu visibility).
