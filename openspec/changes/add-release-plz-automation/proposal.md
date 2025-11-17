# Proposal: Add release-plz automation for tagging and releases

## Change ID
add-release-plz-automation

## Why
- We want tags, changelog updates, and GitHub Releases to be created automatically without manual tagging.
- Using release-plz keeps versioning and changelog generation consistent across macOS/Windows builds and aligns with existing Tauri bundle workflow on tags.

## What Changes
- Add release-plz configuration (changelog path, tag prefix) and initialize `CHANGELOG.md`.
- Add a GitHub Actions workflow that runs release-plz on main/tag triggers to generate the changelog, bump version, create tags, and publish GitHub Releases (without publishing to crates.io).
- Ensure the release-bundles workflow consumes those tags to upload macOS/Windows artifacts to the corresponding Release.

## Scope
- CI automation and documentation updates.

## Out of Scope
- Publishing to crates.io.
- Changing runtime behavior or packaging outputs.

## Open Questions
- None.
