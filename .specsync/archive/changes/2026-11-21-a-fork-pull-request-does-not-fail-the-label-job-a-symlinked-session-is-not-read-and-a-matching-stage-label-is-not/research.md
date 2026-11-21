---
change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
artifact: research
---

# Research

GitHub gives a pull request from a fork a read-only `GITHUB_TOKEN`. `gh label create --force` then fails, and `set -euo pipefail` fails the job. Skipping the job when `github.event.pull_request.head.repo.full_name` is not `github.repository` leaves same-repository pull requests, pushes, and `workflow_dispatch` on the write token.

A symlink at a session path is still a name in `git diff`. `[[ -L ]]` is true for a symlink whose target exists, so the read has to be skipped before `awk` opens it. A push to main walks every open pull request with this repository's token, which is why a symlink there matters.

`gh label list` can return the current name, color, and description in one call. Creating a label that is already present, or deleting one that is absent, is the extra API work this change stops.
