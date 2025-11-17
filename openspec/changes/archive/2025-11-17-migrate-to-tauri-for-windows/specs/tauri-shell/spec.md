## ADDED Requirements

### Requirement: Launch cross-platform Tauri app
The application MUST launch using Tauri on both macOS and Windows and present its main window.

#### Scenario: Launch on macOS
- Given macOS with required toolchains installed
- When the developer runs `tauri dev` or `tauri build` and opens the app
- Then the app window launches and is functional

#### Scenario: Launch on Windows
- Given Windows with required toolchains installed
- When the developer runs `tauri dev` or installs from the generated installer
- Then the app window launches and is functional

