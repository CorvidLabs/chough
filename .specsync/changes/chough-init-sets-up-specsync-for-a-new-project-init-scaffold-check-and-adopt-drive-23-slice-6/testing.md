---
change: chough-init-sets-up-specsync-for-a-new-project-init-scaffold-check-and-adopt-drive-23-slice-6
artifact: testing
---

# Testing

New tests in `tests/setup.rs`. They use a recording stand-in `specsync` on PATH: init creates `.specsync/config.toml`, scaffold creates `specs/<module>/`, and `.stub-fail-<step>` makes a step exit 1.

- `a_new_project_gets_specsync_setup_from_chough_init`: the four calls in order, `specsync_setup` lists them with exit code 0, and the checkpoint and skill are then written.
- `an_existing_setup_or_no_module_runs_no_specsync_step`: with `.specsync/`, with `specs/`, and with no module, there are no calls and the status names why.
- `a_failed_setup_step_stops_init_naming_the_steps_left`: a failing scaffold leaves no `.chough/`, and the refusal names `specsync scaffold shipping`, `specsync check`, `specsync change adopt` and the same `chough init`. After running them, that init succeeds and makes no call.
- `a_missing_specsync_refuses_init_naming_install`: the refusal names the install command, and nothing is written. With the stand-in installed, the same init succeeds.
- `the_walkthrough_starts_a_new_project_with_chough_init`: the new-project section has no `specsync init`, `scaffold` or `change adopt`, and names `chough init --module`.

They fail first because init runs no setup and the walkthrough names the direct calls.

Mutation round:
- skip the eligibility check;
- run setup after Chough's writes;
- skip adopt;
- swallow a failed step;
- drop a step from the refusal's list.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-062 | tests/setup.rs |

## Lanes

The local spec check (`--require-coverage 100`), then `fledge run linux-verify`.
