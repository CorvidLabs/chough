---
change: show-a-failed-native-command-as-exit-n-or-killed-by-signal-n-instead-of-rust-debug-text-issue-87
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/87 (field test 2026-11-03, chough
a33c747): a failed native read is shown as `specsync change status does-not-exist --json
(failed): exit=Some(1)`, Rust's debug form of an optional exit code. Expected: plain text
such as `exit 1` or `killed by signal N`. The unknown ID in the issue's repro is #63,
fixed by #71; the text is still produced on main 91a1fdd by `failure` in src/records.rs
for any failed native command. tests/cli.rs asserted the debug form (`exit=Some(2)`);
that expectation is corrected with this fix.

Criteria: LIFE-7. Authority: Leif's 2026-11-04 standing approval for bounded Chough bug
fixes and his brief to fix issues 81 to 88 one PR each.
