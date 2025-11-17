## Why
- Window size is persisted on every resize event, leading to excessive disk writes during continuous resizing and making it harder to guarantee the final size is captured.
- Debouncing saves until a short quiet period reduces churn while ensuring the last user intent is remembered on reopen.

## What Changes
- Persist window size about one second after the last resize event instead of on every resize callback.
- Keep restore-on-startup behavior the same; only the timing of writes changes.
- Update window-size requirement to cover the debounced persistence behavior.

## Impact
- Touches Tauri resize handling/persistence; no UI or settings changes.
- Minimal risk; requires manual verification that the final size is restored after resizing and waiting briefly.
