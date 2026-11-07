---
change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
artifact: tasks
---

# Tasks

- [x] Add `timing_skips_direct_write_events_without_marking_the_report_partial` in `tests/direct_writes.rs`. It fails first: the report is partial with one unrecognised kind.
- [x] Skip `stage::DIRECT_WRITE` in `src/timing.rs`'s kind filter, without counting it.
- [x] Add `change_new_passes_no_spec_change_with_its_rationale` in `tests/change.rs`. It fails first, because the options don't exist.
- [x] Add `--no-spec-change` and `--rationale` to `chough change new` in `src/change.rs`, and the options to docs/cli.md.
- [x] Add CHANGELOG lines for both.

After `specsync change check` materializes the delta, not task boxes:
- record the case;
- run a mutation round;
- run `fledge run linux-verify`.
