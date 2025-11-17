## Tasks
1. [ ] Map current single-file flow (dialog options, state updates, front-end display) and outline what changes to track multiple selections and an active index.
2. [ ] Update the backend and IPC contract to accept multiple selected file paths, track the ordered set plus the active file, and reuse auto-fit/aspect-lock/title logic for each navigation change.
3. [ ] Add user-facing controls (menu entries and shortcuts, plus any minimal frontend affordance) to move to the next/previous file in the current selection without reopening the dialog.
4. [ ] Validate manually: multi-select a set and navigate through it (including boundaries), confirm titles/auto-fit/aspect-lock/persistence reflect the active file, and run `openspec validate add-multi-file-open --strict`.
