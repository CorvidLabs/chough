---
id: 2026-11-21-optional-github-labels-for-the-seven-main-stages
title: Optional GitHub labels for the seven main stages
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-21
time: 2026-11-21T17:04:00Z
kind: shipped
links:
  - 443
  - .chough/archive/sessions/2026-11-21-optional-github-stage-labels.md
  - .specsync/archive/changes/2026-11-21-optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages/
record: .chough/archive/sessions/2026-11-21-optional-github-stage-labels.md
actor:
  - terminal: t10j
---
# Optional GitHub labels for the seven main stages

The optional integrations/github template labels a pull request with intent, design, plan, build, test, accept and done. Chough runs the same workflow and script. chough init does not install them, and the binary does not call GitHub.

Three cases passed: the script labels one main stage and refuses anything else, the template matches the seven stages, and chough init does not install the workflow. Pull request 443's workflow run applied chough-stage:build. Raven draft 723 walked all seven colors.

Hosted CI is next, after the change is archived. The pull request is not merged.

Read more: the pull request 443 · [the story](../../.chough/archive/sessions/2026-11-21-optional-github-stage-labels.md) · [the archived change](../../.specsync/archive/changes/2026-11-21-optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages/)
