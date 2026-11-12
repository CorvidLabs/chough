---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: requirements
---

# Requirements

One requirement changes, through this change's delta (`deltas/cli.md`):

- REQ-cli-061 (gate lists): the gate summary's "what changed" counts since the
  change's own base. The base is the change's `base_commit`, so an earlier change on
  the same branch is not counted again. It is the merge base with the default branch
  only when that merge base is not an ancestor of the `base_commit` (default-branch
  work merged or rebased in after the change began, #190) or when the change records
  no `base_commit`. A merge base that is HEAD itself is never used, and with no usable
  base the summary says the base is unknown. One acceptance criterion is added: a
  second native change on one branch counts only its own files.

Every other fix here works within the current contract: D1 (REQ-records-009, every
record and alias readable after a migration), the migrate preview's warning
(REQ-records-009 already refuses the apply while a native change is open; the preview
now says so first), the KNOW case links (KNOW-3, -4, -6, -15, -24) and #148.
