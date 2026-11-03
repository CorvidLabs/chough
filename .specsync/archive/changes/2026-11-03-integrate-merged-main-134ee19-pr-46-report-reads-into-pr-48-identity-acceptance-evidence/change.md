---
id: integrate-merged-main-134ee19-pr-46-report-reads-into-pr-48-identity-acceptance-evidence
state: archived
type: operations
base_commit: 781ee446db654bd159295a48fdbee25cebb26cb1
---

# Integrate merged main 134ee19 (PR 46 report reads) into PR 48 identity acceptance evidence

## Intent

Integrate merged main 134ee19 (PR 46 report reads) into PR 48 identity acceptance evidence

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The checkpoint is a three-way merge against base 16fe55d: 197 observation pointers, 37 file refs, about 64466 bytes; PR #46 removals and its 24 direct evidence-file pins kept; no union resurrection
- Transitions keep main lines in order plus PR #48 additions
- PR #48 version-history row becomes 25 after main row 24
- The PR #48 same-second test reads freshness through the renamed request-local reads without weakening its assertion
- Existing identity observations stay as history; the combined source is requalified with fresh observations
- fledge lanes run verify passes

## No-spec Rationale

Integration of accepted changes only: checkpoint reconciliation, version-history row renumbering and a test-only call update; no requirement or criterion changes
