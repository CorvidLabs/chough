---
change: external-evidence-artifacts-by-recorded-fingerprint-never-copied-into-the-project-14
artifact: plan
---

# Plan

## Approach

`evidence::artifact_fingerprints` and the freshness re-read accept absolute paths, hashing them in place (64 MiB bound, plain files only); `evidence add`'s canonical-spelling check admits absolute paths. REQ-cli-006 is amended by the drafted delta.

## Out of scope

`~` expansion, copying or syncing external files into the repository, Git LFS, and changes to the automated run's own capture artifact (still `.chough/artifacts/<observation-id>.log`).

## Steps

1. Draft the REQ-cli-006 delta (done).
2. Implement with a RED-first regression (done).
3. Verify with change check --commit, refresh knowledge, open the PR for acceptance.

## Risks

An external file can change or vanish after recording; the recorded fingerprint detects that on freshness reads but bytes are not retained — that is the approved trade of fingerprint-instead-of-copy.

## Constraints consulted

specs/cli lessons; REQ-cli-042 (native sources at full size) untouched: the 64 MiB bound is Chough's own record rule.
