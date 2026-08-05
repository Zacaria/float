# release-automation Specification

## Purpose
TBD - created by archiving change add-release-plz-automation. Update Purpose after archive.
## Requirements
### Requirement: Automated tagging and releases via release-plz
The project MUST use release-plz to create git tags with prefix `v` and publish GitHub Releases from versions and changelog entries prepared in the repository. The pipeline MUST avoid publishing to crates.io and run via GitHub Actions.

#### Scenario: Release created from main
- Given a version and changelog entry are prepared on the default branch
- When the release automation workflow runs
- Then release-plz creates the matching `v*` tag and a GitHub Release for the previously unreleased prepared version

#### Scenario: No crates.io publish
- Given release-plz runs in CI
- When the workflow completes
- Then no crate is published to crates.io (publish is disabled), but the GitHub tag and Release exist

#### Scenario: Artifacts attached by bundling workflow
- Given a Release exists for tag `v*`
- When the release workflow dispatches the bundling workflow at that tag
- Then the GitHub Release contains `Float-macos-universal.dmg`, `Float-macos-universal.sha256`, and `Float-windows-x64-setup.exe`
