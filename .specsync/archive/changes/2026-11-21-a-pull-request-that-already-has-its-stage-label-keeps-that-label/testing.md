---
change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-070 | `tests/github_labels.rs`: `the_script_keeps_a_label_it_already_applied` |

The test uses the same stand-in `gh` as the other script tests. One run already has `chough-stage:build` and expects no `--remove-label` and no `--add-label`. Another also has `chough-stage:intent` and expects only that label to be removed.

`TC-cli-keep-applied-stage-label` binds that test. The existing label cases stay as they are.
