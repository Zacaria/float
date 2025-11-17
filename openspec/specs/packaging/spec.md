# packaging Specification

## Purpose
TBD - created by archiving change create-macos-keep-on-top-app. Update Purpose after archive.
## Requirements
### Requirement: Package as a macOS app bundle
The project MUST build distributable artifacts for macOS and Windows using the Tauri bundler, producing a macOS app bundle and a Windows NSIS installer.

#### Scenario: Build macOS app bundle
- Given the Rust and Tauri toolchains are installed
- When the developer runs `tauri build` (or `cargo tauri build`)
- Then a macOS `.app` is produced under `src-tauri/target/release/bundle/macos/` or equivalent

#### Scenario: Build Windows installer (NSIS)
- Given the Rust and Tauri toolchains are installed on Windows
- When the developer runs `tauri build`
- Then a Windows NSIS installer (`.exe`) is produced under `src-tauri/target/release/bundle/nsis/`

#### Notes
- Replace the existing `cargo-bundle` flow; update docs and Justfile.

