---
id: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
state: archived
type: bug_fix
base_commit: 9d53604071df50b537adccad0ccf5cac9a2fc32b
---

# 0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve

## Intent

0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- At the 400-observation cap, chough cases run, cases record, evidence add and checkpoint --observation each refuse before running or writing anything, naming the bound and chough session shelve <id>, with every file unchanged; and at accept next takes a case latest run only from the observations the checkpoint references, so an unreferenced newer entry does not hide a stale run (red then green)

## No-spec Rationale

A fix within the existing requirements; no contract text changes.
