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

# Build, bundle, and run the macOS app bundle
bundle-run:
	set -euo pipefail
	cargo build --release
	if ! command -v cargo-bundle >/dev/null 2>&1; then echo "cargo-bundle not found; installing v{{BUNDLE_VERSION}}..." >&2; cargo install cargo-bundle --version {{BUNDLE_VERSION}}; fi
	cargo bundle --release
	open "{{APP}}"
