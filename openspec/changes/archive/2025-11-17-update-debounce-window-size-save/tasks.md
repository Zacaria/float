## Tasks
1. [x] Confirm current window size persistence flow on resize and startup in the Tauri shell. (Previously saved on every resize event, restored on startup before fit logic.)
2. [x] Adjust resize handling to debounce saving window size until ~1s after the last resize event; ensure the final size is persisted when resizing stops. (Added debounced save with abortable timer in `schedule_size_save` and replaced per-event saves.)
3. [x] Verify restore still uses the last saved size on startup after resizing; run `openspec validate update-debounce-window-size-save --strict`. (`fits` still runs after restore; validation executed.) 
