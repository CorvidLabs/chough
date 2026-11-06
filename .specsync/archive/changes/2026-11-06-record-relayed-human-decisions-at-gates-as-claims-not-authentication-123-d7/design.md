---
change: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
artifact: design
---

# Design

- **`stage.rs`:**
  - `Transition` gains `relayed_by: Option<RelayedBy>`, with `skip_serializing_if`.
  - `RelayedBy { decided_by, words, where (serde rename), recorded_by }` denies
    unknown fields, and `RelayedBy::validate` checks each value is non-blank and at
    most 4096 bytes.
  - `log_gate` takes an optional relay. Only `gate-closed` accepts one.
  - The `refused:awaiting-human` message and the waiting action name `--close-gate`.
- **`history.rs`:** `validate_payload` refuses `relayed_by` on any kind but
  `gate-closed`, and refuses invalid values. The legacy-log reader applies the same
  check.
- **`checkpoint.rs`:**
  - `CheckpointArgs` gains `--close-gate`, `--decided-by`, `--words`, `--where` and
    `--recorded-by`. `--close-gate` conflicts with `--clear-awaiting-human`,
    `--awaiting-human` and `--merge`.
  - The relay checks run before any write, in this order: without-gate, incomplete,
    gate-not-open, too-long.
  - The gate is removed from `awaiting_human`, the decision line is pushed, and the
    gate-closed event is logged with the relay, in the existing save/log order.
  - The output gains `relayed_decision: {gate, relayed_by, provenance}`.
- No SpecSync command runs, and stage gates are not read or changed.
