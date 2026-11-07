---
change: chough-init-sets-up-specsync-for-a-new-project-init-scaffold-check-and-adopt-drive-23-slice-6
artifact: requirements
---

# Requirements

ADDED REQ-cli-062 (see `deltas/cli.md`):
- `chough init` with at least one module, in a project with neither `.specsync/` nor `specs/`, runs SpecSync's init, scaffold, check and adopt before its own writes;
- otherwise it runs none of them and says why;
- a failure or a missing `specsync` refuses with nothing of Chough's written, naming the recovery.

Trace: DRIVE-23.
