# Proposal: Persist settings and last opened file (JSON)

- Change ID: persist-settings-and-last-file
- Summary: Persist the two settings (fit window to image, lock aspect ratio) and the last opened file path in a JSON file under the user config directory. Load these on startup and save when they change.
- Motivation: Preserve user preferences and last working context across app launches for a smoother workflow.

## Goals
- Store settings and last file in a JSON file in a standard per-user location.
- Load on startup, apply immediately (including restoring last file if present).
- Save after settings or file change.

## Non-Goals
- Cloud sync, multi-profile support, or encryption.
- Non-macOS locations (macOS primary; other platforms may no-op).

