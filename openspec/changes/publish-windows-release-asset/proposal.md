## Why
- The release pipeline already builds a Windows NSIS installer in CI, but it is only retained as an internal workflow artifact.
- Public releases currently ship only macOS assets, which leaves the Windows build path undiscoverable and inconsistent with the automated release intent.

## What Changes
- Publish a stable Windows installer asset to each tagged GitHub Release alongside the macOS DMG and checksum.
- Standardize the public Windows asset name so downstream docs and release consumers can rely on it.
- Update release documentation to distinguish between the macOS landing-page flow and the broader GitHub Release asset set.

## Impact
- Specs: `release-automation`, `project-docs`
- CI: `.github/workflows/release-bundles.yml`
- Docs: `README.md`, `docs/releasing.md`
