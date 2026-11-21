---
change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
artifact: design
---

# Design

No product screen changes, so there is no HTML prototype. The labels stay the seven chips from the template.

The label job runs for a push, for workflow dispatch, and for a pull request whose head repository is this repository. A pull request from any other repository skips the job.

The script skips a session path that is a symlink and does not read it.

A run lists `chough-stage:*` once. It creates a label that is missing. It passes `--force` only when that label's color or description differs from the table. It deletes `chough-stage:implement` and `chough-stage:verify` only when the list contains them.
