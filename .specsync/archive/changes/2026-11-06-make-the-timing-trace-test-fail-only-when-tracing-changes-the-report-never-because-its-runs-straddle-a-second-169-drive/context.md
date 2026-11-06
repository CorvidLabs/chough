---
change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
artifact: context
---

# Context

Issue #169. In the auditor's verify-lane run at 1030252c, `tests/timing.rs::timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit` failed with "traced and untraced reports differ". It passed on replay. The test compares a plain and a traced `timing --json` byte for byte. Each report ends its ongoing segments at its own `generated_at`, so the two match only when both runs land in the same second. Five retries make a mismatch less likely, but cannot rule it out on a loaded host.

Leif captured DRIVE-19 ("Capture both"), in the wording of spec-sync's CI-7: "A failing test always means something is actually wrong, never that it ran at an unlucky moment."

Issue #30 rules out a runtime clock switch, which could forge observation times, so the fix stays in the test.

Leif then asked to reword the criterion ('Record both, reword the AC'), relayed by the orc on 2026-11-06 with a send-back. REQ-reporting-002's acceptance criterion said the traced standard output is "byte-identical to the untraced run", which two runs in different seconds cannot be. It now says "identical to the untraced run except for the clock": this change's one MODIFIED line, in `deltas/reporting.md`.
