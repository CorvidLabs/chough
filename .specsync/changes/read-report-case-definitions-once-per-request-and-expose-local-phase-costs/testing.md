---
change: read-report-case-definitions-once-per-request-and-expose-local-phase-costs
artifact: testing
---

# Verification

New portable regressions exercise one suite load despite multiple observations,
case bindings and reference labels; zero loads without bound cases; exact unchanged
legacy/current digests; malformed/duplicate cases failing visibly; and new requests
observing changed case files, source content and artifact bytes. Existing environment
mismatch and native gate tests remain intact. Trace-on/off runs use fixed fixtures
and compare stdout, records and exit results, including failure paths.

Proposed bound cases: TC-cli-report-case-read-budget and
TC-cli-report-read-invalidation, linked to REQ-cli-028 after materialization. A
trace-parity regression also covers next/status and timing diagnostic phases.

Before/after profiling uses an identical frozen main root with old/new freshly
built debug binaries. Capture SHA-256, revision, environment, input sizes, flags,
stdout bytes, per-call native state/call counts and wall time. Label censored reads
and first-process versus repeated runs honestly. Retention preview is read-only.

Required final checks: Fledge fmt, test, lint and the mandatory verify lane; full
Rust 1.88 suite; all selected executable cases; native committed verification after
source plus .chough records are committed. Hosted platforms are reported only when
observed. Native review/ship, merge and live actions are not verification tasks.
