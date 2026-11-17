---
change: fix-301
artifact: plan
---

# Plan

## Approach

Add clap `///` doc comments to the list-valued checkpoint flags in `src/checkpoint.rs` (`--criterion`, `--requirement`, `--case`, `--design`, `--file`, `--observation`, `--bug`), and update `docs/cli.md` to document the checkpoint command's flags.

## Out of scope

No behavior, contract, or requirement changes; no code logic changes; no spec deltas.

## Steps

1. Add doc comments to each list-valued checkpoint flag in `src/checkpoint.rs`.
2. Update `docs/cli.md` to document the checkpoint flags.
3. Run `chough change ready fix-301` until it reports nothing missing.

## Risks

None — docs and help-text only.

## Constraints consulted

None.
