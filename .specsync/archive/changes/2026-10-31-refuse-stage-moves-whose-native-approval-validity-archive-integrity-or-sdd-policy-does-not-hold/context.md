---
change: refuse-stage-moves-whose-native-approval-validity-archive-integrity-or-sdd-policy-does-not-hold
artifact: context
---

# Context

A review of Chough after PR #2 found three gate defects, reproduced on main `f19823d`
against SpecSync 6.1.1 in disposable copies of `examples/shipping` (synthetic
approvals only). They are BUG-0006, BUG-0007 and BUG-0008, each with a failed
observation and a reproduction log under `.chough/artifacts/`.

- BUG-0006: `stage.rs` gates build on `summary.state` alone. With state `approved` and
  `approval_valid: false` (scope changed after approval), plan -> build was allowed as
  `native:approved`.
- BUG-0007: `reset` accepts any main stage and gates only `build`, so `reset --to done`
  and `--to accept` succeeded while the change was draft.
- BUG-0008: `sdd_enabled` turns a parse failure into `Value::Null`, so truncated JSON,
  `"enabled": "yes"` and a missing `enabled` all became `ungated:sdd-disabled`.
  SpecSync itself rejects the first two as an invalid policy and reads a missing
  `enabled` as true (`#[serde(default)]`, default enabled).

Native interface, investigated in spec-sync `f240d37`: `change status <id> --json`
recomputes `approval_valid` from the scope digest on every call, but does not compare
approved delta bodies (BUG-0002, upstream; a separate SpecSync change). For archived
changes, `summary.terminal_evidence.validity` is `authenticated-history` (or `exact`)
when the archive authenticates and `corrupt-history` when it does not; `stale` applies
only to accepted changes. Frozen archives are not re-tested against later HEADs.

Out of scope: process bounds, validation stop timing and native read bounds (BUG-0009
to BUG-0011, a separate change), and SpecSync's own behavior.
