## Why
- The app currently accepts any file path, which can include videos or other unsupported types; this can lead to failed loads and confusing behavior.
- Restricting picks to image files aligns with the viewer’s purpose and avoids showing broken content.

## What Changes
- Constrain native file selection to images only (standard picture extensions) and reject/skip other file types.
- Ensure multi-file selection and navigation only include allowed image files; provide a clear fallback when nothing valid is selected.
- Keep title/auto-fit/persistence behavior unchanged for valid images.

## Impact
- Affected specs: file-selection (input validation), possibly file-display behavior wording.
- Affected code: dialog filters, backend validation, frontend handling of invalid selections.
