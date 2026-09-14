# Changelog

All notable changes to this project will be documented here. Release automation is powered by `release-plz`.

## Unreleased

## 0.2.3 - 2026-09-14

- Ship the approved layered-panes artwork across macOS/Windows apps, disk image, installer, and uninstaller.
- Accept the Windows compiler's valid PNG color-plane metadata normalization while still requiring exact icon image bytes.
- Retain real compiled-resource regression evidence; supersede the unpublished v0.2.1 and v0.2.2 attempts without moving their tags.

## 0.2.2 - 2026-09-14

- Deliver the approved layered-panes icon in the macOS app and DMG volume, and the Windows app, installer, and uninstaller.
- Fix release checks on Windows CRLF checkouts and add an isolated cross-platform regression.
- Supersede the unpublished v0.2.1 release attempt without moving its tag; native package checks remain mandatory before publication.

## 0.2.1 - 2026-09-14

- Ship the approved layered-panes icon in macOS DMG and Windows installers, including the legacy macOS bundle.
- Verify final packaged icons and versions before publishing release assets.

## 0.2.0 - 2026-08-05

- Move settings into a dedicated window and keep active images isolated between viewer windows.
- Add native opacity, slideshow timing, looping navigation, and clearer empty and failed-load states.
- Add deterministic macOS window-target verification and broader mocked UI coverage.
- Publish macOS and Windows downloads from a product page centered on keeping references visible.
