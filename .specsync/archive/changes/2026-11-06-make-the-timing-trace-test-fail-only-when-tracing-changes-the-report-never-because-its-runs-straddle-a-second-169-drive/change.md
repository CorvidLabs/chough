---
id: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
state: archived
type: bug_fix
base_commit: 167790489735b3e6a88ec21a20bb2530d4c96138
---

# Make the timing trace test fail only when tracing changes the report, never because its runs straddle a second (169, DRIVE-19)

## Intent

Make the timing trace test fail only when tracing changes the report, never because its runs straddle a second (169, DRIVE-19)

## Affected Canonical Specs

- `reporting`

## Acceptance Criteria

- DRIVE-19 (Leif, relayed by orc 2026-11-06, 'Capture both', then 'Record both, reword the AC'; chough#169). REQ-reporting-002's trace criterion is reworded from 'byte-identical to the untraced run' to 'identical to the untraced run except for the clock'; no other text changes. timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit checks that criterion. It runs plain and traced timing --json once each, with no retry, and they must be equal except that every time and span running to now is later by exactly the gap between the two runs' generated_at. With a native stub that makes each run take over a second, the old comparison fails every time and the new one passes. A mutated field or a wrong shift fails the comparison. No other test, gate, bound or timeout changes.

## No-spec Rationale

Not applicable
