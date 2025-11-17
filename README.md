# Always On Top (macOS/Windows via Tauri)

A minimal utility written in Rust using Tauri for cross‑platform packaging. It opens a native file picker and keeps its window always on top. The selected file name is shown in the window title.

## Build

Requirements (legacy winit app):
- macOS
- Rust toolchain (`rustup`, `cargo`)

Steps:

```
cargo run --release
```

Notes:
- The window launches as always-on-top by default.
- On launch, a native file dialog lets you pick a file. If you cancel, the app stays open.
- The selected file name appears in the window title.
- macOS menu includes File → Open… (Cmd+O) and View → Quick Look (Cmd+Y).
- Quick Look uses the system `qlmanage` tool to present a preview.

## Tauri (macOS + Windows)

This repo includes a Tauri shell under `src-tauri/` with a minimal static frontend in `dist/` to enable Windows distribution (NSIS) and cross‑platform packaging while preserving core behavior.

Requirements:
- Rust toolchain (`rustup`, `cargo`)
- Tauri CLI (`cargo install tauri-cli`) and platform prerequisites (see Tauri docs)
- macOS or Windows (for platform‑specific bundles)

Dev run:

```
just tauri-dev
```

Build bundles:

```
just tauri-build
```

- macOS: `.app` under `src-tauri/target/release/bundle/macos/Always On Top.app`
- Windows: NSIS installer `.exe` under `src-tauri/target/release/bundle/nsis/`

Open the built macOS app:

```
just tauri-open
```

Notes:
- Always‑on‑top is enabled by default on both macOS and Windows.
- File → Open… (Cmd/Ctrl+O) and View → Fit to Image Now (Cmd/Ctrl+F) are available via menu and shortcuts.
- Quick Look is macOS‑only; on Windows it is a no‑op.

## Bundle as .app (legacy)

Install bundler:

```
cargo install cargo-bundle
```

Build app bundle:

```
cargo bundle --release
```

The `.app` will appear under `target/release/bundle/osx/Always On Top.app`.

## Future Enhancements
- Toggle always-on-top from a menu or shortcut
- Embed Quick Look view instead of external panel
- App icon and signing/notarization
- Embedded preview with zoom/pan (see OpenSpec change `add-embedded-preview-ui`)
