---
change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
artifact: plan
---

# Plan

## Approach

Prefer introduced open session files when choosing the stage. Ignore archived stories while any open session remains. If none remains, choose from the archived stories. Then apply the existing rule: one distinct main stage is applied, and zero, several, or a non-main stage applies none. A stage label that already matches is left in place. Copy the template script onto `tools/chough-label-prs.sh`. Say the same rule in the template README.

## Out of scope

Do not edit the keep-label change, its review gate, or any archived change. Do not merge the pull request. Do not ship this change until its own implementation review. Do not teach `chough init` to install the template, and do not call GitHub from `src/`.

## Steps

- The failing test is already in `tests/github_labels.rs`. It reports several stages for archived `done` plus open `intent`.
- After scope approval, teach both script copies the open-session rule and update the template README.
- Bind the test as its own case. Add the changelog fragment.
- Run the label tests and the integrations README test. Run the verify lane with the pinned tools before push.

## Risks

The keep-label change is still open and declares the same two scripts and the same test file. This change declares them too because it edits them. Its records stay separate.

A side-stage open session must not fall back to an archived main stage. The rule chooses the open set first, then rejects a non-main stage.

## Constraints consulted

`tasks.md` has no unchecked boxes. The delta is frozen once approved. The two script copies stay byte-identical. Archived records and the approved keep-label change are not edited.

Relative size is small: one choice in the label script, plus the requirement text. No hour estimate. The uncertainty is the overlap with the open keep-label change, not the rule itself.
