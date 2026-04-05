# packaging (Change Delta)

## MODIFIED Requirements

### Requirement: Package as a macOS app bundle
The project MUST build distributable artifacts for macOS and Windows using the Tauri bundler, producing a macOS app bundle and a Windows NSIS installer, and the README MUST document the commands and output locations for these bundles plus the current Linux support status (manual dev run only, no packaged binary yet). GitHub Actions MUST build and upload these artifacts to GitHub Releases for distribution.

#### Scenario: Build macOS app bundle
- Given the Rust and Tauri toolchains are installed
- When the developer runs the documented bundle command from the README (e.g., `just tauri-build`)
- Then a macOS `.app` is produced under `src-tauri/target/release/bundle/macos/` or equivalent and the README reflects this location

#### Scenario: Build Windows installer (NSIS)
- Given the Rust and Tauri toolchains are installed on Windows
- When the developer runs the documented bundle command from the README
- Then a Windows NSIS installer (`.exe`) is produced under `src-tauri/target/release/bundle/nsis/` and the README reflects this location

#### Scenario: Linux support documented
- Given a Linux user reads the README
- When they review install and download notes
- Then it states that Linux builds are not yet provided and directs them to run the dev server/binary locally if they wish to experiment

#### Scenario: CI artifacts published
- Given a release workflow runs on GitHub Actions
- When a release is built
- Then macOS and Windows artifacts are uploaded to the GitHub Releases page for download
