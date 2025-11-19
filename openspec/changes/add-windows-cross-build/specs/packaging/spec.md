## MODIFIED Requirements
### Requirement: Package as a macOS app bundle
The project MUST build distributable artifacts for macOS and Windows using the Tauri bundler, producing a macOS app bundle and a Windows NSIS installer. macOS contributors MUST also be able to cross-build the Windows release executable for quick validation.

#### Scenario: Build macOS app bundle
- Given the Rust and Tauri toolchains are installed
- When the developer runs `tauri build` (or `cargo tauri build`)
- Then a macOS `.app` is produced under `src-tauri/target/release/bundle/macos/` or equivalent

#### Scenario: Build Windows installer (NSIS)
- Given the Rust and Tauri toolchains are installed on Windows
- When the developer runs `tauri build`
- Then a Windows NSIS installer (`.exe`) is produced under `src-tauri/target/release/bundle/nsis/`

#### Scenario: Cross-build Windows executable from macOS
- Given a macOS developer installed `cargo-xwin`, added the `x86_64-pc-windows-msvc` Rust target, and has the repo dependencies installed
- When they run `just tauri-build-windows`
- Then a Windows release executable is generated at `src-tauri/target/x86_64-pc-windows-msvc/release/Float.exe` for testing or sharing
