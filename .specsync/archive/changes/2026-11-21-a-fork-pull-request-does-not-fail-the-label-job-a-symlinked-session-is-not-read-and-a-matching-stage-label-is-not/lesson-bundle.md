# Lesson bundle: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated.
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: integrations/github/chough-labels.yml, integrations/github/chough-label-prs.sh, .github/workflows/chough-labels.yml, tools/chough-label-prs.sh, tests/github_labels.rs, changelog.d/2026-11-21-fork-symlink-label-refresh.md, specs/cli/requirements.md, specs/cli/history/a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not.md
- **Acceptance**: A fork pull request skips the label job. A symlinked session.md is not read. An existing label is created only when it is missing, and --force is used only when its color or description differs. implement and verify are deleted only when they exist.

## Evidence

- Verification commit: `195bdc30073f28b0c49e05ce0a28463f9d8f1abf`
- Base commit: `97df042ea89d865cc08b36d1f96c3901103e366d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

A review of pull request 443 found three failures in the optional stage-label workflow. The repository is public, so a pull request from a fork gets a read-only token and the first `gh label create --force` fails the job. The script reads every introduced session path, including a symlink, and a push to main does that with a write token. Every run also deletes two labels and recreates all seven.

The change that added the template is archived. This follow-up does not edit that archive and does not reopen it.

Ruled out: giving fork pull requests a write token, and recreating the seven labels on every run.

## From the change's design.md

# Design

No product screen changes, so there is no HTML prototype. The labels stay the seven chips from the template.

The label job runs for a push, for workflow dispatch, and for a pull request whose head repository is this repository. A pull request from any other repository skips the job.

The script skips a session path that is a symlink and does not read it.

A run lists `chough-stage:*` once. It creates a label that is missing. It passes `--force` only when that label's color or description differs from the table. It deletes `chough-stage:implement` and `chough-stage:verify` only when the list contains them.

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-070 | `tests/github_labels.rs`: a symlinked session is ignored; matching labels are not created again; the fork condition is in the workflow; the installed copies match the template |

`the_script_ignores_a_symlinked_session` replaces the introduced `session.md` with a symlink to a file whose stage is `build`. The script applies no stage label.

`the_workflow_does_not_recreate_matching_labels` runs the workflow's label-sync shell with a stand-in `gh`. When the seven labels match and `implement` exists, the log deletes `implement` and does not create a label. A missing label is created without `--force`. A different color is created with `--force`. `verify` is not deleted when it is absent.

The same test requires the job `if` that skips a fork pull request. `the_template_follows_the_main_stages_and_chough_runs_that_copy` already checks that the installed workflow and script match the template bytes.

## Where these lessons go

- `specs/cli/cli.3md`
