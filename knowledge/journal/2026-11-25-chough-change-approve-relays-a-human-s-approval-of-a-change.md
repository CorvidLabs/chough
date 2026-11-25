---
id: 2026-11-25-chough-change-approve-relays-a-human-s-approval-of-a-change
title: chough change approve relays a human's approval of a change…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-25
time: 2026-11-25T04:15:07Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-25-approve-passthrough.md
  - .specsync/archive/changes/2026-11-25-chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071/
record: .chough/archive/sessions/2026-11-25-approve-passthrough.md
actor:
  - terminal: t14z
---
# chough change approve relays a human's approval of a change…

chough change approve relays a human's approval of a change definition to SpecSync through chough, with a required citation, so no agent calls specsync change approve directly (DRIVE-20)

Why: Code, tests, docs and the changelog fragment are written and green before the scope approval, per the orchestrator's dispatch (quest q-14-1); the definition approval of this change stays Leif's and comes after the handoff.

Verified: Ran cargo test --locked --test change -- --exact approve_refuses_a_leaking_argument_before_specsync_runs for TC-cli-approve-passthrough-leak-checks: passed (exit 0, tests run 1). - e3a7e9f334d6; limits: inherited environment variables were not recorded

Read more: [the story](../../.chough/archive/sessions/2026-11-25-approve-passthrough.md) · [the archived change](../../.specsync/archive/changes/2026-11-25-chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071/)
