---
change: doctor-fix-safety
artifact: testing
---

# Testing

Failing tests first, on main:
- #381: a real merge in which main archives a session and a branch adds an evidence entry and an inbox entry to it; `doctor --fix` keeps both, the finding names them and the way out, and the entry still resolves. An entry whose receipt no longer matches is kept too.
- #377: a `.DS_Store` and a `._session.md` in an archived folder; `doctor --fix` removes the folder.
- #371: `next` after done, after the commit, and after `doctor --fix`.
- #374: a front-matter conflict gives `conflict_markers` and an `unreadable` finding naming the conflict; after `doctor --fix --redact` on a Timeline leak no `timeline_rewritten`, while a hand edit writing `<redacted:token>` over ordinary words is still reported.
- #402: a spec file link out of the project, a folder link out of it and a link cycle; `doctor` and `init` finish within a time bound and give the same advice; a `specs/` past the bound gets the note.
