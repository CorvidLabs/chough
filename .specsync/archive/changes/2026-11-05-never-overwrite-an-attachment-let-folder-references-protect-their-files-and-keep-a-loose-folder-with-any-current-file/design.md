---
change: never-overwrite-an-attachment-let-folder-references-protect-their-files-and-keep-a-loose-folder-with-any-current-file
artifact: design
---

# Design

- **Named through a folder.** A helper `naming(map, key)` yields the sources recorded for the key and for each of its ancestors, split at `/` on the case-folded canonical key that `file_key` gives, so `a/b` never matches `a/bc`. The loose-group protections (checkpoint and bug sources, open bug fixtures) and the observation-set rules (named reasons, and artifacts kept out of removal) use it.
- **Ownership per file.** Each file of a loose group is owned by the archives that list it, or else by `owner_by_cycle` of its own first commit. A group whose files are all current or uncommitted is skipped as before. A group with some such files is reported protected, naming them. Hot owners protect as before. `listed_by` stays the listing archives; `attributed` is set when any file was owned by cycle time.
- **Create-only moves.** `summary_plan` compares each planned move with what is at its destination. Different bytes protect the candidate with the reason; the same bytes mean nothing needs copying. The apply opens the destination with `create_new`. If something appeared meanwhile, the same bytes are kept without registering a copy, and different bytes fail the apply, which restores the originals and leaves that file alone. Only copies this apply created are deleted on rollback.
