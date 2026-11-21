---
change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
artifact: context
---

# Context

A review of pull request 443 found three failures in the optional stage-label workflow. The repository is public, so a pull request from a fork gets a read-only token and the first `gh label create --force` fails the job. The script reads every introduced session path, including a symlink, and a push to main does that with a write token. Every run also deletes two labels and recreates all seven.

The change that added the template is archived. This follow-up does not edit that archive and does not reopen it.

Ruled out: giving fork pull requests a write token, and recreating the seven labels on every run.
