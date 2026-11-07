---
change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
artifact: testing
---

# Testing

- `timing_skips_direct_write_events_without_marking_the_report_partial` (`tests/direct_writes.rs`): with one recorded `direct-native-write` event, `chough timing --json` has no "unrecognised kinds" partial entry, and every `unrecognised_kinds` count is 0. It fails first: the report says "1 records of unrecognised kinds skipped".
- `change_new_passes_no_spec_change_with_its_rationale` (`tests/change.rs`): `chough change new d --spec cli --no-spec-change --rationale "why"` runs `specsync change new d --spec cli --no-spec-change --rationale why --json`. `--no-spec-change` alone is refused with nothing run, naming `--rationale`, and the named command then succeeds. It fails first, because the options don't exist.
- **Mutations:** counting `direct-native-write` as unrecognised again fails the timing test, and dropping `--rationale` from the argv fails the change test.
- **Regression:** an event of a truly unknown kind still marks the report partial. The existing timing tests keep that behavior, and the suite runs them.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-013 (modified) | tests/direct_writes.rs |
| REQ-reporting-001 (modified) | tests/direct_writes.rs (the timing test) |

## Lanes

`fledge run linux-verify` passes; the local spec check runs first with `--require-coverage 100`.
