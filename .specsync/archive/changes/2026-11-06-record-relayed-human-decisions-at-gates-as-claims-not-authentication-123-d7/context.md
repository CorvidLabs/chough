---
change: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
artifact: context
---

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
