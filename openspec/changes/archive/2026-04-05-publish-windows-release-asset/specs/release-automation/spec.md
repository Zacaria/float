## MODIFIED Requirements
### Requirement: Automated tagging and releases via release-plz
The project MUST use release-plz to generate changelog entries, create git tags with prefix `v`, and publish GitHub Releases. The pipeline MUST avoid publishing to crates.io and run via GitHub Actions.

#### Scenario: Artifacts attached by bundling workflow
- Given a Release exists for tag `v*`
- When the bundling workflow runs on that tag
- Then the GitHub Release contains `Float-macos-universal.dmg`, `Float-macos-universal.sha256`, and `Float-windows-x64-setup.exe`
