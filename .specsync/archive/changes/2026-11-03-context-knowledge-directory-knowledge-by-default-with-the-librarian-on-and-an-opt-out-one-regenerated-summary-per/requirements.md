---
change: context-knowledge-directory-knowledge-by-default-with-the-librarian-on-and-an-opt-out-one-regenerated-summary-per
artifact: requirements
---

# Requirements

Modify REQ-cli-015 (optional librarian):

- The librarian becomes on by default, with an opt-out preference.
- Its directory defaults to `knowledge/`.
- The index adds archive entries, a lessons index and a pointer to an existing `library/` folder.
- Trace adds KNOW-1 to KNOW-6 and KNOW-12.

Add REQ-cli-033 (post-archive summary):

- A preview-first `library archive` step for observation sets that only cold archives (outside the newest 3) list.
- Authorized apply, refused while any native change is open.
- Removes only files committed identically at HEAD, ledger first.
- An archive validity check before and after removal, restoring the files when validity changes.
- Trace: KNOW-7 to KNOW-11, LIFE-8.
