---
id: chough-doctor-finds-and-fixes-the-chough-debt-that-stops-a-fresh-worktree-from-entering-chough-drive-17
state: draft
type: feature
base_commit: 1030252cb5ca2e2b45b736833013f8cd5dbabab0
---

# Chough doctor finds and fixes the Chough debt that stops a fresh worktree from entering Chough (DRIVE-17)

## Intent

chough doctor finds and fixes the Chough debt that stops a fresh worktree from entering Chough (DRIVE-17)

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- DRIVE-17 (Leif, relayed by orc 2026-11-06, 'Capture + both defaults'), in the new REQ-cli-055 and REQ-history-012. chough doctor reports legacy_checkpoint, stale_pointer, finished_session, stale_gates and unfinished_session findings without a native read. No flag and --dry-run report only and change nothing. --fix applies the safe fixes: adopt the legacy checkpoint, remove a stale pointer, archive finished sessions byte for byte (a session with unfolded inbox entries is skipped). Decision fixes (withdraw stale gates, close sessions named by --close) need --reason; without it they are skipped. No human decision is ever recorded. Exit 0 only when nothing is left. The five new tests in tests/doctor.rs fail first; docs, the skill reference and CHANGELOG say so; fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
