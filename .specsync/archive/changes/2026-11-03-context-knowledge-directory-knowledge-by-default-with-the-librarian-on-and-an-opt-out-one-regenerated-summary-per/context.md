---
change: context-knowledge-directory-knowledge-by-default-with-the-librarian-on-and-an-opt-out-one-regenerated-summary-per
artifact: context
---

# Context

Leif (0xLeif) asked on 2026-11-03: "Yes but I want the librarian and archiving and compounding of lessons and archives and all.. artifacts.. we need a context knowledge directory". He then answered the drafted questions in chat, between 8:09 and 8:27 AM MT:

- Q1, "Keep only a summary; raw files stay recoverable from git".
- Q2, "Keep the newest 3 archived changes hot".
- Q3, "Librarian on everywhere with an opt-out".
- Q4, "Use knowledge/ at the project root".

He also said yes to the drafted intent. HI KNOW-1 to KNOW-12 in hi/knowledge.md capture these answers.

At main 0363d4a the librarian is off by default and its folder defaults to `library/`. It only snapshots and indexes, and no step runs after an archive. `.chough/` tracks 1,845 files (about 25 MB), and `chough evidence prune` finds nothing eligible, because every record is younger than 14 days.

Human authority: Leif (0xLeif) in chat, 2026-11-03 8:09–8:27 AM MT. The parent agent relayed the answers above as his intent approval; his next decision is final acceptance.

Covered scope:

- the librarian defaults;
- the `knowledge/` index, archive entries and lessons index;
- a previewed, authorized `chough library archive` step;
- `chough next` naming the index;
- tests, REQ-cli-015, a new REQ-cli-033, and docs.

Limits:

- This change does not run the apply step on this repository's own `.chough` history; that live cleanup needs Leif's separate yes.
- No outside storage (#14 item 3), no release and no deployment.
- Duplicate or contradicting lesson detection, scoped lessons in `next`, and retired-lesson history are deferred.
