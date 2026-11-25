---
change: fix-441
artifact: testing
---

# Testing

## Observed baseline

The public CLI run at `0d5988f8` reproduced partial persistence at the actual Timeline and session-size limits. A below-limit control closed both. The three Chough evidence receipts link FLOW-20 and REQ-cli-061 and identify the synthetic gate fixtures.

## Acceptance

- **Timeline boundary.** Start at 4,095 prior events with two listed gates open. The grouped command would exceed 4,096. Assert a nonzero exit, the named cap and recovery, both gates still pending, and byte-identical session and Timeline. Repeat the refusal and compare bytes again.
- **Session boundary.** Use a fixture whose first closure fits below 1,048,576 bytes and whose full result exceeds it. Assert a nonzero exit, the named cap and recovery, both gates pending and equal bytes. Recover capacity in a fresh or clearable fixture, relist and retry so both close together exactly once.
- **Successful control.** Close two selected gates below both limits while a third stays open. Compare verbatim relay words, each `summary_digest` and the list to the original gate list. Confirm one decision and one close entry per selected gate. Retry the already-closed selection and assert no new entry and no re-execution.
- **Two sessions.** List one open gate in each of two sessions. Submit both numbers with one session selected. The documented `other-session` refusal leaves both gates pending and both files and Timelines unchanged. Confirm choosing two gates within the selected session can close them without editing the other session.
- **Native preflight and refusal.** Run a valid mixed group from one session with one configured native driver and one non-native gate. A failing full local preflight results in zero native calls. A native refusal invokes the expected configured actor at most once and leaves both Chough gates pending with unchanged bytes. A retry after clearing a genuine native refusal may run the step and publish both closures.
- **Native success and interrupted local save.** Check one native invocation, both closures and each relay on success. Inject a local publication failure after a successful synthetic native step. Both Chough gates stay pending and the native operation remains recorded. A retry must preserve that native receipt, either by using an authoritative match or refusing for explicit reconciliation without invoking the native step again.
- **History and policy controls.** Check earlier direct native writes appear once before both closures and the grouped native step is not logged as a direct write. Keep single-close tests, existing stale-list, multi-native and verifying refusals, and existing `--no-native` semantics.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-061 | Public `gate record` control and boundary, two-session and mixed-native regressions in `tests/gates.rs` |
| REQ-cli-050 and REQ-cli-060 | Existing single-close tests and new grouped relay and direct-write history assertions |
| REQ-history-003 and REQ-records-001 | Exact Timeline event count and session bytes in the boundary tests |
