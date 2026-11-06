---
id: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
state: approved
type: bug_fix
base_commit: 167790489735b3e6a88ec21a20bb2530d4c96138
---

# Make the timing trace test fail only when tracing changes the report, never because its runs straddle a second (169, DRIVE-19)

## Intent

Make the timing trace test fail only when tracing changes the report, never because its runs straddle a second (169, DRIVE-19)

## Affected Canonical Specs

- `reporting`

## Acceptance Criteria

- DRIVE-19 (Leif, relayed by orc 2026-11-06, 'Capture both'; chough#169): timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit fails only when tracing changes the report. It runs the plain and traced timing --json once each, with no retry, and requires them to be equal except that every time and span running to now is later by exactly the gap between the two runs' generated_at. With a native stub that makes each run take over a second, today's test fails every time and the fixed test passes. A mutated field or a wrong shift makes the comparison fail. No other test, gate, bound or timeout changes.

## No-spec Rationale

Test-only: the timing trace test compared two reports that end ongoing segments at their own now. No product behavior or contract changes.
