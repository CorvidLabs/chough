---
id: reuse-specsync-s-authentication-of-an-unchanged-committed-archive-so-next-status-stage-moves-and-timing-skip-the-slow
state: implementing
type: feature
base_commit: d673efc9fadd00953cf9e42dad96e08f3c76103c
---

# Reuse SpecSync's authentication of an unchanged committed archive so next, status, stage moves and timing skip the slow live status read, and advance through several stages on one native read (DRIVE-2, DRIVE-9)

## Intent

Reuse SpecSync's authentication of an unchanged committed archive so next, status, stage moves and timing skip the slow live status read, and advance through several stages on one native read (DRIVE-2, DRIVE-9)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Source: Leif's brief t811u item 2, "Want: archived changes are read from their committed archive files (or a cache keyed by archive digest) without asking SpecSync for live status; one native status read per command at most; `next` answers within a few seconds on Raven. Measure before and after.", and item 7, "`stage advance` run three times in a row (plan to build to test to accept) each re-read native status; one read should serve a batch." Leif confirmed the brief as his criteria at 5:24 PM MST. Outcomes: (1) when SpecSync reports a committed archive as archived with terminal evidence authenticated-history, Chough records that result under the Git common directory, keyed by a SHA-256 digest of the archive's bytes, the legacy archive baseline, the native policy and config, and the SpecSync binary identity, pinned to the archive's committed Git tree and a commit still reachable from HEAD; only archives with an accepted-state snapshot and an acceptance manifest, which SpecSync judges from their own bytes and reachable Git history rather than live project files, are recorded; (2) chough next, status, stage moves and timing reuse a recorded result only when the digest matches, without running specsync, and the gate label native:archived-authenticated-cached and detail say what was proved, by which SpecSync, when, and for which digest; (3) any changed byte, policy, baseline or SpecSync binary, a malformed or mismatched entry, an active change of the same ID, a symlink, uncommitted archive bytes, a recorded commit no longer reachable from HEAD, or no Git directory falls back to the live native read, whose refusals are unchanged; a stale, corrupt-history or failed read is never recorded; (4) retention and approved-delta reads stay live; (5) chough stage advance --to <stage> moves through each main stage to the target on one native read, checks every step's gate before recording any transition, records one transition per step, and refuses the whole batch with nothing recorded when any step is refused; (6) next and status still make at most one native read, and a stage advance batch makes at most one; (7) Chough still refuses to record human approval or review; (8) the new tests fail before the fix; docs/cli.md, CHANGELOG.md and the cli spec describe the cache; fledge lanes run verify passes; on a Raven checkout next, status, stage advance and timing are timed before and after

## No-spec Rationale

Not applicable
