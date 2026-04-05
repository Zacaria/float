## MODIFIED Requirements
### Requirement: README covers onboarding and distribution
The repository MUST include a README that describes what the app does, supported platforms, current features, installation/prerequisites, development and build commands, and guidance for obtaining macOS/Windows binaries with notes for Linux status.

#### Scenario: Release pipeline documented
- Given a contributor preparing a release
- When they follow the README’s release/bundle section
- Then they understand that tagged GitHub Releases publish `Float-macos-universal.dmg`, `Float-macos-universal.sha256`, and `Float-windows-x64-setup.exe`, while the GitHub Pages landing page remains macOS-only
