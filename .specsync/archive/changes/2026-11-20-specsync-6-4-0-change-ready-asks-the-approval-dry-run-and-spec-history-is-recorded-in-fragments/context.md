---
change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
artifact: context
---

# Context

Three issues, one move to SpecSync 6.4.0, drafted on its first release candidate,
v6.4.0-rc.1, so it is ready when 6.4.0 publishes.

- #404: adopt the next SpecSync. Move the pin; drop the `specs/**/*.spec.md
  merge=specsync` line, its docs and the advice to rebuild a spec's Change Log by hand,
  since 6.4.0 records each change in its own history fragment
  (`specs/<module>/history/<change-id>.md`) instead of a Change Log row and a `version:`
  bump. Its item 2, accepting a delta SpecSync rebased (spec-sync #1089), waits:
  #1089 is still an open pull request and not in the candidate.
- #406, the rest of it: `chough change ready` says "Nothing is missing." for drafts that
  `specsync change approve` then refuses (25 delta shapes, a delta with no SHALL, a
  declared spec with no spec file, an accepted change, an approval refusal ready does not
  know). Leif's decision, "Ask SpecSync itself": ready calls SpecSync's read-only dry run
  of the approval (spec-sync #1091, in 6.4.0) and does not copy its rules. Also from
  #406: SpecSync's own untracked files, and the work of another open change on the same
  branch, are named as outside the declared paths.
- #307, folded into #406 by Leif's decision of 2026-11-19: an affected path no declared
  module owns, which the dry run's `ownership` check names.

Leif also chose to drop `change ready`'s handling of `.specsync/hashes.json` (making the
file before `specsync check` runs and restoring it afterwards).

Constraints: every machine that runs change commands in this repository uses the same
SpecSync, because 6.3.0's `change check` on a change 6.4.0 recorded adds a Change Log row
that 6.4.0 then refuses as a double record (6.4.0's own release notes).
