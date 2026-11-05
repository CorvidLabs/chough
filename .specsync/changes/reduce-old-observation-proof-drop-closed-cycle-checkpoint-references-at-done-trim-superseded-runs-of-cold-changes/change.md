---
id: reduce-old-observation-proof-drop-closed-cycle-checkpoint-references-at-done-trim-superseded-runs-of-cold-changes
state: approved
type: feature
base_commit: 5326ec48b4ae856543e4d8cabd7c06b29daef0e5
---

# Reduce old observation proof: drop closed-cycle checkpoint references at done, trim superseded runs of cold changes, attribute unlisted proof to its change by cycle time, and read archive validity in parallel

## Intent

Reduce old observation proof: drop closed-cycle checkpoint references at done, trim superseded runs of cold changes, attribute unlisted proof to its change by cycle time, and read archive validity in parallel

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- At done with the librarian on, checkpoint observation references from before the current cycle are dropped and reported. The post-archive summary trims superseded routine runs of a case and environment listed by a cold change, keeping the newest run and the newest failure, and trims proof no archive lists that was observed in a cold change cycle, ledgering it with that change and summarizing it in its knowledge entry; proof from a hot change cycle or after the newest archive stays. Archive validity is read in parallel. New tests fail first, the trim on this repository is measured, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
