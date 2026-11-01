---
change: make-chough-timing-read-each-native-change-at-most-once-skip-archived-history-the-requested-report-does-not-show-bound
artifact: design
---

# Design

- Request-local memo in `src/timing.rs`: a map from change ID to the `NativeJson`
  result, used only inside `native_section`. Nothing is written to disk or shared
  between commands.
- History selection: `native_section` receives the parsed filters. The new JSON
  fields are `native.history_selection = {state: "selected"|"skipped-by-filter",
  filters: [...], read, reused, skipped_beyond_bound, not_read_budget}`. When the
  selection skips history, `native.history` is `null`. `skipped_archived` keeps its
  current meaning when history is selected and is `null` when it is not. Text output
  gains one line naming the selection.
- Budget: `records.rs` gains `native_json_within(root, program, args, cap)`, which
  runs the existing `native_capture` with `min(per-read deadline, cap)`. The existing
  `native_json` and `native` keep their signatures and behavior, so the stage, next,
  status, evidence and context call sites are untouched. Timing tracks remaining
  budget with a monotonic clock.
- Trace: one `trace()` helper in `records.rs`, checked through `CHOUGH_TRACE` once
  per process, writing to stderr only. It is called from `native_capture` and from
  the timing phases (transitions, tests, validation runs, native, render).
- Rejected alternatives: caching native responses across commands (stale
  approval/verification risk, invalidation scope); parallel native reads (SpecSync
  concurrency on one repository is unqualified and would multiply Git load); changing
  next/status/gates (their single read is already the minimum and must stay
  authoritative).
