## Why

The active Tauri app persists viewer settings and exposes some toggles in the native menu, but it does not currently provide the user-facing Settings surface described in OpenSpec. At the same time, the current specs still contain stale references to a persisted auto-fit toggle even though Fit is now manual-only.

This leaves the product with two problems:
- the app feels unfinished because durable preferences have no polished home
- the current contract is internally inconsistent, which makes implementation and review ambiguous

Recent product feedback sharpens that scope:
- slideshow playback needs explicit timing configuration to feel complete
- opacity needs to be user-configurable
- both belong in a dedicated clean configuration panel rather than in scattered runtime affordances

Live app validation adds three corrections:
- Settings cannot live inside the image viewer viewport because they cover the active image and break the product feel.
- Viewer state must stay isolated per Float window; opening an image in one window must not replace the image in another.
- Opacity must be real native window transparency, not CSS-only dimming of the viewer content.

## What Changes

- Add a polished Settings surface to the active Tauri shell, reachable from the native menu and `Cmd/Ctrl+,`, as a dedicated separate window.
- Present a lightweight General / Shortcuts settings experience that matches the current Tauri product shape without covering the viewer.
- Make the Settings surface the dedicated clean home for slideshow timing and opacity controls.
- Reconcile spec drift by removing stale auto-fit toggle expectations from Settings and menu contracts.
- Add explicit window appearance requirements for real native opacity and optional blur with graceful platform fallback.
- Correct state ownership so viewer file selection remains window-scoped while durable preferences remain shared.
- Keep the redesign minimal and product-focused rather than expanding into a larger preferences system.

## Impact

- Affected specs: `settings-panel`, `settings-persistence`, `menu-and-shortcuts`, `fit-window`, `window-appearance`
- Affected code: `src-tauri/src/main.rs`, `dist/index.html`, `tests/ui-mock.spec.ts`, `tests/tauri-driver.spec.ts`
