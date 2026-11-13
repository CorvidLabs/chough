---
change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
artifact: tasks
---

# Tasks

- [ ] A failing test in `tests/vault.rs`: with `.grok` gitignored, `chough init --host grok` and a commit, a fresh clone's `library index --check` fails today.
- [ ] The fix in `src/vault.rs`: `rule_files()` drops the paths Git ignores; `docs/cli.md` and the CHANGELOG.
- [ ] Red then green recorded through Chough, and the full local suite.

Human gates and delivery, as plain text: Leif's review through `chough change ship`, then CI, then his merge.
