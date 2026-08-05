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
cargo tauri build --bundles app,dmg --target universal-apple-darwin
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

7. Upload the macOS files plus the Windows installer to the tagged GitHub Release.

## Release checklist

Before creating the tag:

- Update the root package, Tauri package, and Tauri bundle versions together
- Move the release notes from `Unreleased` into the matching version in `CHANGELOG.md`
- Run `cargo check --manifest-path src-tauri/Cargo.toml`
- Run `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
- Confirm the Pages site still describes the current release and links to the stable asset names
- Run `npm run test:ui`

After the tag build finishes:

- Verify the release contains `Float-macos-universal.dmg`, `Float-macos-universal.sha256`, and `Float-windows-x64-setup.exe`
- Download the DMG from the release, compare the checksum, and test the install from a clean macOS machine
- Confirm first launch, file open, fit-to-image, aspect lock, dedicated settings window, slideshow interval, looping multi-file navigation, native opacity, and persistence all work
- Confirm opening or changing an image in one Float window does not replace the active image in another
- Confirm empty, missing-file, and failed-load states still read clearly
- Confirm the GitHub Pages download button resolves to the latest release asset
