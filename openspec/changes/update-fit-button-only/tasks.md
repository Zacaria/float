## Tasks
1. [ ] Remove the persisted fit setting and related toggle UI; keep manual Fit action available via menu/shortcut.
2. [ ] Implement the new fit algorithm: anchor on the current window’s larger dimension, adjust the other dimension to match the image aspect ratio without upscaling, and clamp to reasonable minimums.
3. [ ] Update specs to reflect manual-only Fit and revised sizing behavior; ensure auto-fit on selection is removed.
4. [ ] Validate with `openspec validate update-fit-button-only --strict` and run `cargo tauri build` (or at least `cargo check`).
