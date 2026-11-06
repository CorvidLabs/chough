---
id: advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into
state: approved
type: feature
base_commit: 260ecc949ea01214f4bec43b30ea4226ce8fae59
---

# Advance through several stages on one native read with stage advance --to, and read native status once on the move into done (DRIVE-9, DRIVE-2)

## Intent

Advance through several stages on one native read with stage advance --to, and read native status once on the move into done (DRIVE-9, DRIVE-2)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Under a per-change session, 'chough stage advance --to accept' from plan on an approved change runs specsync change status once, records three advance events with their gate labels in applied.steps and ends at accept, and a single-checkpoint project appends the same three transitions. From intent, '--to plan' makes no native read. A refused step (blocking bug, draft state, a target not ahead, a side or unknown target, an open human gate) refuses the whole batch with at most one read, nothing recorded and the session file and history byte-identical. On an archived change with an acceptance manifest and the librarian on, 'stage advance' from accept to done and '--to done' from test each run specsync change status exactly once and report librarian_trim; before this change, accept to done ran it twice. A plain 'stage advance' is unchanged. The new-behavior tests fail against main first and pass after, and fledge lanes run verify passes.

## No-spec Rationale

Not applicable
