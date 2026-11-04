---
id: record-a-stage-gate-that-already-happened-with-its-original-time-quoted-evidence-and-a-backfilled-flag-that-timing
state: archived
type: feature
base_commit: d673efc9fadd00953cf9e42dad96e08f3c76103c
---

# Record a stage gate that already happened with its original time, quoted evidence and a backfilled flag that timing reports separately (DRIVE-5)

## Intent

Record a stage gate that already happened with its original time, quoted evidence and a backfilled flag that timing reports separately (DRIVE-5)

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- Source: Leif's brief t811u item 5, "Want: `chough stage backfill` (or similar) that records a past gate with its original time, the approval evidence (commit, quoted choice), and a backfilled flag that `timing` reports separately.", quoted as DRIVE-5 and confirmed as his criteria at 5:24 PM MST (card t811s9271); the parent relayed at 6:03 PM MST that the hard rule holds: it records a past gate's original time, the quoted evidence and a backfilled flag that timing reports separately, never creates a human approval or review record, and refuses if the native state doesn't support the gate. Outcomes: (1) chough stage backfill --at <UTC time or unix seconds> --commit <rev> --quote <text> [--note <text>] makes the move advance would make now, to the next main stage, and records it as an advance transition whose time is the given past time and which carries backfill {recorded_at_unix_seconds, commit (full SHA), quote}; (2) it applies every refusal advance applies (inactive or side stage, open awaiting-human gate, blocking bugs and unlinked failures leaving test, and the destination's native prerequisite read now), so native state still decides the gate; (3) it refuses with nothing recorded a time in the future, a time before the latest recorded transition, an unreadable latest transition, a commit that does not resolve or is not an ancestor of HEAD, a blank or oversized quote, and the release to done move; (4) it records no human approval, review, consent or awaiting-human gate, and its output says the quote is evidence only; (5) timing lists backfilled moves separately with their recorded time, commit and quote, marks segments a backfilled move bounds, reports their backfilled time per stage, and leaves them out of estimate samples; (6) records without backfill keep their exact bytes, and the legacy log and history events both carry it; (7) docs, CHANGELOG and the lifecycle skill reference describe it, and `chough skill update` replaces an unedited v0.2.0 lifecycle reference with it; (8) the new tests fail before the change and fledge lanes run verify passes

## No-spec Rationale

Not applicable
