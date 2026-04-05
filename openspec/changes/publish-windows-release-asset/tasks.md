## 1. Implementation
- [x] 1.1 Update the release workflow so tagged builds publish a stable Windows installer asset to GitHub Releases alongside the macOS assets.
- [x] 1.2 Document the public Windows release asset name and clarify that the GitHub Pages landing page remains macOS-only.
- [x] 1.3 Update the release automation and project docs spec deltas to reflect the public Windows release asset.

## 2. Validation
- [x] 2.1 Run `openspec validate publish-windows-release-asset --strict`.
- [ ] 2.2 Trigger a tagged release build and verify the resulting GitHub Release contains the macOS DMG, macOS checksum, and Windows installer asset.
