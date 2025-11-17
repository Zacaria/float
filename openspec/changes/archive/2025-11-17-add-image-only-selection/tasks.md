## Tasks
1. [ ] Identify current selection and loading paths (multi-file selection, navigation) and list supported image extensions.
2. [ ] Add filtering/validation so native file picks and persisted paths only admit allowed image types; handle invalid selections with a user-visible fallback.
3. [ ] Confirm title/auto-fit/persistence and navigation behavior remains correct with filtered selections; adjust frontend messaging if needed.
4. [ ] Validate with `openspec validate add-image-only-selection --strict` and run `cargo tauri build` (or at least `cargo check`) after changes.
