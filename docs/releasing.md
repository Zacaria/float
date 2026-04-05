# Releasing Float

Float ships publicly as a notarized macOS download hosted on GitHub Releases and promoted through GitHub Pages. The public deliverables are:

- `Float-macos-universal.dmg`
- `Float-macos-universal.sha256`

Windows can continue to build as an internal preview artifact, but it is not part of the public release surface yet.

## CI secret contract

The `release-bundles` workflow expects these repository secrets:

- `APPLE_CERTIFICATE`: base64-encoded `.p12` for a `Developer ID Application` certificate.
- `APPLE_CERTIFICATE_PASSWORD`: export password for the `.p12`.
- `APPLE_ID`: Apple account email used for notarization.
- `APPLE_PASSWORD`: app-specific password for the Apple account.
- `APPLE_TEAM_ID`: Apple Developer team identifier.

The workflow imports the certificate, resolves the `Developer ID Application` signing identity, builds a universal Tauri bundle, explicitly notarizes the generated DMG, staples the app and DMG, validates notarization, and publishes only the stable macOS assets to GitHub Releases.

## GitHub Pages landing page

The marketing/download site lives under `site/` and deploys with `.github/workflows/pages.yml`.

- Public download CTA: `https://github.com/Zacaria/float/releases/latest/download/Float-macos-universal.dmg`
- Checksum CTA: `https://github.com/Zacaria/float/releases/latest/download/Float-macos-universal.sha256`

When updating the landing page, keep it macOS-only until the Windows public release path has signing, trust, and support language of its own.

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
spctl -a -vvv -t open "$DMG_PATH"
```

6. Rename to the stable public asset names and create the checksum:

```sh
cp "$DMG_PATH" Float-macos-universal.dmg
shasum -a 256 Float-macos-universal.dmg > Float-macos-universal.sha256
```

7. Upload both files to the tagged GitHub Release.

## Release checklist

Before creating the tag:

- Run `cargo check --manifest-path src-tauri/Cargo.toml`
- Run `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
- Confirm the Pages site still describes the current release and links to the stable asset names

After the tag build finishes:

- Verify the release contains `Float-macos-universal.dmg` and `Float-macos-universal.sha256`
- Download the DMG from the release, compare the checksum, and test the install from a clean macOS machine
- Confirm first launch, file open, fit-to-image, aspect lock, slideshow, multi-file navigation, and persistence all work
- Confirm the GitHub Pages download button resolves to the latest release asset
