# development-workflow (Change Delta)

## ADDED Requirements

### Requirement: Conventional Commits are enforced for new commits
The repository MUST reject new commits whose subject lines do not follow Conventional Commits. The same validation logic MUST be available in local git hooks and in GitHub Actions so contributors and CI enforce the same rule consistently.

#### Scenario: Local commit blocked by hook
- Given a contributor has installed the repository git hooks
- When they attempt to create a commit with the subject `Rename app to Float`
- Then the `commit-msg` hook rejects the commit
- And it shows the expected Conventional Commit format

#### Scenario: Pull request commits validated in CI
- Given a pull request contains one or more non-merge commits
- When the Conventional Commits workflow runs
- Then CI validates only the new commit subjects in the pull request
- And the job fails if any of those subjects do not match the allowed Conventional Commit pattern

#### Scenario: Manual range validation
- Given a contributor wants to validate recent commits before pushing
- When they run the documented commit-range validation command
- Then the repository checks that range with the same Conventional Commit rules used by the hook and CI
