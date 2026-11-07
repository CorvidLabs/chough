---
change: chough-init-sets-up-specsync-for-a-new-project-init-scaffold-check-and-adopt-drive-23-slice-6
artifact: plan
---

# Plan

## Approach

Add `src/setup.rs`, which `init` in `src/main.rs` calls before its own writes. It runs each step through `records::run_owned`, records each command and exit code, and refuses naming the rest. Move the walkthrough's new-project section, the README and `docs/cli.md` to `chough init --module`.

## Out of scope

- Authored spec content.
- Projects with `specs/` but no `.specsync/`.
- An opt-out flag.
- Upgrading an existing SpecSync setup.

## Steps

1. Add the tests in `tests/setup.rs` with a recording stand-in `specsync`. They fail first.
2. Add `src/setup.rs`, the `init` hook and `specsync_setup` in init's JSON. List `src/setup.rs` and `tests/setup.rs` in the cli spec after approval, so the cli module owns them.
3. Update the walkthrough, README, `docs/cli.md` and CHANGELOG.
4. Materialize, record the cases, run a mutation round, and run the local spec check and then `fledge run linux-verify`.

## Risks

- A failed step after `specsync init` leaves a partial SpecSync setup. The refusal names exactly the steps left, and init completes once they are done.
- `specsync change adopt` turns SDD on at once. That is DRIVE-23's intent, and it gates stage moves from then on.
- `specsync check` may report warnings on fresh scaffolds. Init passes its exit code through as a step result and fails only on a non-zero exit.

## Constraints consulted

- DRIVE-23 and DRIVE-20.
- FLOW-14: a stop stops SpecSync.
- Leif's rule: each refusal names a recovery, and a test runs it.
