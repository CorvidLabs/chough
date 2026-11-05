---
change: never-overwrite-an-attachment-let-folder-references-protect-their-files-and-keep-a-loose-folder-with-any-current-file
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-046 (see `deltas/cli.md`):
- loose files are owned per file, and a folder with any current file stays;
- a folder named by the checkpoint or by a bug that is not closed protects the files and observation sets inside it;
- moves never overwrite: an identical attachment is kept, a different one keeps both files and protects the set, and one that appears during the apply refuses it.
