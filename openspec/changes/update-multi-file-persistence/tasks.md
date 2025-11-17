## Tasks
1. [ ] Map current persistence shape in legacy and Tauri shells (single `last_file` entry) and how startup restoration behaves with/without files.
2. [ ] Draft spec deltas to persist an ordered list of open files and restore them on startup, ignoring missing paths and keeping single-window behavior compatible.
3. [ ] Run `openspec validate update-multi-file-persistence --strict` and capture any open questions about per-window size/state for future follow-up.
