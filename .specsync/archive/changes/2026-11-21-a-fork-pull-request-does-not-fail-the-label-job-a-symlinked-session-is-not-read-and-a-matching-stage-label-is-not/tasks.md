---
change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
artifact: tasks
---

# Tasks

Add the fork condition to both copies of `chough-labels.yml`.

Skip a symlinked session path in both copies of `chough-label-prs.sh`.

List `chough-stage:*` once, create a missing label, pass `--force` only when its color or description differs, and delete `implement` and `verify` only when they are present.

Extend `tests/github_labels.rs` for the symlink and for labels that already match. Keep the two workflow copies and the two script copies byte-identical.

Write the changelog fragment. The requirement text for REQ-cli-070 gains these three rules and keeps every existing sentence.
