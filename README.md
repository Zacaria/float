# Float

A tiny always-on-top image viewer built in Rust with a Tauri shell for macOS and Windows (Linux dev-only for now). It pins the window above other apps, lets you open an image, and auto-fits the window to the image. Behavior is defined in OpenSpec (see `openspec/specs/`).

## Features
- Always-on-top window on launch (macOS + Windows).
- Open an image via File → Open… (`Cmd/Ctrl+O`); title shows the filename.
- Auto-fit to image on selection with manual Fit Now (`Cmd/Ctrl+F`).
- Optional aspect-lock toggle in the native menu; remembers window size and last opened file.

Relevant specs: `specs/always-on-top/`, `specs/file-selection/`, `specs/fit-window/`, `specs/aspect-lock/`, `specs/menu-and-shortcuts/`, `specs/window-size/`, `specs/settings-persistence/`.

## Platforms
- macOS: supported (development and bundled app).
- Windows: supported (development and NSIS installer).
- Linux: dev-only; no packaged binary yet (build/run locally).

## Prerequisites
- Rust toolchain (`rustup`, `cargo`).
- Tauri CLI for bundling/dev: `cargo install tauri-cli`.
- Platform deps:
  - macOS: Xcode Command Line Tools.
  - Windows: Visual Studio Build Tools (MSVC) + WebView2 Runtime.
  - Linux: system dependencies per Tauri docs; only dev run covered here.
- Optional: `just` for common tasks (install via `cargo install just`).

## Quick Start (Tauri shell)
```sh
# Clone and enter repo
just tauri-dev            # Runs Tauri in dev mode
```
- The window launches always-on-top; use File → Open… to pick an image.

## UI Tests (Playwright)
- Install Node.js 20+ and run `npm ci` to grab Playwright.
- Install the Tauri WebDriver once via `cargo install tauri-driver --locked` so the `tauri-driver` binary is on your `PATH` (or export `TAURI_DRIVER_PATH` pointing to it).
- Execute `npm run test:ui` to run the Playwright spec in `tests/`.

### Build Bundles (release artifacts)
```sh
just tauri-build          # macOS .app + Windows NSIS installer
```
Artifacts:
- macOS app bundle: `src-tauri/target/release/bundle/macos/Float.app`
- Windows NSIS installer: `src-tauri/target/release/bundle/nsis/Float_*.exe`

To open the built macOS app locally:
```sh
just tauri-open
```

### Cross-build Windows executable (macOS host)
```sh
just tauri-build-windows
```
- Installs the `x86_64-pc-windows-msvc` Rust target and `cargo-xwin` if missing, then cross-builds the Tauri shell.
- Outputs a Windows executable at `src-tauri/target/x86_64-pc-windows-msvc/release/Float.exe` for quick sharing/tests (NSIS packaging still requires Windows or CI).

### Legacy winit app (macOS only)
```sh
just build-run            # cargo run
just bundle-run           # cargo bundle --release (macOS .app)
```

## Downloads
- GitHub Actions (workflow: `release-bundles`) builds macOS and Windows artifacts and uploads them to the Releases page when a tag (`v*`) is pushed or the workflow is dispatched manually.
- If no release is published yet, build locally using the commands above.
- Linux packages are not produced; run locally on Linux if needed.

## Release Pipeline (manual + CI)
1) Ensure `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and `cargo check` pass.
2) Release automation: `.github/workflows/release-plz.yml` (runs on `main` pushes or manual dispatch) uses `release-plz` to update `CHANGELOG.md`, bump versions, tag with `v*`, and create the GitHub Release (no crates.io publish).
3) Bundles on tags: `.github/workflows/release-bundles.yml` builds on `v*` tags (or manual dispatch), uploads build artifacts as workflow artifacts, and publishes/updates the GitHub Release with the macOS zip + Windows installer.
4) Local build sanity (optional): `just tauri-build`; collect artifacts from `src-tauri/target/release/bundle/macos/Float.app` and `src-tauri/target/release/bundle/nsis/Float_*.exe`.
5) Draft release notes summarizing changes and link relevant OpenSpec change IDs (release-plz populates the changelog automatically).

## Troubleshooting
- **Tauri missing deps**: install platform prereqs (Xcode CLT on macOS; MSVC + WebView2 on Windows).
- **Linux**: if building locally, ensure WebKit/WebView2 deps required by Tauri are installed; packaging not yet supported.
- **Window/menu missing**: ensure you’re running the Tauri shell (`just tauri-dev` or `just tauri-build`) and not the legacy winit binary unless you’re on macOS.

## Contributing
- Specs live under `openspec/specs/`; proposed changes go in `openspec/changes/`.
- Prefer `just tauri-dev` for local runs; keep changes small and update specs when behavior changes.
