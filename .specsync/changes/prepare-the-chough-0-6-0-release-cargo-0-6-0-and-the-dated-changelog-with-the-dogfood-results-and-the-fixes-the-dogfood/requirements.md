---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: requirements
---

# Requirements

Three requirements change, through this change's deltas:

- REQ-cli-061 (gate lists): the gate summary's "what changed" counts since the
  change's own base. The base is the change's `base_commit`, so an earlier change on
  the same branch is not counted again. It is the merge base with the default branch
  only when that merge base is not an ancestor of the `base_commit` (default-branch
  work merged or rebased in after the change began, #190) or when the change records
  no `base_commit`. A merge base that is HEAD itself is never used, and with no usable
  base the summary says the base is unknown. One acceptance criterion is added: a
  second native change on one branch counts only its own files.

- REQ-records-009 (migration): while any native change is not archived, the preview
  also says that `--apply` will refuse until it is, naming each such change; one
  acceptance criterion is added.
- REQ-history-006 (sessions): an active pointer that holds a session's alias or bare
  slug, as a 0.5.x pointer that an in-place migration keeps, names that session, and
  doctor does not call it stale (D1); one acceptance criterion is added.

The KNOW case links (KNOW-3, -4, -6, -15, -24) and #148 work within the contract.
