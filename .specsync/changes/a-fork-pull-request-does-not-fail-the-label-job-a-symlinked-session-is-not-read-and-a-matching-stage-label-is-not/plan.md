---
change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
artifact: plan
---

# Plan

## Approach

Keep both copies of the workflow identical, and both copies of the script identical. Add the fork condition on the label job. Skip symlinks before the script reads `stage:`. Create or refresh a label only when the listed name, color, or description requires it.

## Out of scope

No reopen of the archived template change. No new token permissions. No change to stage order, colors, or which session paths count. Jackdaw is not updated here.

## Steps

1. Extend `tests/github_labels.rs` so a symlink is ignored and a matching label is not created again.
2. Apply the three fixes to the template and copy them onto the installed files.
3. Approve under the review instruction, then run the label tests, the integrations README test, and the verify lane.

## Risks

A fork pull request's commits are not labeled by the pull_request event. A later push to main can still label that pull request from this repository's token, which is the existing push path.

## Constraints consulted

`tasks.md` has no unchecked boxes. The approved delta stays frozen. `actions/checkout` stays on the commit `ci.yml` uses.
