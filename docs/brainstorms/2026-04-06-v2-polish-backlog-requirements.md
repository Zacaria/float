---
date: 2026-04-06
topic: v2-polish-backlog
---

# V2 Polish Backlog

## Problem Frame
Float already delivers the core utility of an always-on-top image viewer, but the product still feels rough in places where users judge quality quickly: the settings surface looks functional rather than intentional, runtime chrome is too persistent, and empty or failed states are not polished enough.

V2 should make the current app feel complete and dependable without expanding scope into a larger feature set. The release focus is polish, coherence, and behavior that feels logical in day-to-day use, with a dedicated clean configuration window as the clear home for durable preferences.

## Requirements

**Settings Presentation**
- R1. The app must present a minimal but clearly polished settings experience with stronger grouping, spacing, labels, and overall visual hierarchy than the current implementation.
- R2. Settings must feel like the clear home for user-configurable behavior instead of leaving the product feeling split between temporary overlay controls and durable preferences.
- R3. The settings redesign must remain lightweight and aligned with Float's minimal product identity rather than turning into a heavyweight preferences system.
- R4. Settings must open in a dedicated separate native window rather than inside the image viewer viewport, so configuration never covers or competes with the active image.
- R5. Slideshow configuration must include an explicit timing control, because slideshow playback without a configurable interval is incomplete.
- R6. Window opacity must be user-configurable from the dedicated settings surface.
- R7. Opacity must be real native window transparency rather than a visual dimming effect applied only inside the app chrome.

**Contextual Runtime Chrome**
- R8. Runtime overlay chrome must hide when the user is inactive and reappear when the user interacts, so the image remains the primary focus.
- R9. Navigation controls such as Previous and Next must appear only when they are logically useful, including staying hidden for single-image sessions.
- R10. Runtime controls must avoid suggesting capabilities that are unavailable in the current session state.

**Multi-Window Integrity**
- R11. Opening or changing an image in one Float window must never replace the active image shown in another Float window.
- R12. Viewer session state such as active file selection must remain isolated per window, while only true user preferences are shared globally.

**Empty And Error States**
- R13. The app must provide a more intentional placeholder state when no file is selected, with clear guidance on what the user can do next.
- R14. The app must provide polished missing-file and failed-load states that feel deliberate rather than broken.
- R15. Empty and error states must match the same visual language as the rest of the v2 polish work.

**Scope Discipline**
- R16. V2 must stay focused on polish of existing behavior and avoid net-new convenience features unless they are required to complete the polish work already in scope.

## Success Criteria
- The settings surface feels intentionally designed rather than merely functional, with a clean dedicated window for durable configuration.
- Slideshow playback no longer feels half-finished because timing can be configured where users expect it.
- Opacity is configurable and reveals content behind the Float window instead of only dimming the app UI.
- The bottom overlay no longer lingers in a way that distracts from the image.
- Previous and Next controls do not appear in single-image sessions.
- Opening an image in one window does not disturb any other Float window.
- Empty, missing, and failed-load states feel clear and productized.
- The release reads to users as a refinement of the current app, not a grab bag of unrelated additions.

## Scope Boundaries
- No new feature bucket for v2 beyond polish work on the current image-viewing workflow.
- No expansion into larger product directions such as a broader asset manager, workflow tool, or major new viewing mode.
- No settings-system expansion into a complex preferences architecture unless required by the selected polish work.

## Key Decisions
- Polish-first release: v2 should optimize for finishing the current app, not widening scope.
- Settings-first priority: the primary v2 track is visual polish of the settings experience.
- Dedicated config window: slideshow timing and opacity belong in Settings, not as implied or missing runtime behavior.
- Separate window over inline modal: configuration must not open on top of the current image viewport.
- Real transparency: opacity must be implemented at the native window level, not with CSS-only dimming.
- Per-window isolation: viewer selection state is window-scoped and must not leak across windows.
- Contextual chrome: overlays should appear when useful, then get out of the way.
- Minimal aesthetic target: the redesign should feel more polished, but still small and restrained.

## Backlog Priorities
- P1. Dedicated clean settings window, including slideshow timing and real opacity controls.
- P1. Multi-window state isolation so each Float window preserves its own active image.
- P1. Contextual overlay behavior, including hiding Previous and Next for single-image sessions.
- P2. Empty, missing, and failed-load state polish.

## Dependencies / Assumptions
- The existing app behavior defined in `openspec/specs/settings-panel/spec.md`, `openspec/specs/file-selection/spec.md`, and `openspec/specs/display-image/spec.md` remains the functional baseline.
- Current runtime chrome behavior in `dist/index.html` is the main surface that will need behavioral and visual polish for overlay improvements.

## Outstanding Questions
- Should the dedicated settings window be singleton-app-wide or opened per viewer window while still editing shared preferences?
- Should blur remain exposed beside opacity on all platforms, or only where the native window stack can support it without misleading users?
