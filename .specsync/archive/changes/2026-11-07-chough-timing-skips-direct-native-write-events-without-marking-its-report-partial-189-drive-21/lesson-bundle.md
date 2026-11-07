# Lesson bundle: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: chough timing skips direct-native-write events, and chough change new passes --no-spec-change (189, DRIVE-21, DRIVE-20)
- **Kind**: BugFix
- **Specs**: cli, history, reporting
- **Paths**: docs/cli.md, tests/change.rs, src/change.rs, src/timing.rs, tests/direct_writes.rs, CHANGELOG.md
- **Acceptance**: REQ-history-013 (modified): chough timing reads a history holding a direct-native-write event as complete, skipping it as part of no stage segment, with no unrecognised kind and no partial mark; truly unknown kinds still mark the report partial. The new test in tests/direct_writes.rs fails first, a mutation that counts the kind again fails it, and fledge run linux-verify passes.

## Evidence

- Verification commit: `50fba3bd3555bf954cd51779f4d2705e1baf3297`
- Base commit: `f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8`
- Verified by: `specsync check --spec cli --spec history --spec reporting`

## From the change's context.md

# Context

Issue #189, a regression from #183 (#186, DRIVE-21). `src/timing.rs` keeps the stage kinds plus `gate-opened` and `gate-closed`, and counts every other kind as unrecognised. When that count is above 0, it marks the timing report partial ("N records of unrecognised kinds skipped").

So any history holding a valid `direct-native-write` event (REQ-history-013) now gets a partial timing report. Chough's own main holds such events since #186 and #188. `src/timing.rs` was outside #183's approved paths, so the gap was noted in #186 and filed as #189.

## Decisions

- **The fix is small and in scope for DRIVE-21.** Timing treats `stage::DIRECT_WRITE` as a known kind that starts or ends no stage segment, as the gate kinds already are. Other unknown kinds are still counted and still mark the report partial.
- **REQ-history-013 is modified** to say so, with a criterion that `chough timing` reads such a history as complete.
- **Folded in at the orc's request (2026-11-07):** ch-docs-accuracy found that `chough change new` cannot pass `--no-spec-change`, so a no-spec change still needed a direct `specsync change new`. `chough change new` gains `--no-spec-change` and `--rationale`. REQ-cli-057 is modified for them, and `--no-spec-change` without `--rationale` is refused before SpecSync runs.
- **Owning module:** the first approval was refused with "no declared module canonically owns this affected path: src/timing.rs". `src/timing.rs` is in the reporting spec, so `reporting` joins the definition with the minimal delta. REQ-reporting-001's criterion "gate records start or end no segment" now covers `direct-native-write` events too.
- **Ruled out:** showing direct writes in timing's output. Timing reports durations, and direct writes are not segments.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
- `specs/reporting/context.md`
