---
change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
artifact: design
---

# Design

Nothing visual changes. `changed_files` in `src/ready.rs` keeps its rule and adds one
filter: a committed file whose blob at HEAD equals its blob at the default branch's tip
or at HEAD's merge base with it is left out.

- The default branch: the remote-tracking ref `refs/remotes/origin/HEAD` names, unless
  it names no existing ref or HEAD's own branch (by name or as its upstream); then
  `refs/remotes/origin/main`; a local `main` only with no remote at all. These are the
  choices spec-sync #1081 settled this week.
- No filter while HEAD is on the default branch, its tip is HEAD, or there is none, so
  work done on the default branch itself is counted from `base_commit` as before.
- A shallow clone without the merge base: only the tip is compared, said on standard
  error, no Git error printed.
- A base the repository lacks: named plainly, and the count runs from the merge base;
  with no default branch or merge base, one finding that the files could not be listed
  (`git fetch --unshallow` in a shallow clone, `chough change status` otherwise), never
  "Nothing is missing."
