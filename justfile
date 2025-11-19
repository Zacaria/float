set shell := ["bash", "-uc"]

APP := "target/release/bundle/osx/Float.app"
BUNDLE_VERSION := "0.6.0"

# Install cargo-bundle in a known-good version
install-bundler:
	set -euo pipefail
	cargo install cargo-bundle --version {{BUNDLE_VERSION}}

# Build and run the app in dev mode
build-run:
	set -euo pipefail
	cargo run


# Build, bundle, and run the macOS app bundle (legacy winit app)
bundle-run:
	set -euo pipefail
	cargo build --release
	if ! command -v cargo-bundle >/dev/null 2>&1; then echo "cargo-bundle not found; installing v{{BUNDLE_VERSION}}..." >&2; cargo install cargo-bundle --version {{BUNDLE_VERSION}}; fi
	cargo bundle --release
	open "{{APP}}"

# --- Tauri (cross-platform) ---

# Run Tauri app in dev mode (requires tauri-cli)
tauri-dev:
	set -euo pipefail
	RUST_BACKTRACE=1 cargo tauri dev

# Build Tauri bundles (macOS .app / Windows NSIS)
tauri-build:
	set -euo pipefail
	cargo tauri build

# Cross-build Windows release executable from macOS using cargo-xwin
tauri-build-windows:
	set -euo pipefail; TARGET="x86_64-pc-windows-msvc"; if ! rustup target list --installed | grep -q "^${TARGET}$"; then rustup target add "${TARGET}"; fi; if ! command -v cargo-xwin >/dev/null 2>&1; then echo "cargo-xwin not found; installing..." >&2; cargo install cargo-xwin --locked; fi; if [ "$(uname -s)" != "Darwin" ]; then echo "tauri-build-windows is intended to run from macOS hosts" >&2; fi; cargo xwin build --release --target "${TARGET}" --manifest-path src-tauri/Cargo.toml; APP_EXE="src-tauri/target/${TARGET}/release/always-on-top-tauri.exe"; FLOAT_EXE="src-tauri/target/${TARGET}/release/Float.exe"; if [ ! -f "${APP_EXE}" ]; then echo "Expected executable not found at ${APP_EXE}" >&2; exit 1; fi; cp "${APP_EXE}" "${FLOAT_EXE}"; echo "Windows executable ready at ${FLOAT_EXE}"

# Open built macOS .app from Tauri
tauri-open:
	set -euo pipefail
	APP_TAURI="src-tauri/target/release/bundle/macos/Float.app"
	if [ -d "$APP_TAURI" ]; then open "$APP_TAURI"; else echo "App not found: $APP_TAURI" >&2; exit 1; fi

# Run release-plz to bump versions, create tag, and push branch+tags
# Requires clean working tree and access to origin remote.
release-bump:
	set -euo pipefail
	if [ -n "$(git status --porcelain)" ]; then echo "Working tree not clean. Commit or stash first." >&2; exit 1; fi
	if ! command -v release-plz >/dev/null 2>&1; then echo "Installing release-plz..." >&2; cargo install release-plz --locked; fi
	release-plz release --config release-plz.toml
	git push --follow-tags
