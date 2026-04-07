# Float

A polished always-on-top image viewer built in Rust with a Tauri shell for macOS and Windows (Linux dev-only for now). It pins the window above other apps, lets you open an image or image set, and keeps reference material visible without turning into another workspace. Behavior is defined in OpenSpec (see `openspec/specs/`).

## Features
- Always-on-top window on launch (macOS + Windows).
- Open an image via File → Open… (`Cmd/Ctrl+O`); title shows the filename.
- Auto-fit to image on selection with manual Fit Now (`Cmd/Ctrl+F`).
- Dedicated Settings window (`Cmd+,`) with Behavior and Appearance tabs.
- Configurable slideshow timing with looping previous/next navigation for multi-image selections.
- Real native window opacity control plus durable aspect-lock and click-through preferences.
- Per-window image isolation, so changing one Float window does not replace another.
- Polished empty, missing-file, and failed-load states.

Relevant specs: `specs/always-on-top/`, `specs/file-selection/`, `specs/display-image/`, `specs/fit-window/`, `specs/aspect-lock/`, `specs/menu-and-shortcuts/`, `specs/settings-panel/`, `specs/settings-persistence/`, `specs/window-size/`.

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

## Local Development
```sh
just tauri-dev            # Runs Tauri in dev mode
```
- The window launches always-on-top; use File → Open… to pick an image or image set, and `Cmd+,` / `Ctrl+,` to open Settings.

## UI Tests (Playwright)
- Install Node.js 20+ and run `npm ci` to grab Playwright.
- Run `npm run test:ui` for the mocked frontend coverage in `tests/ui-mock.spec.ts` and `tests/settings-panel.spec.ts`.
- Install the Tauri WebDriver once via `cargo install tauri-driver --locked` so the `tauri-driver` binary is on your `PATH` (or export `TAURI_DRIVER_PATH` pointing to it).
- Run `npm run test:ui:tauri` for the Playwright smoke test in `tests/tauri-driver.spec.ts`.

## Native macOS Verification
- `just tauri-check-open-target src-tauri/icons/icon.png` launches Tauri dev with a deterministic image path, opens a new viewer window, triggers `Open…`, and checks that the chosen image loads into the front window rather than an older one.
- This harness is macOS-only and requires Accessibility permission for the host app running the command because it drives the real app menu through `System Events`.
- Use it when desktop-native multi-window behavior is in doubt and the mocked UI tests are not enough.

## Internal Packaging
```sh
just tauri-build          # macOS .app + Windows NSIS installer
```
Outputs:
- macOS app bundle: `src-tauri/target/release/bundle/macos/Float.app`
- Windows NSIS installer: `src-tauri/target/release/bundle/nsis/Float_*.exe`

To open the built macOS app locally:
```sh
just tauri-open
```

### Windows cross-build from macOS
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

## Public Release

The public distribution channel is:

- GitHub Pages for the landing page
- GitHub Releases for the public downloadable assets

Public release assets are published with stable names:

- `Float-macos-universal.dmg`
- `Float-macos-universal.sha256`
- `Float-windows-x64-setup.exe`

The landing page lives in `site/` and links to:

- `https://github.com/Zacaria/float/releases/latest/download/Float-macos-universal.dmg`
- `https://github.com/Zacaria/float/releases/latest/download/Float-macos-universal.sha256`
- `https://github.com/Zacaria/float/releases/latest/download/Float-windows-x64-setup.exe`

The landing page highlights both supported public downloads: the notarized macOS DMG and the Windows x64 installer.

### CI release flow

1. `release-plz` updates `CHANGELOG.md`, versions, tags, and the GitHub Release from `.github/workflows/release-plz.yml`.
2. `.github/workflows/release-bundles.yml` builds a universal macOS bundle on `v*` tags, signs it with a `Developer ID Application` certificate, staples and validates the notarized app and DMG, generates `Float-macos-universal.sha256`, builds the Windows NSIS installer, and publishes all three public assets to the GitHub Release.
3. `.github/workflows/pages.yml` deploys the static landing page from `site/` to GitHub Pages on `master`.

### CI secret contract

The macOS public release workflow requires these repository secrets:

- `APPLE_CERTIFICATE`
- `APPLE_CERTIFICATE_PASSWORD`
- `APPLE_ID`
- `APPLE_PASSWORD`
- `APPLE_TEAM_ID`

See [`docs/releasing.md`](docs/releasing.md) for the exact contract, fallback commands, and the release checklist.

## Troubleshooting
- **Tauri missing deps**: install platform prereqs (Xcode CLT on macOS; MSVC + WebView2 on Windows).
- **Linux**: if building locally, ensure WebKit/WebView2 deps required by Tauri are installed; packaging not yet supported.
- **Window/menu missing**: ensure you’re running the Tauri shell (`just tauri-dev` or `just tauri-build`) and not the legacy winit binary unless you’re on macOS.
- **macOS native verification not working**: grant Accessibility permission to the host app running `osascript` before using `just tauri-check-open-target ...`.

## Contributing
- Specs live under `openspec/specs/`; proposed changes go in `openspec/changes/`.
- Prefer `just tauri-dev` for local runs; keep changes small and update specs when behavior changes.
- Commit subjects must use Conventional Commits such as `feat: ...`, `fix(menu): ...`, or `chore!: ...`.
- Run `just install-git-hooks` once to enable the local `commit-msg` guard, or validate a range manually with `just check-commits origin/main..HEAD`.
