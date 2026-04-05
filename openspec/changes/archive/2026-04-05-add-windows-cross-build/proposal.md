## Why
- macOS contributors cannot currently produce a Windows executable without using a Windows VM/host, which slows down validation and distribution.
- Providing a repeatable `just` recipe keeps platform build steps aligned and lets us document the supported cross-build flow.

## What Changes
- Add a cross-target `just` task that compiles the Tauri shell for the Windows MSVC target using `cargo-xwin` and renames the resulting `.exe` for easy sharing.
- Document the required toolchain additions (`rustup target add`, `cargo-xwin`) and update packaging requirements to cover the macOS-to-Windows build flow.

## Impact
- Specs: `packaging` capability gains a scenario covering the macOS cross-build command/output.
- Tooling: `justfile` grows a Windows-cross-build recipe.
- Docs: README build instructions mention the new flow and prerequisites.
