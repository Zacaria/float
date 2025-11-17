# Proposal: Add public README and release distribution docs

## Change ID
add-public-readme-and-release-docs

## Why
- The current README is terse and doesn’t help new users install, run, or download builds across macOS/Windows/Linux.
- There’s no documented release pipeline, download links, or feature overview to share with others.

## What Changes
- Author a comprehensive README covering features, platform support, install/run instructions, and troubleshooting.
- Document the build/release pipeline (Tauri bundles, just tasks, expected output locations) and provide download guidance/placeholders for macOS, Windows, and Linux.
- Clarify platform nuances (Quick Look macOS only, Linux preview TBD) and link to OpenSpec capabilities for source of truth.

## Scope
- Documentation only (README and supporting references). No code or behavior changes.

## Out of Scope
- New features or platform enablement beyond documentation.
- Automated release infra; we’ll describe the manual flow with just/tauri.

## Open Questions
- None (resolved): GitHub Actions builds the binaries and uploads them to GitHub Releases for macOS/Windows.
