---
change: context-knowledge-directory-knowledge-by-default-with-the-librarian-on-and-an-opt-out-one-regenerated-summary-per
artifact: plan
---

# Plan

## Approach

Change the librarian preference defaults and point `library index` at a regenerated knowledge directory:

- `INDEX.md`;
- `archives/<change-id>.md`, generated from each archive's own records, Chough observation records and the retention ledger;
- a lessons index of pointers.

Add `chough library archive [--apply]`, built on the existing retention classification plus hot-archive protection, and name the index in `chough next`.

## Out of scope

- Applying the summary step to this repository's history.
- Outside storage.
- Lesson duplicate detection.
- Editing archives or specs after archive.
- Changing `evidence prune` thresholds.

## Steps

1. Write failure-first tests and cases.
2. Change the preference defaults and the index (knowledge entries, lessons index, legacy library listing).
3. Add the archive preview and apply with ledger, HEAD byte check, open-change refusal and validity restore.
4. Make `next` name the index.
5. Update existing tests where the defaults change, plus docs and skill references.
6. Run a dry-run preview on this repository and report it without applying.

## Risks

- **Archive verification files list `.chough` paths.** Removing those paths could change native validity. The apply step re-reads the validity of every affected archive and restores the files on any change.
- **Running `specsync change status` is slow per archive.** Only archives whose listed files are removed are probed.

## Constraints consulted

- AGENTS.md (preserve records; never invent results).
- LIFE-8 retention protections.
- specs/cli/context.md lessons on native manifests and retention.
