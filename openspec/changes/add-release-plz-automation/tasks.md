## 1. Automation
- [x] 1.1 Add release-plz configuration (changelog path, tag prefix, disable crate publish) and seed CHANGELOG.md.
- [x] 1.2 Add GitHub Actions workflow to run release-plz on main/tags (or manual dispatch) to create tags, changelog entries, and GitHub Releases.
- [x] 1.3 Ensure README references the automated release flow and how it pairs with the bundling workflow publishing artifacts.

## 2. Validation
- [x] 2.1 `openspec validate add-release-plz-automation --strict`.
- [x] 2.2 Sanity check workflow syntax and paths; mention required secrets/tokens if any.
