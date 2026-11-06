# Lesson bundle: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Record relayed human decisions at gates as claims, not authentication (123, D7)
- **Kind**: Feature
- **Specs**: cli, history
- **Paths**: .agents/skills/chough-workflow, skills/chough-workflow, src/checkpoint.rs, src/stage.rs, src/history.rs, src/main.rs, src/skill.rs, tests/relay.rs, tests/stages.rs, tests/history.rs, tests/package.rs, tests/workflow_authority.rs, docs/cli.md, docs/FRD.md, docs/architecture.md, docs/walkthrough.md, README.md, CHANGELOG.md, hi/flow.md, INTENT.md, .chough/cases/cli.relayed-decisions.cases.toml
- **Acceptance**: chough checkpoint --close-gate with --decided-by, --words, --where and --recorded-by closes only the named open gate, appends one gate-closed event carrying relayed_by with exactly those four values, and adds one decision marked a relayed claim, not authenticated, which chough next shows from a fresh process; every other gate stays open and still holds advance and skip.
- **Acceptance**: A missing or blank relay value, relay values without --close-gate, a gate text that is not open, and a decision over 4096 bytes each refuse with the checkpoint and history unchanged, and a test runs the recovery each refusal names and sees it succeed.
- **Acceptance**: A relayed decision leaves every SpecSync file byte-identical and never satisfies a native gate: with a draft change, advance to build stays refused after its scope gate is closed with a relay; --clear-awaiting-human still withdraws every gate with no relay and no decision.
- **Acceptance**: Transitions without relayed_by keep their exact bytes; a relayed gate-closed record reads back with the same four values from history events and from the legacy log; a relayed_by with a missing, blank, over-long or unknown key, or on any other kind, is refused as an invalid payload.
- **Acceptance**: The awaiting-human refusal and waiting action name --close-gate; README, the skill and its installed copy, docs/cli.md, docs/FRD.md and the cli spec's helper invariant describe approvals and reviews as recorded claims bound to their content, not authenticated identity. The new tests fail first, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `eb38f705adfe93cbd4cd3b12b53414ec15632bcf`
- Base commit: `ec830cfb001cc5f71f4f83809ca3294d685f42a4`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

## Audit finding D7 (chough#123)

The 2026-11-05 audit accepted a fabricated `audit-human` approval and an edited actor
label in SpecSync (spec-sync#633). Chough's guidance asks the agent to record the human
source of a decision, but Chough keeps nothing structured about who decided, so a gate
closed by an agent looks the same whether a human decided or not.

## Leif's decisions

- 95-plan, orc tab, 2026-11-05: "narrow the promise to what's true (approvals are
  recorded, digest-protected claims, not authentication) and record 'relayed by' for
  human decisions an agent records (who decided, their words, where, which agent
  recorded it)."
- 2026-11-05 20:18 MST, Herdr orc tab, relayed by orc t18: "Approve shape + FLOW-16".
  This approved:
  - the `relayed_by` shape `{decided_by, words, where, recorded_by}`, shared with
    SpecSync's side (ss-d7);
  - option A for `--clear-awaiting-human`: it stays for withdrawing gates and records
    no decision;
  - FLOW-16 as drafted. FLOW-10 is unchanged.

## Ruled out

- Authenticating identity or signing inside Chough: spec-sync#633 puts provenance in
  attest. The relay record is a claim, not proof.
- Refusing `--clear-awaiting-human` without a relay: offered, not chosen.

## From the change's design.md

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

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
