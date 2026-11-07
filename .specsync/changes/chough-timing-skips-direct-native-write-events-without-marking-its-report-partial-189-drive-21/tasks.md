---
change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
artifact: tasks
---

# Tasks

- [ ] Add `timing_skips_direct_write_events_without_marking_the_report_partial` in `tests/direct_writes.rs`. It fails first: the report is partial with one unrecognised kind.
- [ ] Skip `stage::DIRECT_WRITE` in `src/timing.rs`'s kind filter, without counting it.
- [ ] Add a CHANGELOG line.

After `specsync change check` materializes the delta, not task boxes:
- record the case;
- run a mutation round;
- run `fledge run linux-verify`.
