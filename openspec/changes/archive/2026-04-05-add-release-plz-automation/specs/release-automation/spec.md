# release-automation (Change Delta)

## ADDED Requirements

### Requirement: Automated tagging and releases via release-plz
The project MUST use release-plz to generate changelog entries, create git tags with prefix `v`, and publish GitHub Releases. The pipeline MUST avoid publishing to crates.io and run via GitHub Actions.

#### Scenario: Release created from main
- Given changes are merged to the default branch
- When the release automation workflow runs
- Then release-plz updates `CHANGELOG.md`, bumps versions as needed, creates a `v*` tag, and creates/updates a GitHub Release

#### Scenario: No crates.io publish
- Given release-plz runs in CI
- When the workflow completes
- Then no crate is published to crates.io (publish is disabled), but the GitHub tag and Release exist

#### Scenario: Artifacts attached by bundling workflow
- Given a Release exists for tag `v*`
- When the bundling workflow runs on that tag
- Then macOS and Windows artifacts are uploaded to the same GitHub Release
