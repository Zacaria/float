---
title: Deterministic macOS Feedback Loop for Tauri Window Flows
date: 2026-04-07
category: developer-experience
module: Tauri desktop shell
problem_type: developer_experience
component: tooling
severity: medium
applies_when:
  - You need to verify native macOS menu-driven behavior in the Tauri desktop shell.
  - Browser-only tooling cannot exercise the real app window lifecycle.
  - A bug depends on multiple Float windows or native app-menu actions.
symptoms:
  - Manual reproduction loops are too slow to trust while iterating on multi-window bugs.
  - Browser and mocked UI tests pass, but native desktop behavior is still unclear.
root_cause: missing_tooling
resolution_type: tooling_addition
tags:
  - tauri
  - macos
  - accessibility
  - desktop-automation
  - feedback-loop
  - window-targeting
---

# Deterministic macOS Feedback Loop for Tauri Window Flows

## Context
The repo needed a self-serve validation loop for a native macOS Tauri bug where `New Window` and `Open…` behavior had to be checked against real app windows, not just mocked frontend state. `tauri-driver` was not a practical answer on this machine, so the fix needed to use tooling that could drive the actual desktop app.

## Guidance
Use a macOS-specific automation harness that launches Tauri dev with a deterministic file path, drives the real app menu through Accessibility, and asserts against actual Float window titles.

The working pieces are:

- [scripts/macos-open-target-check.sh](/Users/zacariachtatar/repos/always-on-top/scripts/macos-open-target-check.sh)
- [justfile](/Users/zacariachtatar/repos/always-on-top/justfile)

The harness relies on `FLOAT_TEST_PATH` so `Open…` can use a known image without going through the Finder chooser:

```bash
just tauri-check-open-target src-tauri/icons/icon.png
```

The stable flow was:

1. Kill stale `float-tauri` or `Float` dev instances before starting a new run.
2. Launch `cargo tauri dev` with `FLOAT_TEST_PATH` set to an absolute image path.
3. Drive the app through `System Events` with Accessibility enabled.
4. Use the real app menu items under the `Float` menu, not guessed shortcuts or a `File` menu path.
5. Assert against actual window titles like `Float — icon.png` and `Float`.

The critical implementation details were:

- use the dev-process name `float-tauri` for automation
- use menu clicks instead of shortcut delivery for reliability
- target the `Float` menu in the accessibility tree
- clean up stale dev instances before each run

## Why This Matters
Without a deterministic native loop, every iteration on desktop-only behavior depends on human validation, which makes regressions harder to isolate and makes bug-fixing slower than it needs to be. The harness turns a subjective “try it again” cycle into a concrete pass/fail command that checks the real Tauri shell instead of only the mocked webview layer.

## When to Apply
- When a bug depends on native app menus, multiple windows, or macOS accessibility behavior.
- When mocked Playwright tests are green but desktop behavior is still in doubt.
- When `Cmd+T`, `Cmd+O`, or similar app-menu flows need to be verified repeatedly during iteration.
- When the Finder chooser would make automation too brittle, but `FLOAT_TEST_PATH` can bypass it safely.

## Examples
Before this harness, the team had to reproduce the window-targeting flow by hand and infer behavior from screenshots, manual notes, or temporary logging.

After the harness, the verification loop became:

```bash
just tauri-check-open-target src-tauri/icons/icon.png
```

Expected passing output:

```text
Open-target check passed.
Front window: Float — icon.png
All window titles:
  - Float — icon.png
  - Float
```

Key failure modes discovered while building the harness:

- `System Events` is blocked until macOS Accessibility permission is granted to the host app running `osascript`.
- The dev process is exposed as `float-tauri`, not `Float`.
- The app menu is exposed as `Float`, not `File`.
- Keyboard shortcut delivery was less reliable than clicking the menu items directly.

## Related
- No existing `docs/solutions/` entries covered this repo yet.
- No related GitHub issues were returned for the search terms used during documentation.
