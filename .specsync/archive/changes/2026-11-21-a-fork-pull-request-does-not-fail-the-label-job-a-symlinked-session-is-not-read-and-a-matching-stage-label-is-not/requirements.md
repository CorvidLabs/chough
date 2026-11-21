---
change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
artifact: requirements
---

# Requirements

REQ-cli-070 gains three rules and keeps its existing sentences. The label job does not run for a fork pull request. The script does not read a symlink. A label is created only when it is missing, `--force` is used only when color or description differs, and `implement` and `verify` are deleted only when they exist. The full text is in `deltas/cli.md`.
