---
change: never-overwrite-an-attachment-let-folder-references-protect-their-files-and-keep-a-loose-folder-with-any-current-file
artifact: tasks
---

# Tasks

- [x] Add failing tests for each P1: an occupied attachment, one written during the apply, an identical one kept after a rollback, an open bug's folder fixture, a checkpoint-named folder, and a mixed-age folder.
- [x] Match references through ancestor folders for loose groups and observation sets.
- [x] Decide loose ownership per file.
- [x] Make moves create-only and register only the copies this apply made.
- [x] Update docs/cli.md and CHANGELOG.md.
- [x] Run the cases and the verify lane on Linux, then close out (the trim reruns on this repository).
