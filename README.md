# Always On Top (macOS, Rust)

A minimal macOS utility written in Rust. It opens a native file picker and keeps its window always on top. The selected file name is shown in the window title.

## Build

Requirements:
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

## Bundle as .app

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
