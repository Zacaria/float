## Tasks
1. [ ] Review current persistence artifacts (settings.json contents, save/load flows) and menu/shortcut coverage to see where a reset hook should plug in.
2. [ ] Draft spec deltas: add menu/shortcut for Reset Cache, define what state is cleared (settings, last files, window sizes), and the behavior (all windows close, next start clean).
3. [ ] Validate with `openspec validate add-reset-cache-action --strict`; note any open questions (e.g., confirmation prompts, platform shortcut choice) if needed.
