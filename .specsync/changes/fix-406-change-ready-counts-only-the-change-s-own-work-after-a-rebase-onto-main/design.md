---
change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
artifact: design
---

# Design

Nothing visual changes. `changed_files` in `src/ready.rs` keeps its rule and adds one
filter: a committed file whose blob at HEAD equals its blob at the default branch's tip
or at HEAD's merge base with it is left out. The default branch is the ref
`refs/remotes/origin/HEAD` names, else `main`; with neither, nothing is left out. A base
the repository lacks is named in one plain finding line, and the count runs from the
merge base instead.
