---
id: keep-the-archived-session-that-holds-the-legacy-history-anchor-out-of-the-roll-up-143
state: archived
type: bug_fix
base_commit: f35f721ae0049cd6247119b6ff1085108fa4494f
---

# Keep the archived session that holds the legacy history anchor out of the roll-up (143)

## Intent

Keep the archived session that holds the legacy history anchor out of the roll-up (143)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- KNOW-29 as amended by Leif on 2026-11-06 (HI captured in hi/knowledge.md) and REQ-history-011: the archived session whose history cursor holds the legacy anchor is never rolled into the vault and does not count toward the five; with seven archived sessions where only the one that would roll first holds the anchor, the move into done keeps it and the five newest others and rolls only the remaining one. The KNOW-29 fixture models a single anchor holder; the new test fails first, no other assertion changes, docs/cli.md says so, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
