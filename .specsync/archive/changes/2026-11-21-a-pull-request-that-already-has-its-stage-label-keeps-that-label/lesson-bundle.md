# Lesson bundle: a-pull-request-that-already-has-its-stage-label-keeps-that-label

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: A pull request that already has its stage label keeps that label
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: integrations/github/chough-label-prs.sh, tools/chough-label-prs.sh, tests/github_labels.rs, .chough/cases/cli.keep-applied-stage-label.cases.toml, changelog.d/2026-11-21-keep-applied-stage-label.md
- **Acceptance**: A pull request that already has chough-stage:build, and whose introduced session is build, is not edited: the script does not remove that label and does not add it. A different chough-stage label on that pull request is still removed. The two script copies stay byte-identical.

## Evidence

- Verification commit: `522fb47fc2aaabfdb22a1df1eebba7665d5a6ee1`
- Base commit: `727d3bfdeef47b5a5df912d542565f8ba321a377`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

corvid-agent reviewed pull request 443 at `727d3bfd` and requested one code change. The label script removes every `chough-stage:*` label, including the one it is about to apply, then adds that label again. REQ-cli-070 already says to apply the one main stage and remove every other stage label.

The fork guard, the symlink skip, and the label-definition refresh are already archived. This change does not reopen that archive.

Ruled out: leaving the remove-and-add in place and only filing a follow-up issue. The review asked for the script change.

## From the change's design.md

# Design

Nothing visual changes. There is no HTML mockup.

When the pull request already has `chough-stage:<stage>` for the one main stage, that label stays. The script does not remove it and does not add it. Any other `chough-stage:*` label is still removed. When there is no stage to apply, every `chough-stage:*` label is still removed.

The same edit is copied into both script files so they stay byte-identical. The workflow is unchanged.

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-070 | `tests/github_labels.rs`: `the_script_keeps_a_label_it_already_applied` |

The test uses the same stand-in `gh` as the other script tests. One run already has `chough-stage:build` and expects no `--remove-label` and no `--add-label`. Another also has `chough-stage:intent` and expects only that label to be removed.

`TC-cli-keep-applied-stage-label` binds that test. The existing label cases stay as they are.

## Where these lessons go

- `specs/cli/cli.3md`
