# Sequential scope review by the drafting agent

These are role lenses, not independent human reviews.

## Product

LIFE-3/8 and #31: deleting old data merely to reduce file counts would conflict with proof preservation. Resolution: explicit adoption, default preview, active/reference protection and a truthful protected-growth outcome.

## QA

REQ-cli-025/026: timestamps alone cannot prove safe branch integration; byte limits without fixtures are untestable claims. Resolution: real Git branches at controlled equal times, parent-link assertions, instrumented boundary fixtures and preserved-file digests. All checks remain planned.

## Engineering

src/stage.rs and src/timing.rs: every consumer must switch coherently after adoption. Resolution: one history module, bounded cursor traversal, conservative legacy-anchor handling, and explicit reader limits. Large scope estimate; no precise effort estimate from unrelated history.

## Architecture

REQ-cli-007 and #31: a separate mutable head would compete with the checkpoint, and merging unrelated heads would mix tasks. Resolution: optional cursor in the existing atomic current-cycle group, explicit old-reader refusal, immutable event/receipt files and preserved orphan/fork attribution. No extra runtime or lifecycle authority.

## Decision for the human

Approve or revise this one package, including explicit adoption, per-event lineage, the optional checkpoint cursor, the proposed bounds and whole-cycle retention. No remaining product choice is silently treated as a human decision.
