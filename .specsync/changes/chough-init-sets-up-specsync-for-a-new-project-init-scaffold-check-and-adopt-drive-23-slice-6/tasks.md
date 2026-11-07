---
change: chough-init-sets-up-specsync-for-a-new-project-init-scaffold-check-and-adopt-drive-23-slice-6
artifact: tasks
---

# Tasks

- [ ] Add the REQ-cli-062 tests in `tests/setup.rs`. They fail first.
- [ ] Add `src/setup.rs` and the `init` hook.
- [ ] List `src/setup.rs` and `tests/setup.rs` in `specs/cli/cli.spec.md`.
- [ ] Move the walkthrough's new-project section, README and `docs/cli.md` to `chough init --module`, and add a CHANGELOG line.
- [ ] Add REQ-cli-062 (materialized from the delta).

After `specsync change check` materializes the delta, not task boxes:
- record the cases;
- run the mutation round;
- run the local spec check and `fledge run linux-verify`.
