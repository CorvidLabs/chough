---
change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
artifact: tasks
---

# Tasks

- [x] Red tests in `tests/ready.rs` and `tests/package.rs`, committed failing.
- [x] `src/ready.rs`: the dry run, the two paths exceptions, the delta-set rule and the
      `.specsync/hashes.json` code removed.
- [x] The 6.4.0 pin in CI, `.github/scripts/`, `.specsync/config.toml`, `AGENTS.md`, the
      README, docs and `src/change.rs`.
- [x] `.gitattributes` without the spec line; `.specsync/sdd.json` without `specs/`.
- [x] Cases, a changelog fragment, and the full lane with the candidate.

After verification, outside this list: the tester's real-repository run of ready (the 66
changes, as for #406), and at 6.4.0's publish, the Action commit swapped for the
published tag's and the lane run again.
