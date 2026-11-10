---
id: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
state: implementing
type: bug_fix
base_commit: 86ae8dda94aba981094e92d5395dc085ebe65dbd
---

# Make native process test probes zombie-aware for #187 and preserve the #132 reproduction limit

## Intent

Make native process test probes zombie-aware for #187 and preserve the #132 reproduction limit

## Affected Canonical Specs

- `cli`
- `reporting`

## Acceptance Criteria

- A new regression first fails when the shared probe uses only kill -0 on a killed, unreaped child. After the fix, the bounded common::alive probe reports that zombie as not alive, still reports a live sleep 60 child as alive, and is used by tests/timing.rs, tests/native.rs and tests/change.rs. Focused regressions and stressed repetitions pass on macOS and Linux, and fledge run linux-verify passes. src/checkpoint.rs and its lock test stay unchanged: #132 remains open with the handoff's 0/40 Linux rounds and 0/30 loaded rounds, with no speculative diagnostics or fix. No product behavior, HI wording or requirement text changes.

## No-spec Rationale

Test-only correction of process liveness probes under existing DRIVE-19 and native timeout requirements; no product behavior or requirement text changes.
