## Tasks
1. [x] Verify current bundle outputs/paths so we know where the cross-built `.exe` should land.
2. [x] Add a `just tauri-build-windows` recipe that installs `cargo-xwin` if missing, ensures the `x86_64-pc-windows-msvc` target exists, and cross-builds the Tauri shell (renaming the output to `Float.exe`).
3. [x] Document the new command + prerequisites in the README.
4. [x] Update the `packaging` spec with a macOS cross-build scenario and validate via `openspec validate add-windows-cross-build --strict`.
