## 1. Enforcement
- [x] 1.1 Add a shared Conventional Commits validator script for commit messages and commit ranges.
- [x] 1.2 Add a repository `commit-msg` hook plus a documented install step so contributors can enable the guard locally.
- [x] 1.3 Add a GitHub Actions workflow that validates new pull request and default-branch push commits with the same rule set.

## 2. Documentation And Validation
- [x] 2.1 Document the Conventional Commits requirement and local commands in `README.md` and `justfile`.
- [x] 2.2 `just check-commits e3d9304^!`
- [x] 2.3 `openspec validate add-conventional-commit-guard --strict`
