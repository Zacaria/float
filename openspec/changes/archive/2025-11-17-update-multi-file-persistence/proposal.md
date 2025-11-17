## Why
- The app already plans multi-window support, but persisted state only tracks a single last file path, so multiple open files are lost on restart.
- Restoring the previous working set across launches improves continuity and aligns with anticipated multi-file flows.

## What Changes
- Extend persisted state to store all open files (and their order) at shutdown instead of a single path.
- Restore previously open files on startup, ignoring entries that no longer exist.
- Keep behavior backward compatible for single-window cases.

## Impact
- Affected specs: last-file-persistence (upgrade to multiple entries; restore rules). Window-size may need follow-on adjustments per-window in a later change once multi-window sizing is defined.
- Affected code: legacy macOS crate (`src/main.rs`) and Tauri shell (`src-tauri/src/main.rs`) persistence/load routines and window bootstrapping.
- Validation: ensure clippy/tests still pass and manual startup restores multiple files when present, falling back gracefully if entries are missing.
