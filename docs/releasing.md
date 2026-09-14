# Releasing Float

Float ships publicly through GitHub Releases, with the macOS and Windows downloads promoted through GitHub Pages. The public deliverables are:

- `Float-macos-universal.dmg`
- `Float-macos-universal.sha256`
- `Float-windows-x64-setup.exe`

## CI secret contract

The `release-bundles` workflow expects these repository secrets:

- `APPLE_CERTIFICATE`: base64-encoded `.p12` for a `Developer ID Application` certificate.
- `APPLE_CERTIFICATE_PASSWORD`: export password for the `.p12`.
- `APPLE_ID`: Apple account email used for notarization.
- `APPLE_PASSWORD`: app-specific password for the Apple account.
- `APPLE_TEAM_ID`: Apple Developer team identifier.

After `release-plz` creates a tag and GitHub Release, the release workflow explicitly dispatches `release-bundles.yml` at that tag. The bundle workflow imports the certificate, resolves the `Developer ID Application` signing identity, builds a universal Tauri bundle, explicitly notarizes the generated DMG, staples the app and DMG, validates notarization, builds the Windows NSIS installer, and publishes the stable macOS and Windows assets to GitHub Releases.

## GitHub Pages landing page

The marketing/download site lives under `site/` and deploys with `.github/workflows/pages.yml`.

- Public download CTA: `https://github.com/Zacaria/float/releases/latest/download/Float-macos-universal.dmg`
- Checksum CTA: `https://github.com/Zacaria/float/releases/latest/download/Float-macos-universal.sha256`
- Windows CTA: `https://github.com/Zacaria/float/releases/latest/download/Float-windows-x64-setup.exe`

When updating the landing page, keep the public feature story aligned with the shipped app:

- dedicated settings window for durable controls
- configurable and looping slideshow playback for multi-image selections
- real native opacity rather than CSS-only dimming
- per-window image isolation and polished empty/error states

Do not promote capability-gated blur support as a headline feature unless the implementation becomes uniformly available across supported platforms.

## Manual fallback

If the GitHub workflow is unavailable, use a local macOS machine with the Developer ID certificate installed in Keychain.

1. Confirm the signing identity:

```sh
security find-identity -v -p codesigning
```

2. Build a universal macOS bundle:

```sh
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm exec --yes --package=@tauri-apps/cli@2.8.4 -- tauri build --bundles app,dmg --target universal-apple-darwin
```

3. Locate the outputs:

```sh
APP_PATH="src-tauri/target/universal-apple-darwin/release/bundle/macos/Float.app"
DMG_PATH="$(find src-tauri/target/universal-apple-darwin/release/bundle/dmg -maxdepth 1 -name '*.dmg' | head -n 1)"
```

4. Submit the DMG for notarization:

```sh
xcrun notarytool submit "$DMG_PATH" \
  --apple-id "$APPLE_ID" \
  --password "$APPLE_PASSWORD" \
  --team-id "$APPLE_TEAM_ID" \
  --wait
```

5. Staple and validate:

```sh
xcrun stapler staple "$APP_PATH"
xcrun stapler validate "$APP_PATH"
xcrun stapler staple "$DMG_PATH"
xcrun stapler validate "$DMG_PATH"
spctl -a -vvv "$APP_PATH"
spctl -a -vvv -t open --context context:primary-signature "$DMG_PATH"
```

6. Rename to the stable public asset names and create the checksum:

```sh
cp "$DMG_PATH" Float-macos-universal.dmg
shasum -a 256 Float-macos-universal.dmg > Float-macos-universal.sha256
```

7. Run the packaged verification commands below on each native host. Upload
   only after both pass; keep the JSON reports as release evidence.

## Release checklist

Before creating the tag:

- Update both Rust packages and their own lockfile entries, npm package and lockfile root entries, Tauri config, and the live site latest-version copy together
- Move the release notes from `Unreleased` into the matching version in `CHANGELOG.md`
- Run `cargo check --manifest-path src-tauri/Cargo.toml`
- Run `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
- Confirm the Pages site still describes the current release and links to the stable asset names
- Run `npm run test:branding`, `npm run test:release` (requires Python and pinned pefile), and `npm run test:ui`

After the tag build finishes:

- Verify the release contains `Float-macos-universal.dmg`, `Float-macos-universal.sha256`, and `Float-windows-x64-setup.exe`
- Download the DMG from the release, compare the checksum, and test the install from a clean macOS machine
- Confirm first launch, file open, fit-to-image, aspect lock, dedicated settings window, slideshow interval, looping multi-file navigation, native opacity, and persistence all work
- Confirm opening or changing an image in one Float window does not replace the active image in another
- Confirm empty, missing-file, and failed-load states still read clearly
- Confirm the GitHub Pages download button resolves to the latest release asset


## Packaged verification

Release builds use `npm exec --yes --package=@tauri-apps/cli@2.8.4 -- tauri build`.
The pin matches icon export; it replaces the unpinned Cargo CLI installation.
Signing and notarization remain mandatory. Both build jobs have `contents: read`;
only the final asset publication job has `contents: write` and needs both jobs.
The separate release-plz workflow still creates the tag/release before bundling;
this gate controls publication of the downloadable assets.

Run source/helper checks with Node and Python 3.12:

```sh
python3 -m pip install pefile==2024.8.26
npm run test:branding
npm run test:release
```

On Windows use `python` in place of `python3`. The cross-platform npm test
launcher selects the interpreter. The helper suite uses real old release icons
and minimal PE byte fixtures; it is independent of native signing tools.

After creating the stable final artifacts on their respective native hosts:

```sh
# macOS, after notarization and stapling:
python3 scripts/verify-packaging.py macos Float-macos-universal.dmg verification-macos.json
# Windows, with the built src-tauri/target/release/float-tauri.exe still present:
python scripts/verify-packaging.py windows Float-windows-x64-setup.exe verification-windows.json
```

The macOS check mounts the final DMG read-only and checks the actual `Float.app`
inside it: plist icon resource, exact approved app and volume ICNS, both versions, bundle ID,
code signature, Gatekeeper and ICNS decode. Exposed frontend files must match;
compiled frontend assets are explicitly reported inaccessible. Detach runs even
when verification fails.

The Windows check compares all PE icon group frame bytes and file/product
versions in the installer, built EXE, silently installed EXE and uninstaller.
The install uses `/S /NS /D=<isolated RUNNER_TEMP path>`, with `/D` last and no
`/R`; the app is not launched. These options follow the
[NSIS command-line contract](https://nsis.sourceforge.io/Docs/Chapter3.html) and
[Tauri 2.8.4 installer template](https://github.com/tauri-apps/tauri/blob/tauri-cli-v2.8.4/crates/tauri-bundler/src/bundle/windows/nsis/installer.nsi).
Temporary files are removed, though registry installation records may remain on
the disposable runner. No Windows signing bypass is introduced; this workflow
had no Windows signing credential setup.

CI uploads successful reports as `float-macos-verification` and
`float-windows-verification` for 14 days. Inspect their source commit, config
version, icon hashes and container hashes before publication review. The reports
are not attached to the public release; the stable download names are unchanged.
No JSON report is emitted as a success when a native assertion fails.
