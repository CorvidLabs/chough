---
id: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
state: approved
type: feature
base_commit: ec830cfb001cc5f71f4f83809ca3294d685f42a4
---

# Record relayed human decisions at gates as claims, not authentication (123, D7)

## Intent

Record relayed human decisions at gates as claims, not authentication (123, D7)

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- chough checkpoint --close-gate with --decided-by, --words, --where and --recorded-by closes only the named open gate, appends one gate-closed event carrying relayed_by with exactly those four values, and adds one decision marked a relayed claim, not authenticated, which chough next shows from a fresh process; every other gate stays open and still holds advance and skip.
- A missing or blank relay value, relay values without --close-gate, a gate text that is not open, and a decision over 4096 bytes each refuse with the checkpoint and history unchanged, and a test runs the recovery each refusal names and sees it succeed.
- A relayed decision leaves every SpecSync file byte-identical and never satisfies a native gate: with a draft change, advance to build stays refused after its scope gate is closed with a relay; --clear-awaiting-human still withdraws every gate with no relay and no decision.
- Transitions without relayed_by keep their exact bytes; a relayed gate-closed record reads back with the same four values from history events and from the legacy log; a relayed_by with a missing, blank, over-long or unknown key, or on any other kind, is refused as an invalid payload.
- The awaiting-human refusal and waiting action name --close-gate; README, the skill and its installed copy, docs/cli.md, docs/FRD.md and the cli spec's helper invariant describe approvals and reviews as recorded claims bound to their content, not authenticated identity. The new tests fail first, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
