## ADDED

### REQUIREMENT REQ-cli-062

SpecSync setup through `chough init`. When at least one `--module` is named, and the project root has neither `.specsync/` nor `specs/`, `chough init` SHALL set up SpecSync before it writes anything of its own. It SHALL run, in the project root:
- `specsync init`;
- `specsync scaffold <module>` for each named module, in order;
- `specsync check`;
- `specsync change adopt`.

It SHALL report each step's command and exit code under `specsync_setup` in its JSON. Otherwise it SHALL run none of them, and SHALL report why in `specsync_setup`: no module named, or an existing SpecSync setup left as it is.

When `specsync` cannot be started, init SHALL refuse with nothing written, naming `cargo install --locked specsync@6.2.0` and the same `chough init` command. When a step exits non-zero, init SHALL stop and write nothing of its own. Its refusal SHALL name the failed step and each step left, then the same `chough init` command. That command, run again once those steps are done, finds the setup in place and completes. Trace: DRIVE-23.

Acceptance Criteria
- In a new project, `chough init --module shipping` runs `specsync init`, `specsync scaffold shipping`, `specsync check` and `specsync change adopt` in that order, reports them under `specsync_setup`, and then writes the checkpoint and skill.
- With `.specsync/` or `specs/` present, or with no module named, `chough init` runs no SpecSync step and reports why.
- When a step fails, init stops with nothing of its own written and names the failed step, the steps left and the `chough init` to run again; after those steps, that command succeeds and runs no SpecSync step.
- When `specsync` cannot be started, init refuses with nothing written, naming the install command; with SpecSync installed, the same command succeeds.
- The walkthrough's new-project section runs `chough init --module` instead of `specsync init`, `scaffold` and `change adopt`.
