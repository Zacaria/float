<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Legacy macOS app using `winit`/Cocoa (kept for reference).
- `src-tauri/`: Tauri v2 Rust shell (`Cargo.toml`, `src/main.rs`, `tauri.conf.json`).
- `dist/`: Minimal static frontend (HTML/JS/CSS) loaded by Tauri.
- `openspec/`: OpenSpec sources (`specs/` for current requirements, `changes/<id>/` for proposals, tasks, and deltas).
- `justfile`: Common dev/build tasks. Prefer `just` over ad‑hoc commands.

## Build, Test, and Development Commands
- Run (Tauri dev): `just tauri-dev`
- Build bundles: `just tauri-build`
- Open built macOS app: `just tauri-open`
- Legacy run (macOS winit): `just build-run`; bundle: `just bundle-run`
- OpenSpec validation: `openspec list`, `openspec validate <change-id> --strict`
- Lint/format (recommended): `cargo fmt`, `cargo clippy --all-targets -- -D warnings`

## Coding Style & Naming Conventions
- Rust 2021; prefer clear, small functions and explicit types.
- 4‑space indentation, no tabs. Avoid one‑letter names outside short scopes.
- Keep changes tightly scoped; update docs and OpenSpec when behavior changes.
- Paths and commands in docs use backticks; filenames use lowercase with hyphens.

## Testing Guidelines
- No automated tests today. Validate via OpenSpec scenarios and manual checks:
  - Open file → auto‑fit, title updates, always‑on‑top.
  - Aspect‑lock toggling constrains resize when an image is known.
  - Startup auto‑fit when a last file exists and setting is enabled.
- When adding tests, keep them focused and colocated under an appropriate crate/test directory.

## Commit & Pull Request Guidelines
- Write concise titles; describe motivation and scope in the body.
- Reference OpenSpec changes: include `Change-ID: <id>` or link to `openspec/changes/<id>/`.
- Include validation notes (platforms tested, commands run) and screenshots/GIFs when UI changes.
- Keep PRs small and reviewable; avoid unrelated refactors.

## Security & Configuration Tips
- Tauri v2 allowlist is minimal by default; only enable what you use.
- Windows installer is NSIS; macOS bundles via `app`/`dmg`.
- Settings persist to `settings.json` under the OS config dir—do not store secrets.

## Architecture Overview
- Two shells exist: legacy macOS (winit) and the cross‑platform Tauri v2 app.
- Core flows live in Tauri: native file open, auto‑fit, aspect‑lock, persistence, and native menus.
404: Not Found
