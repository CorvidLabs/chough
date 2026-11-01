---
change: run-the-specsync-archive-gate-before-every-ci-job
artifact: context
---

# Context

CorvidLabs requires every SpecSync change to be archived before a pull request merges, and CI
must not run anything until then. SpecSync's GitHub Action gained an archive gate in
CorvidLabs/spec-sync#832 (`archive-gate-only`), merged as `106aaa43d0bcf049f19b173ddb63815a25c3f931`.
This repository's existing `SpecSync Archive` job ran in parallel with the product jobs, so
product CI ran while a change was still open. Approved by 0xLeif as part of the org-wide rollout.
