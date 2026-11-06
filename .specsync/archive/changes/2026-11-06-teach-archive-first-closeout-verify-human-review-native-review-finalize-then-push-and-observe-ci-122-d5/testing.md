---
change: teach-archive-first-closeout-verify-human-review-native-review-finalize-then-push-and-observe-ci-122-d5
artifact: testing
---

# Testing

## tests/workflow_authority.rs

`closeout_guidance_pushes_for_ci_only_after_finalize` checks the bundled and installed lifecycle and accept references, docs/lifecycle.md, the walkthrough and the diagram source. In each, every instruction to push and observe CI must come after the first mention of finalize.

It fails first: the lifecycle shape puts "push, CI" before the human implementation review.

## Exercise

`fledge run exercise` (case TC-cli-0101) follows the documented local closeout:
1. Before the review, the archive inspector `.github/scripts/check-specsync-archive.py` refuses the open change.
2. A simulated reviewer runs the native review, then finalize.
3. The inspector then passes, with no CI involved.

The simulated actor is a fixture claim, never Leif's identity.

## Installed copies

An installed copy from 0.4.0 updates, because its digests are added to `KNOWN_PREVIOUS`. The existing upgrade tests still pass.

## Lanes

`fledge lanes run verify` passes on Linux.
