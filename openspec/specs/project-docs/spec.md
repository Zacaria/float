# project-docs Specification

## Purpose
TBD - created by archiving change add-public-readme-and-release-docs. Update Purpose after archive.
## Requirements
### Requirement: README covers onboarding and distribution
The repository MUST include a README that describes what the app does, supported platforms, current features, installation/prerequisites, development and build commands, and guidance for obtaining macOS/Windows binaries with notes for Linux status.

#### Scenario: New user can install and run
- Given a new developer or user reads the README
- When they follow the install and run instructions for their platform (macOS, Windows, Linux noted)
- Then they can build or obtain the app and launch it with the described commands

#### Scenario: Release pipeline documented
- Given a contributor preparing a release
- When they follow the README’s release/bundle section
- Then they understand that tagged GitHub Releases publish `Float-macos-universal.dmg`, `Float-macos-universal.sha256`, and `Float-windows-x64-setup.exe`, while the GitHub Pages landing page remains macOS-only

#### Scenario: Platform nuances documented
- Given a user on macOS or Windows
- When they read the feature/platform notes
- Then they see which features are platform-specific (e.g., Quick Look macOS-only) and links to OpenSpec capabilities for details
