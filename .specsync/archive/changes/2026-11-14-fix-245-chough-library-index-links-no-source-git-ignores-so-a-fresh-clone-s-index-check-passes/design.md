---
change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
artifact: design
---

# Design

`rule_files()` in `src/vault.rs` drops the paths Git ignores, using one batched
`git check-ignore --stdin -z`. Outside a Git repository nothing is dropped. The index
then lists the same sources in a working tree and in a fresh clone. REQ-cli-043 says
the index links the rule, skill, decision and policy files that exist and that Git
doesn't ignore. `docs/cli.md`'s index row and the CHANGELOG say so.
