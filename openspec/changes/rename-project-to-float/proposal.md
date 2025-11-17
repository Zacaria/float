# Proposal: Rename project to Float

## Change ID
rename-project-to-float

## Why
- The app and docs are branded "Always On Top" today; the request is to rename the entire project to "Float".
- Without a scoped plan, the rename could miss UI strings, bundle names, or persisted identifiers, leaving a confusing mix of names for users and builders.

## What Changes
- Rebrand the application to "Float" across UI strings (window titles, menus, dialogs) and native metadata.
- Update bundler config and outputs (app bundle/installer names, product metadata) plus documentation and release instructions to use the Float name and paths.
- Adjust identifiers and persisted paths as needed to align with the new name while preserving existing user data.

## Scope
- Naming/branding changes only; functionality remains the same.

## Out of Scope
- New features or behavior changes unrelated to the rename.
- Platform support changes.

## Open Questions
- Resolved: use the `com.havesomecode` namespace for identifiers/settings; migrate prior data into the Float-branded path.
