1. Initialize Rust binary crate structure
2. Add dependencies for windowing and file dialog
3. Create window with always-on-top enabled
4. Trigger native file dialog at startup
5. Update window title with selected file name
6. Handle cancel path and close request
7. Add macOS menubar (File->Open, View->Quick Look) and wire actions
8. Implement Quick Look invocation via `qlmanage -p`
9. Add keyboard shortcuts (Cmd+O, Cmd+Y) through menu accelerators
10. Add bundle metadata and document `.app` packaging
