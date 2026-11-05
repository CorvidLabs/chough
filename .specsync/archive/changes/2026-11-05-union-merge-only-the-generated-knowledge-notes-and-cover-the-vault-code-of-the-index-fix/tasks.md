---
change: union-merge-only-the-generated-knowledge-notes-and-cover-the-vault-code-of-the-index-fix
artifact: tasks
---

# Tasks

- [x] Add the failing test: `git check-attr` must give `union` to generated notes only.
- [x] Scope the union driver to INDEX.md, rules.md, timeline.md and the timeline, archives, modules and intent notes.
- [x] Cover `src/vault.rs`, which the index-bound fix changed outside its declared paths.
- [x] Run the verify lane on Linux, with the audit green at the tip.
