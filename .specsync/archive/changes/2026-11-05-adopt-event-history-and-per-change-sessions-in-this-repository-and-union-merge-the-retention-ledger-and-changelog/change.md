---
id: adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog
state: archived
type: migration
base_commit: b9b27c2191835b0185ad449f3cac95bc7be013dd
---

# Adopt event history and per-change sessions in this repository and union-merge the retention ledger and changelog

## Intent

Adopt event history and per-change sessions in this repository and union-merge the retention ledger and changelog

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- This repository runs Chough with event history and one session per change: the legacy transition log and migration backup stay byte for byte, the checkpoint moves to .chough/sessions/ byte for byte, and AGENTS.md tells agents to start each pull request with chough session new. The retention ledger and CHANGELOG.md merge with Git union. A simulated pair of parallel pull requests conflicts in .chough/ and CHANGELOG.md on main and merges with none of those conflicts on this branch; fledge lanes run verify passes on Linux.

## No-spec Rationale

Repository migration onto existing behavior (REQ-history-006, REQ-history-007, DRIVE-3): no requirement text changes.
