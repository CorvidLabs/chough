---
id: 2026-11-21-stage-labels-skip-forks-and-keep-matches
title: Stage labels skip forks and keep matches
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-21
time: 2026-11-21T17:48:24Z
kind: shipped
links:
  - https://github.com/CorvidLabs/chough/pull/443
  - .chough/archive/sessions/2026-11-21-fork-symlink-label-refresh.md
  - .specsync/archive/changes/2026-11-21-a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not/
record: .chough/archive/sessions/2026-11-21-fork-symlink-label-refresh.md
actor:
  - terminal: t10j
---
# Stage labels skip forks and keep matches

Fork pull requests skip the label job, a symlinked session file is not read, and a stage label is created only when it is missing or its color or description differs.

Verified on this machine: the three bound label cases passed, cargo test --locked --test github_labels passed 5, and the verify lane completed 11 steps with SpecSync 6.4.0 and Fledge 1.8.0.

Next: push the archive on pull request 443 and watch CI. Merging was not requested.

Read more: [the pull request](https://github.com/CorvidLabs/chough/pull/443) · [the story](../../.chough/archive/sessions/2026-11-21-fork-symlink-label-refresh.md) · [the archived change](../../.specsync/archive/changes/2026-11-21-a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not/)
