---
change: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
artifact: testing
---

# Testing

Product tests (fail first on main):
- tests/relay.rs:
  - closing one of two gates records the relay, and the other gate still holds advance;
  - a fresh `next` shows the decision;
  - each refusal leaves the checkpoint and history unchanged, and its named recovery
    then succeeds;
  - with a draft native change, SpecSync files are byte-identical and the build gate
    still refuses;
  - `--clear-awaiting-human` writes no relay;
  - closing the only gate lets advance apply;
  - the legacy log and history events round-trip the relay;
  - invalid payloads are refused.
- tests/stages.rs: the refusal and waiting action name `--close-gate`. The existing
  assertions stay, and the expected text gains the new command.

Cases: `.chough/cases/cli.relayed-decisions.cases.toml` binds the main tests, run locally
and on Linux (Docker, local-linux-aarch64).

Gates: `fledge lanes run verify` (fmt, clippy -D warnings, tests, hi, specsync strict,
audit, cases, knowledge, exercise, build). Hosted CI is observed only after finalize.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-050 | tests/relay.rs `a_relayed_decision_closes_only_its_gate_and_keeps_the_four_values`, `relay_refusals_save_nothing_and_their_recovery_succeeds`, `a_relayed_decision_never_touches_native_state_or_its_gate`, `clearing_records_no_relay_and_a_relayed_close_lets_advance_apply` |
| REQ-cli-030 | tests/stages.rs `open_human_gates_refuse_advance_and_skip` (the refusal names `--close-gate` and `--clear-awaiting-human`), `open_human_gates_leave_other_moves_and_agree_in_next` and `done_archived_change_recommends_new_cycle_and_preserves_precedence` (the waiting action names `--close-gate`); tests/relay.rs `clearing_records_no_relay_and_a_relayed_close_lets_advance_apply` (advance applies after a relayed close) |
| REQ-history-010 | tests/relay.rs `invalid_relays_are_refused_as_invalid_payloads` (legacy log and history events), `clearing_records_no_relay_and_a_relayed_close_lets_advance_apply` (no relay key on other records), `a_relayed_decision_closes_only_its_gate_and_keeps_the_four_values` (history event round trip) |
