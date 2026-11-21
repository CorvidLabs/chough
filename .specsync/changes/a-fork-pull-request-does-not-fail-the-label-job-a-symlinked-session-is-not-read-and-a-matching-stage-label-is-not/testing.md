---
change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-070 | `tests/github_labels.rs`: a symlinked session is ignored; matching labels are not created again; the fork condition is in the workflow; the installed copies match the template |

`the_script_ignores_a_symlinked_session` replaces the introduced `session.md` with a symlink to a file whose stage is `build`. The script applies no stage label.

`the_workflow_does_not_recreate_matching_labels` runs the workflow's label-sync shell with a stand-in `gh`. When the seven labels match and `implement` exists, the log deletes `implement` and does not create a label. A missing label is created without `--force`. A different color is created with `--force`. `verify` is not deleted when it is absent.

The same test requires the job `if` that skips a fork pull request. `the_template_follows_the_main_stages_and_chough_runs_that_copy` already checks that the installed workflow and script match the template bytes.
