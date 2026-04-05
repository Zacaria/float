# Proposal: Add conventional commits guard

## Change ID
add-conventional-commit-guard

## Why
- Release automation and changelog generation work best when commit subjects follow Conventional Commits.
- The repository currently treats this as an implicit convention, so non-conforming commit messages can land without any guardrail.

## What Changes
- Add a shared validator script that can check either a `commit-msg` hook payload or a git revision range.
- Add a repository-managed `commit-msg` hook and a GitHub Actions workflow that both use the same validator.
- Document local hook installation and manual validation commands for contributors.

## Scope
- Repository workflow automation and contributing documentation.

## Out of Scope
- Rewriting existing commit history.
- Enforcing commit bodies, footers, or release notes format beyond the subject line.
