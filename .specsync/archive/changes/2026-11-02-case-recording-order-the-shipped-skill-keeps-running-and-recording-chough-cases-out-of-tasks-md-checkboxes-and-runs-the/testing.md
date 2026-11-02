---
change: case-recording-order-the-shipped-skill-keeps-running-and-recording-chough-cases-out-of-tasks-md-checkboxes-and-runs-the
artifact: testing
---

# Testing

## Cases

| Case | Requirement | Criteria | Method | What it shows |
|---|---|---|---|---|
| TC-cli-skill-orders-the-change-check-before-new-requirement-cases | REQ-cli-009 | FLOW-7, LIFE-6, LIFE-2 | agent-assisted, reviewer-note | At the implementation tip the canonical plan, testing and accept references say the scoped check precedes cases linking a requirement the change adds and that running or recording cases is not a `tasks.md` checkbox; no dash; `chough skill status` reports both host copies current |
| TC-cli-0094 (existing) | REQ-cli-005 | as defined | automated | The refusal and check-first hint are unchanged |

The new case is added only after the scoped check materializes the delta, which is
the order this change documents. The package tests (automated, in the full suite)
cover the known-outdated updates and the guidance being present.

## Reproduction

A disposable copy of `examples/shipping`, driven like `examples/feature-bug-loop.sh`
up to build, with one extra unchecked "Record Chough cases" box: `cases check` and
`change check` are expected to exit 1; with the box removed, `change check` exits 0,
the requirement is materialized and `cases check` exits 0. The log is kept as an
artifact of the observation linked to BUG-0017; BUG-0017 is verified only by a pass
observed after its failure.

## Regression

The full suite, `chough cases check` and `fledge lanes run verify` on Linux with
Fledge 1.8.0, SpecSync 6.1.2 and hi 0.8.0. Hosted CI (Ubuntu, macOS, Windows, MSRV)
is reported only as observed; the Archive job blocks while this change is open.
