---
id: pr-50-evidence-fix-record-a-fresh-clean-tree-verify-run-as-the-current-agent-assisted-proof-in-place-of-the-mis
state: implementing
type: bug_fix
base_commit: ba3691cd54b8f776abdd3f9dde9c2fd023b04a50
---

# PR 50 evidence fix: record a fresh clean-tree verify run as the current agent-assisted proof in place of the mis-snapshotted corrective observation

## Intent

PR 50 evidence fix: record a fresh clean-tree verify run as the current agent-assisted proof in place of the mis-snapshotted corrective observation

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- fledge lanes run verify passes on a clean tree at a named committed revision, and a small new artifact log header names that exact revision
- A new agent-assisted product-test observation cites that log and is bound to a clean snapshot at that revision
- Its limitation says OBS-product-test-035959f28aa76d052d48fe78 wrongly carried the 250d637 snapshot for the a788869 log and that the new record replaces it as current proof
- OBS-human-observation-174caaa1021cfdf5a553f7a3, OBS-product-test-035959f28aa76d052d48fe78 and issue-49-verify.log stay byte-identical

## No-spec Rationale

Evidence records only; no requirement, criterion or code change
