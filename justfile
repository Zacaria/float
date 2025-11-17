set shell := ["bash", "-uc"]

APP := "target/release/bundle/osx/Always On Top.app"
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

# Open built macOS .app from Tauri
tauri-open:
	set -euo pipefail
	APP_TAURI="src-tauri/target/release/bundle/macos/Always On Top.app"
	if [ -d "$APP_TAURI" ]; then open "$APP_TAURI"; else echo "App not found: $APP_TAURI" >&2; exit 1; fi

# Run release-plz to bump versions, create tag, and push branch+tags
# Requires clean working tree and access to origin remote.
release-bump:
	set -euo pipefail
	if [ -n "$(git status --porcelain)" ]; then echo "Working tree not clean. Commit or stash first." >&2; exit 1; fi
	if ! command -v release-plz >/dev/null 2>&1; then echo "Installing release-plz..." >&2; cargo install release-plz --locked; fi
	release-plz release --config release-plz.toml
	git push --follow-tags
