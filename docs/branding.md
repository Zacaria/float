# Approved Float icon

Float uses user-approved proposal **02, layered panes**, implemented on 2026-09-14.
The approval is for the delivered PNG, including its artwork, colors, shadows,
transparent edges and interior violet rectangle/coral square. Do not redraw,
remove content, recolor, crop or modify the master's alpha.

Approved source:
`/home/hermes/.local/share/open-design/projects/float-icon-proposals-gpt-image-2-5-2820/proposals/02-layered-panes.png`

SHA-256: `c17820f006724e0c375a6e0d10c13aa41aa45bbb49868af8169ece8f6f53dc81`.

`src-tauri/icons/icon_base_1024.png` and `site/assets/float-icon.png` are exact
516,471-byte copies: 1024×1024, 8-bit RGBA PNGs, with alpha spanning 0–255.
Neither master is re-encoded during export.

## Model provenance

The art was generated through **Codex native `image_gen`**. The original PNG's
C2PA action metadata reports `softwareAgent.name = gpt-image` and
`softwareAgent.version = 2.0`. Those metadata bytes were inspected directly;
C2PA signatures were not cryptographically verified. **GPT Image 2.0** is the
reported generator, not 2.5; the project directory and original prompt's
preference for 2.5 are not evidence of that model being used. No exact backend
snapshot is claimed. The separate OpenDesign Cloud `float-02.json` record says
`not_attempted` and is not the provenance of this approved image.

The OpenDesign project's `proposals/PROVENANCE.md`, `generation-record.json`,
and `originals/02-layered-panes.png` retain the generation evidence. Per that
record, the native output was 1254×1254; its approved 1024×1024 export was made
with Pillow RGBA conversion and LANCZOS downsampling, preserving alpha. This
implementation starts from the approved export, not that earlier original,
and makes no additional AI generation/editing calls.

## Inventory and derivation

The initial checkout was clean on `master`. All 53 existing branding artifacts
were inventoried before replacement. The complete path, SHA-256 and PNG
width/height/bit-depth/color-type inventory for the approved exports is
[`tests/branding-exports.json`](../tests/branding-exports.json).

| Existing artifacts | Count | Derivation |
| --- | ---: | --- |
| `src-tauri/icons/icon_base_1024.png` | 1 | Exact approved source bytes |
| Desktop PNGs, `icon.ico`, `icon.icns` under `src-tauri/icons/` | 16 | Tauri CLI 2.8.4 default family; `icon.png` is 512×512 |
| `src-tauri/icons/android/mipmap-*/ic_launcher*.png` | 15 | Same Tauri export, all five density directories |
| `src-tauri/icons/ios/AppIcon-*.png` | 18 | Same Tauri export with transparent iOS background |
| `dist/favicon.ico` | 1 | Exact copy of generated native `icon.ico` |
| `dist/apple-touch-icon.png` | 1 | Tauri custom 180×180 PNG directly from the master |
| `site/assets/float-icon.png` | 1 | Exact approved source bytes |

Every previously tracked native PNG retains its previous dimensions, including
Tauri's 49×49 Android hdpi launcher/round outputs. The CLI also creates a
64×64 desktop PNG, but it was not previously tracked and is left in temporary
staging. The export script copies only the inventoried paths.

