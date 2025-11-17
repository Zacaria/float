1. Initialize Tauri in repo (or new `src-tauri`) preserving Rust crate metadata
2. Create Tauri window configured always-on-top on macOS and Windows
3. Implement native file open via Tauri dialog; update title with file name
4. Frontend: display selected image and scale to fit; add manual Fit Now action
5. Persist window size and settings (fit/aspect) via Tauri store or filesystem
6. Add menu entries and shortcuts (macOS Cmd+F, Windows Ctrl+F); keep Quick Look macOS-only
7. Replace `cargo-bundle` flow with `tauri build`; document build commands
8. Validate on macOS & Windows VMs: launch, open file, fit now, persistence
9. Update README and Justfile targets for Tauri build
10. Align OpenSpec: validate specs and resolve issues

