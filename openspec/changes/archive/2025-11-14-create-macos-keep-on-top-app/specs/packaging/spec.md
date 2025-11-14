## ADDED Requirements

### Requirement: Package as a macOS app bundle
The project MUST build a signed .app bundle structure suitable for distribution on macOS (signing/notarization may be handled later).

#### Scenario: Build application bundle
- Given the Rust toolchain is installed
- And `cargo-bundle` is installed
- When the developer runs `cargo bundle --release`
- Then a macOS `.app` bundle is produced
- And it appears under `target/release/bundle/osx/Always On Top.app`

#### Notes
- Bundle metadata is specified under `Cargo.toml` `[package.metadata.bundle]`.
- App icon and signing/notarization are tracked as future enhancements.
