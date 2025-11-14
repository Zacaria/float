# Decisions for `create-macos-keep-on-top-app`

Date: 2025-11-14

- Quick Look preview: Yes — provide a system Quick Look panel for the selected file.
- Menu item + keyboard shortcut: Yes — add File → Open… (Cmd+O) and View → Quick Look (Cmd+Y).
- Package as an app: Yes — provide `.app` bundle via `cargo-bundle` with metadata in `Cargo.toml`.

Implications:
- Specs for Quick Look, menu/shortcuts, and packaging are in-scope and tracked under `specs/`.
- Code and README updated to support these capabilities.

