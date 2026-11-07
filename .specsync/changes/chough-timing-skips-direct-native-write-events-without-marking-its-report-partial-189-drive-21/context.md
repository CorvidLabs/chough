---
change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
artifact: context
---

# Context

Issue #189, a regression from #183 (#186, DRIVE-21). `src/timing.rs` keeps the stage kinds plus `gate-opened` and `gate-closed`, and counts every other kind as unrecognised. When that count is above 0, it marks the timing report partial ("N records of unrecognised kinds skipped").

So any history holding a valid `direct-native-write` event (REQ-history-013) now gets a partial timing report. Chough's own main holds such events since #186 and #188. `src/timing.rs` was outside #183's approved paths, so the gap was noted in #186 and filed as #189.

## Decisions

- **The fix is small and in scope for DRIVE-21.** Timing treats `stage::DIRECT_WRITE` as a known kind that starts or ends no stage segment, as the gate kinds already are. Other unknown kinds are still counted and still mark the report partial.
- **REQ-history-013 is modified** to say so, with a criterion that `chough timing` reads such a history as complete.
- **Ruled out:** showing direct writes in timing's output. Timing reports durations, and direct writes are not segments.
