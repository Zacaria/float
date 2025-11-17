## 1. Branding updates
- [x] 1.1 Inventory all "Always On Top" references across code, config, docs, bundle outputs, and persisted paths.
- [x] 1.2 Update UI strings (window titles, menu labels, dialogs) to show "Float" everywhere.
- [x] 1.3 Update bundler metadata/productName and bundle output names to use "Float" on macOS and Windows.
- [x] 1.4 Decide and apply the Float bundle identifier/settings namespace; migrate or alias existing persisted settings if the namespace changes.

## 2. Documentation
- [x] 2.1 Refresh README/release notes and any package metadata to use the Float name and updated bundle paths/installers.
- [x] 2.2 Clean up any remaining references (just recipes, comments) that mention "Always On Top".

## 3. Validation
- [x] 3.1 `openspec validate rename-project-to-float --strict`.
- [ ] 3.2 Manual build check: run `just tauri-build` (or `tauri build`) and confirm bundle/installer names and app UI show "Float".
