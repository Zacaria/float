## Context

The active app shell is Tauri-based and already uses a single webview UI in `dist/index.html`. Persisted settings are owned by `src-tauri/src/main.rs`, but there is no current Tauri Settings panel despite the existing spec. The only prior Settings UI in the repo is legacy macOS-only code in `src/main.rs`, which is not the active shell.

The first implementation pass proved that an inline settings modal is the wrong product shape. It covers the active image, competes with the viewer, and encourages shared-state shortcuts that do not hold up in multi-window use.

## Goals

- Restore a coherent Settings experience in the active Tauri app.
- Keep Settings out of the image viewport by using a dedicated separate window.
- Keep the experience minimal and polished, not heavyweight.
- Make the Settings contract consistent with the current manual-only Fit behavior.
- Support appearance controls only where the active shell can apply them safely and truthfully.
- Keep viewer state isolated per window.

## Non-Goals

- Reintroducing a persisted auto-fit toggle.
- Expanding v2 into a broader convenience-feature release.
- Sharing active viewer state through the settings payload.

## Decisions

- Use a dedicated Tauri settings window opened from the native menu / shortcut so configuration is separate from the viewer.
- Separate Behavior and Appearance so durable viewer controls and capability-gated native effects remain easy to scan.
- Treat durable preferences and session controls differently: Settings owns persistent viewer preferences, while session-state playback controls remain outside the core Settings contract unless later work intentionally moves them.
- Treat active image selection and slideshow playback state as window-scoped viewer state, not settings state.
- Implement opacity through native window transparency rather than CSS opacity on viewer content.
- Gate blur support by capability so unsupported platforms degrade safely without blocking the rest of the Settings surface.
