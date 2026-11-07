---
change: chough-init-sets-up-specsync-for-a-new-project-init-scaffold-check-and-adopt-drive-23-slice-6
artifact: context
---

# Context

Slice 6 of "Chough drives SpecSync". DRIVE-23 was captured from Leif's "Capture + build", relayed by the orc on 2026-11-07: "When I start Chough in a project, chough init also runs SpecSync's setup steps for the modules I name (init, scaffold, adopt) and their structural check, so a new project needs no direct specsync call; an existing SpecSync setup is left as it is."

The interim re-audit counted four direct setup calls per walkthrough: init, scaffold, check and adopt.

## Decisions that bind this change

- **When setup runs.** Only when at least one `--module` is named, and the root has neither `.specsync/` nor `specs/`. That is how this change reads "a new project" and "an existing SpecSync setup is left as it is". Every test fixture and every existing project keeps today's init behavior: fixtures write `specs/`, and package tests name no module.
- **Order, probed in a scratch project with SpecSync 6.2.0.** `specsync init`, then `scaffold` per module, then `check`, then `change adopt` all succeed in a fresh Git project. The check passes with warnings, and adopt enables SDD.
- **Setup before Chough's own writes.** A failed step leaves nothing of Chough's. The refusal names the remaining steps and the same `chough init`. Rerun after them, init finds the setup in place and completes.
- **IDs.** REQ-cli-062, since #188 holds 061.
- **Ruled out:**
  - filling in the scaffolded files, which is authored work;
  - running setup in a project that has `specs/` but no `.specsync/`;
  - an opt-out flag (DRIVE-23 asks for the default).
