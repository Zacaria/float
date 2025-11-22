## 1. Implementation
- [x] 1.1 Update the fit-window spec delta to clarify manual fit shrinks the non-anchored dimension without upscaling.
- [x] 1.2 Fix the manual fit logic so the longer window side stays unchanged and the other side shrinks to maintain aspect ratio.
- [ ] 1.3 Manually verify Fit to Image on wide and tall images keeps the anchor dimension and does not widen the window.
- [x] 1.4 Run `openspec validate fix-fit-window-widening --strict` and note results.
