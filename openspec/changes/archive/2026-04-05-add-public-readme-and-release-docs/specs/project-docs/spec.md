# project-docs (Change Delta)

## ADDED Requirements

### Requirement: README covers onboarding and distribution
The repository MUST include a README that describes what the app does, supported platforms, current features, installation/prerequisites, development and build commands, and guidance for obtaining macOS/Windows binaries (built via GitHub Actions and published on GitHub Releases) with notes for Linux status.

#### Scenario: New user can install and run
- Given a new developer or user reads the README
- When they follow the install and run instructions for their platform (macOS, Windows, Linux noted)
- Then they can build or obtain the app and launch it with the described commands

#### Scenario: Release pipeline documented
- Given a contributor preparing a release
- When they follow the README’s release/bundle section
- Then they can produce platform artifacts using documented commands, know the expected output paths, and understand that GitHub Actions also builds and uploads binaries to Releases

#### Scenario: Platform nuances documented
- Given a user on macOS or Windows
- When they read the feature/platform notes
- Then they see which features are platform-specific (e.g., Quick Look macOS-only) and links to OpenSpec capabilities for details
