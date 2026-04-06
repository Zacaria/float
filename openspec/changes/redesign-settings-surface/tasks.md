## 1. Spec Reconciliation

- [x] 1.1 Update `settings-panel` to define the Tauri Settings entrypoint, General / Shortcuts tabs, and the minimal polished scope for persistent viewer preferences.
- [x] 1.2 Update `settings-persistence` and `fit-window` to remove stale persisted auto-fit expectations and clarify which values remain durable preferences.
- [x] 1.3 Update `menu-and-shortcuts` to add the Settings shortcut/entrypoint and remove the obsolete Auto-fit toggle contract.
- [x] 1.4 Add a `window-appearance` capability covering opacity and blur with graceful unsupported-platform behavior.

## 2. Tauri Settings Surface

- [ ] 2.1 Add a native Settings menu item / shortcut in the Tauri shell that opens or focuses a dedicated settings window.
- [ ] 2.2 Implement a lightweight settings window with General and Shortcuts tabs, separate from the viewer viewport.
- [ ] 2.3 Keep the settings window visually polished but minimal, with stronger grouping, labels, and hierarchy than the current functional baseline.

## 3. Settings State And Appearance

- [x] 3.1 Align the persisted settings model with the reconciled contract and keep menu state and Settings state synchronized.
- [ ] 3.2 Implement real native opacity editing for supported platforms, plus safe bounds and unsupported-platform fallback for blur.
- [x] 3.3 Preserve manual Fit as an action-only command rather than reintroducing a persisted fit toggle.
- [ ] 3.4 Isolate active image state per viewer window so one window cannot overwrite another.

## 4. Validation

- [x] 4.1 Add or update frontend and Tauri smoke coverage for opening Settings, rendering current values, and persisting supported controls.
- [x] 4.2 Run `openspec validate redesign-settings-surface --strict`.
- [ ] 4.3 Manually verify macOS and Windows behavior for dedicated Settings entry, shortcut labels, real-opacity behavior, appearance control fallback, per-window image isolation, and restart persistence.