Tauri's resizing/encoding supplies the platform derivatives. `--ios-color
'#00000000'` avoids its default white flattening; no background is added to
the master or web exports. ICNS entry ordering varies between CLI runs, so the
script sorts its container entries by their four-byte type. Every encoded
image/mask payload remains verbatim. This makes the final family byte reproducible.

The existing links in `dist/index.html` and `site/index.html` and the bundle
icon paths in `src-tauri/tauri.conf.json` already target these files. No HTML,
CSS, UI logic or bundle configuration edits were needed. The legacy root
`Cargo.toml` has an empty bundle icon list and is outside the active Tauri flow.

## Reproduce

Use Node with its built-in test runner and npm. No permanent dependency is
added. The generator downloads/runs exactly `@tauri-apps/cli@2.8.4` via npm's
cache; it does not require Rust or install the Float app.

```sh
npm ci
node scripts/generate-branding.cjs
npm run test:branding
npx playwright install chromium
npm run test:ui
git diff --check
```

For the initial import, pass the approved source path as the generator's sole
argument. With no argument it reads the checked-in canonical master. The
script checks the approved hash before generation and verifies every staged
export against the regression manifest before writing any repository asset.
It never refreshes expected hashes automatically.

The underlying CLI commands, executed in a temporary directory, are:

```sh
branding_stage=$(mktemp -d)
npm exec --yes --package=@tauri-apps/cli@2.8.4 -- tauri icon \
  src-tauri/icons/icon_base_1024.png --output "$branding_stage/icons" \
  --ios-color '#00000000'
npm exec --yes --package=@tauri-apps/cli@2.8.4 -- tauri icon \
  src-tauri/icons/icon_base_1024.png --output "$branding_stage/web" --png 180
```

Use `scripts/generate-branding.cjs` for ICNS ordering, verified copies, favicon
parity, and automatic temporary-directory cleanup. Repeat staged generations
were compared: only raw ICNS ordering differed; after sorting, all bytes matched.

## Verification record and boundaries

- Regression tests were added before repository asset replacement. RED:
  `npm run test:branding` exited 1, with 55 failures and 4 passes against the old
  artwork. GREEN: the same command passed all 59 tests after replacement.
- Checks cover both exact masters, all export bytes, PNG dimensions/type,
  full native inventory, web branding references, configured bundle paths,
  native/web ICO equality and its six frame sizes, and all 12 ICNS entries.
  `.github/workflows/ui-tests.yml` runs the branding suite before browser setup.
  Export hashes protect the complete bytes, including alpha and metadata;
  the Node checks do not implement a full image decoder.
- A separate Pillow 12.3.0 audit decoded all 53 files and every ICO/ICNS
  representation. Every PNG remained RGBA with alpha extrema 0 and 255.
- `npm ci` succeeded with the existing lockfile. The first `npm run test:ui`
  needed Chromium; after `npx playwright install chromium`, all 7 existing
  mocked UI tests passed, including the final run after replacement. No
  additional browser system packages were needed.
- Rust was discovered at `/home/hermes/.cargo/bin` using `rustup show`:
  1.91.1, target `x86_64-unknown-linux-gnu`. The no-bundle attempt
  `PATH=/home/hermes/.cargo/bin:$PATH cargo build --locked --manifest-path src-tauri/Cargo.toml`
  reached dependency compilation, then exited 101 in `glib-sys` because
  `pkg-config` was absent. GTK/WebKit development prerequisites were not
  installed for this asset-only task. No successful native build is claimed.
- macOS Dock/Finder, Windows Explorer/taskbar/NSIS, mobile launcher/store
  acceptance, native webview loading, signing and OS icon cache behavior
  remain untested. Transparency was retained as approved, including iOS;
  these exports do not establish store acceptance. Playwright uses mocked
  Tauri and does not substitute for native verification.
- `git diff --check` passed. No commit, push, release, deployment, application
  installation or sibling repository edit was performed.
- `OPENSPEC_TELEMETRY=0 npm exec --yes --package=@fission-ai/openspec@0.23.0 -- openspec validate app-branding --type spec --strict`
  passed. The existing unrelated pending changes were left untouched.

The visual review is outside the repo, under the OpenDesign project's
`proposals/02-layered-panes-implementation-size-review.png`: actual ICO/PNG
exports at 16, 24, 32, 48, 64, 128, 256 and 512 pixels on dark/light backgrounds.
Its companion `review-float-implementation.py` and
`float-icon-implementation-audit.json` retain the review recipe and decoded
inventory. Compositing there is for review only and does not alter exported assets.
The `proposals/implementation-verification/` directory retains the initial
53-file inventory, RED/GREEN logs, final UI log, native compile failure log,
and reproduction log for parent verification.
