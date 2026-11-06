---
change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
artifact: context
---

# Context

Issue #169. In the auditor's verify-lane run at 1030252c, `tests/timing.rs::timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit` failed with "traced and untraced reports differ". It passed on replay. The test compares a plain and a traced `timing --json` byte for byte. Each report ends its ongoing segments at its own `generated_at`, so the two match only when both runs land in the same second. Five retries make a mismatch less likely, but cannot rule it out on a loaded host.

Leif captured DRIVE-19 ("Capture both"), in the wording of spec-sync's CI-7: "A failing test always means something is actually wrong, never that it ran at an unlucky moment."

Issue #30 rules out a runtime clock switch, which could forge observation times, so the fix stays in the test.
