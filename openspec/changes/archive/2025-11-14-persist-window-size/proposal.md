# Proposal: Persist window size (JSON)

- Change ID: persist-window-size
- Summary: Persist the window size in the same JSON file as settings and last file, restore it on startup, and update it on window resize.
- Motivation: Restore working layout between sessions for a smoother workflow.

## Goals
- Save window size on resize and on app exit.
- Restore size on startup before loading images.
- Interoperate with Fit-to-image: when enabled, image fit may override saved size on selection.

## Non-Goals
- Position persistence or multi-display placement.

