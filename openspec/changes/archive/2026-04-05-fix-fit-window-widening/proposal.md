## Why
Manual Fit to Image currently widens the window instead of shrinking it to the image aspect ratio, breaking the intended fit-window behavior.

## What Changes
- Correct the manual fit algorithm to anchor on the current larger dimension and reduce the other, never upscaling.
- Update the fit-window spec to restate the manual fit expectation and cover the non-upscaling case explicitly.

## Impact
- Affected specs: fit-window
- Affected code: manual fit command handling and resize logic in the Tauri shell
