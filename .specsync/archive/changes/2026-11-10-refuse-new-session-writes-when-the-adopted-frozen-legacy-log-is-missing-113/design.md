---
change: refuse-new-session-writes-when-the-adopted-frozen-legacy-log-is-missing-113
artifact: design
---

# Design

## Existing behavior to preserve

Keep original adopting-session verification, changed-byte mismatches, independent cursor.legacy None in new sessions, valid never-had-log projects, current error family, existing bounds and safe local paths. Reuse the current supported holder locations: open/archive/shelved sessions and existing library attachment sessions. Do not copy the anchor into new sessions or rewrite legacy logs, checkpoints, events, native archives or retention proof.

## Proposed focused repair

Make remembered-anchor lookup reachable whenever the active cursor has no own legacy anchor, including when current log is absent. A bounded readable holder with a remembered anchor makes an absent current log a mismatch before event publication or checkpoint/cursor save. With a present log retain the existing matching-anchor behavior; with neither current log nor remembered anchor permit genuinely new projects. Reuse current supported directory enumeration and bounded_bytes/session cap, without adding a repository-wide scan or widening unreadable-holder policy. If implementation discovers a policy ambiguity about invalid/conflicting/unreadable holders, report it instead of inventing authoritative history or changing the scope.

An implementation may parameterize the existing adopted_anchor helper by optional current anchor, preserving matching lookup when present and returning a remembered anchor when absent. The acceptance behavior, not this proposed signature, binds scope. No all-records-invalid shortcut or schema migration is needed. Holder directories are test fixture arrangements only; real archived/source records remain immutable.

## Files

src/history.rs: verification and remembered-anchor lookup only. tests/sessions.rs: focused tests/helper only, all existing tests/assertions unchanged. .chough/cases/history.missing-log-113.cases.toml: new per-change definitions binding focused tests to existing REQ-history-002 and REQ-history-006, existing LIFE/DRIVE criteria. CHANGELOG.md: one accurate Unreleased bug entry. knowledge/INDEX.md: supported projection only; broader generation churn must be returned/raised. Own session, own exact history cycle and .chough/artifacts/issue-113/ hold only this change's records/results. Native workspace is tool-owned. No other production, policy, canonical requirements, HI, schemas or other issue source is included.
