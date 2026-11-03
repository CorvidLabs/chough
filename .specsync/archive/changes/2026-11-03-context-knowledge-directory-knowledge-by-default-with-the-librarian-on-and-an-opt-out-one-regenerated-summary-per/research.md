---
change: context-knowledge-directory-knowledge-by-default-with-the-librarian-on-and-an-opt-out-one-regenerated-summary-per
artifact: research
---

# Research

**Validity after removal.** On a scratch copy of main 0363d4a, removing `.chough/artifacts` and `.chough/evidence` from the working tree left the archive `windows-safe-git-paths-in-the-issue-30-same-second-test` at terminal evidence validity `authenticated-history`. Archived changes therefore authenticate from history. The existing retention code relies on the same behavior: only the latest archive protects, and older archives authenticate from history.

**Probe cost.** `specsync change status <id> --json` took about 12 s per archive here, so the apply step probes only the archives it affects.

**Local manifests.** `verification.json` `acceptance_manifest.entries` hold the same paths and payload digests as `acceptance_entries`. The plan reads them locally.
