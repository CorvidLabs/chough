---
id: 2026-11-12-turn-the-mvp-interview-into-intent-and-a-1-0-roadmap
title: Turn the MVP interview into intent and a 1.0 roadmap
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-12
kind: shipped
links:
  - .chough/archive/sessions/2026-11-10-roadmap-1-0-hi-capture.md
  - .specsync/archive/changes/2026-11-11-capture-the-2026-11-10-mvp-interview-as-hi-criteria-and-add-the-1-0-roadmap-test-plan-and-draft-issue-mapping/
record: .chough/archive/sessions/2026-11-10-roadmap-1-0-hi-capture.md
actor:
  - agent: claude-code
    terminal: t382
---
# Turn the MVP interview into intent and a 1.0 roadmap

The 2026-11-10 MVP interview became human-intent criteria, a 1.0 roadmap with milestones M1 to M7, a 1.0 test plan and a draft issue list, in PR #212, alongside a Chough 1.0 milestone, epic #232 and issues #213 to #231. Later rounds of decisions and design review were folded in, including the 0.6.0 plan: the human-first redesign ships as one change, dogfooded on Chough first.

After review the merged head passed the 10-step verify lane. The intent ended at 112 criteria, and verification was refreshed after a bug was recorded for intermittent lane failures, with tests green under the shared lock.

The maintainer approved the intent and native scope and said to merge when green, relayed rather than authenticated. Nine superseded knowledge criteria stayed with notes, to be retired in 0.6.0. The story closed done on 2026-11-12.

Read more: [the story](../../.chough/archive/sessions/2026-11-10-roadmap-1-0-hi-capture.md) · [the archived change](../../.specsync/archive/changes/2026-11-11-capture-the-2026-11-10-mvp-interview-as-hi-criteria-and-add-the-1-0-roadmap-test-plan-and-draft-issue-mapping/)
