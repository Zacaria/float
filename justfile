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

# Build Tauri bundles for the current host platform
tauri-build:
	set -euo pipefail
	cargo tauri build

# Cross-build Windows release executable from macOS using cargo-xwin
tauri-build-windows:
	set -euo pipefail; TARGET="x86_64-pc-windows-msvc"; if ! rustup target list --installed | grep -q "^${TARGET}$"; then rustup target add "${TARGET}"; fi; if ! command -v cargo-xwin >/dev/null 2>&1; then echo "cargo-xwin not found; installing..." >&2; cargo install cargo-xwin --locked; fi; if [ "$(uname -s)" != "Darwin" ]; then echo "tauri-build-windows is intended to run from macOS hosts" >&2; fi; cargo xwin build --release --target "${TARGET}" --manifest-path src-tauri/Cargo.toml; APP_EXE="src-tauri/target/${TARGET}/release/float-tauri.exe"; FLOAT_EXE="src-tauri/target/${TARGET}/release/Float.exe"; if [ ! -f "${APP_EXE}" ]; then echo "Expected executable not found at ${APP_EXE}" >&2; exit 1; fi; cp "${APP_EXE}" "${FLOAT_EXE}"; echo "Windows executable ready at ${FLOAT_EXE}"

# Open built macOS .app from Tauri
tauri-open:
	set -euo pipefail
	APP_TAURI="src-tauri/target/release/bundle/macos/Float.app"
	if [ -d "$APP_TAURI" ]; then open "$APP_TAURI"; else echo "App not found: $APP_TAURI" >&2; exit 1; fi

# Launch Tauri dev with a deterministic image path, then verify Cmd+T + Cmd+O loads into the new window.
# macOS only. Requires Accessibility permission for the terminal/Codex app.
tauri-check-open-target image:
	set -euo pipefail; \
	if [ "$(uname -s)" != "Darwin" ]; then echo "tauri-check-open-target is macOS-only" >&2; exit 1; fi; \
	ABS_IMAGE="$(cd "$(dirname "{{image}}")" && pwd)/$(basename "{{image}}")"; \
	if [ ! -f "$ABS_IMAGE" ]; then echo "Image not found: $ABS_IMAGE" >&2; exit 1; fi; \
	pkill -x float-tauri >/dev/null 2>&1 || true; \
	pkill -x Float >/dev/null 2>&1 || true; \
	LOG_FILE="$(mktemp -t float-open-target.XXXXXX.log)"; \
	APP_NAME=float-tauri FLOAT_TEST_PATH="$ABS_IMAGE" RUST_BACKTRACE=1 cargo tauri dev >"$LOG_FILE" 2>&1 & \
	DEV_PID=$!; \
	trap 'kill "$DEV_PID" >/dev/null 2>&1 || true; pkill -x float-tauri >/dev/null 2>&1 || true; pkill -x Float >/dev/null 2>&1 || true; echo "Tauri dev log: $LOG_FILE"' EXIT; \
	scripts/macos-open-target-check.sh "$ABS_IMAGE"

# Publish a version and changelog entry already prepared in the repository.
# Requires a clean working tree and access to origin.
release-bump:
	set -euo pipefail
	if [ -n "$(git status --porcelain)" ]; then echo "Working tree not clean. Commit or stash first." >&2; exit 1; fi
	if ! command -v release-plz >/dev/null 2>&1; then echo "Installing release-plz..." >&2; cargo install release-plz --locked; fi
	release-plz release --config release-plz.toml
	git push --follow-tags
