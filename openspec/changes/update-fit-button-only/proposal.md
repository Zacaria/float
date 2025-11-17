## Why
- Fit was previously a toggle persisted in settings and auto-applied on selection; the desired behavior is a one-time action that adjusts the current window using the image aspect ratio.
- The fit operation should derive from the current window’s longer side and adjust the other dimension down to match the image ratio, avoiding persisted state.

## What Changes
- Replace the Fit window toggle with a manual Fit action (button/menu/shortcut) that does not persist any setting.
- Update the fit algorithm: take the current window’s larger dimension as the anchor and set the other dimension based on the image’s aspect ratio, shrinking as needed rather than fitting to real image pixels.
- Remove fit persistence and auto-fit-on-selection behavior; manual Fit remains via existing shortcut/menu.

## Impact
- Affected specs: fit-window (remove auto-fit setting, redefine manual fit behavior).
- Affected code: settings schema/state, menu wiring (remove toggle), fit algorithm implementation.
