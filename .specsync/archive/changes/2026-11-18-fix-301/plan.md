---
change: fix-301
artifact: plan
---

# Plan

## Approach

Add clap `///` doc comments to all eleven list-valued checkpoint flags in `src/checkpoint.rs` (`--criterion`, `--requirement`, `--case`, `--design`, `--file`, `--observation`, `--bug`, `--decision`, `--question`, `--completed`, `--blocker`). Update `docs/cli.md` to document the checkpoint command's flags, add `change ready` to the change row and to the SpecSync change steps section, add `doctor --leaks` to the doctor row, add `migrate --json` to the migrate row, and add stage help and the top-level `chough help` command. Add a `help` subcommand (likely in `src/main.rs`) so `chough help --help` exits 0 with help text, and add a test for it.

## Out of scope

No behavior, contract, or requirement changes; no code logic changes beyond the help command; no spec deltas; no changes to other subcommands' help text.

## Steps

1. Add clap help text to each of the eleven list-valued checkpoint flags in `src/checkpoint.rs`.
2. Update `docs/cli.md` for the checkpoint flags, change ready, doctor --leaks, migrate --json, stage help and top-level chough help.
3. Add a `chough help` subcommand and a test so `chough help --help` exits 0 with help text.
4. Run `chough change ready fix-301` until it reports nothing missing.

## Risks

None — docs and help-text only.

## Constraints consulted

None.
