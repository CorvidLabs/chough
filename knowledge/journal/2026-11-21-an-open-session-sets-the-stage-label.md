---
id: 2026-11-21-an-open-session-sets-the-stage-label
title: An open session sets the stage label
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-21
time: 2026-11-21T18:39:20Z
kind: shipped
links:
  - 443
  - .chough/archive/sessions/2026-11-21-open-session-sets-the-stage-label.md
  - .specsync/archive/changes/2026-11-21-an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request/
record: .chough/archive/sessions/2026-11-21-open-session-sets-the-stage-label.md
actor:
  - terminal: t10j
---
# An open session sets the stage label

When a pull request archives a session and starts a new one, the stage label shows the new open session. An archived story sets the label only when no open session remains.

Verified on this machine: the new test failed first on archived done plus open intent, then TC-cli-open-session-sets-the-stage-label passed. cargo test --test github_labels passed 7, and the verify lane completed 11 steps.

Next: push both archives on pull request 443 and watch Archive. Merging was not requested.

Read more: the pull request 443 · [the story](../../.chough/archive/sessions/2026-11-21-open-session-sets-the-stage-label.md) · [the archived change](../../.specsync/archive/changes/2026-11-21-an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request/)
