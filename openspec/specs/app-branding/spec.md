# app-branding Specification

## Purpose
TBD - created by archiving change rename-project-to-float. Update Purpose after archive.
## Requirements
### Requirement: Approved layered-panes icon across surfaces
The canonical and site masters MUST preserve the exact user-approved proposal 02
PNG bytes (SHA-256 `c17820f006724e0c375a6e0d10c13aa41aa45bbb49868af8169ece8f6f53dc81`),
including its content, colors and alpha, at `src-tauri/icons/icon_base_1024.png`
and `site/assets/float-icon.png`. Existing desktop/mobile and webview
icons MUST be derived from that master using the pinned export process in
`docs/branding.md`.

#### Scenario: Masters preserve the approved artwork
- Given the user-approved layered-panes proposal
- When inspecting the canonical and site master PNGs
- Then both are identical 1024×1024 RGBA files with the approved SHA-256

#### Scenario: Native and web branding stay current
- Given the approved master and pinned Tauri CLI export process
- When regenerating the existing icon family
- Then all tracked platform outputs are updated without redesigning or flattening the artwork
- And the webview favicon equals the native ICO byte-for-byte
- And the apple touch icon is derived directly from the master at 180×180
- And `npm run test:branding` verifies export bytes, dimensions and branding references

#### Scenario: Published installers contain the approved artwork
- Given a patch release built after the approved-icon source update
- When preparing the final notarized DMG and Windows NSIS installer for publication
- Then both runners pass source branding tests before building
- And the mounted DMG's Float.app icon bytes and bundle versions match the approved source and current config
- And Windows installer, application, installed application and uninstaller PE icons match the approved ICO frames
- And legacy macOS packaging references the canonical ICNS
- And failed packaged verification blocks public asset upload

### Requirement: Product name is Float across surfaces
The application MUST present the product name as "Float" across UI, metadata, bundles, and documentation so no "Always On Top" branding remains.

#### Scenario: Window and menus show Float
- Given the app runs on macOS or Windows
- When the user views the window title or app/menu labels (e.g., Quit)
- Then the product name shown is "Float" and not "Always On Top"

#### Scenario: Bundles and installers named Float
- Given the app is built using the documented steps (e.g., `just tauri-build` or `tauri build`)
- When inspecting the outputs
- Then the macOS bundle is named `Float.app` and the Windows installer uses the Float product name (e.g., `Float_*.exe`), reflecting the Float brand

#### Scenario: Docs and metadata use Float
- Given a contributor reads the README or app metadata
- When they follow build/run instructions or view app details
- Then the product name referenced is "Float" with updated paths/output names, and no "Always On Top" strings remain

#### Scenario: Settings namespace uses Float branding
- Given the app stores settings on disk
- When inspecting the settings namespace or config path
- Then it uses the Float-branded identifier and does not reference legacy app names
