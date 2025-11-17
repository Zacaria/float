# app-branding (Change Delta)

## ADDED Requirements

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

#### Scenario: Settings namespace aligns with Float without data loss
- Given existing users have persisted settings under the previous app name
- When launching the renamed app
- Then persisted settings continue working under a Float-branded identifier, migrating prior data if the identifier changed
