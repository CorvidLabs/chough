---
kind: session
id: 2026-11-25-approve-passthrough
aliases: [approve-passthrough]
objective: chough change approve relays a human's approval of a change definition to SpecSync through chough, with a required citation, so no agent calls specsync change approve directly (DRIVE-20)
intent_scope: ""
modules: []
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
pr: ""
criteria: [DRIVE-20]
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Scope approval of chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
    holds: plan
    drives: approve
    opened: 2026-11-25T02:17:25Z
run_consents: []
next: "Awaiting Leif's scope approval of the change (the open gate). After it: add .chough/cases/cli.approve-passthrough.cases.toml from the session's agent folder, run chough cases run, open the review gate, then chough change check <id> to materialize REQ-cli-071 and commit the verification."
---
# chough change approve relays a human's approval of a change definition to SpecSync through chough, with a required citation, so no agent calls specsync change approve directly (DRIVE-20)

## Goal

chough change approve relays a human's approval of a change definition to SpecSync through chough, with a required citation, so no agent calls specsync change approve directly (DRIVE-20)

## Where it stands

## Decisions and why

- Code, tests, docs and the changelog fragment are written and green before the scope approval, per the orchestrator's dispatch (quest q-14-1); the definition approval of this change stays Leif's and comes after the handoff.

## What was verified

## Open threads

## Timeline

- 2026-11-25T02:09:13Z · start · intent → intent · chough change approve relays a human's approval of a change definition to SpecSync through chough, with a required citation, so no agent calls specsync change approve directly (DRIVE-20) ^t-172786
  - revision: 61479feb1e88a57bc45716ff4e42ce84fb2b463b
  - by: unknown agent (hc t17v)
- 2026-11-25T02:09:22Z · advance · intent → design ·  ^t-b7c2ad
  - revision: 61479feb1e88a57bc45716ff4e42ce84fb2b463b
  - by: unknown agent (hc t17v)
- 2026-11-25T02:09:25Z · skip · design → plan · CLI passthrough command with no visuals or interactions ^t-61a37f
  - revision: 61479feb1e88a57bc45716ff4e42ce84fb2b463b
  - by: unknown agent (hc t17v)
- 2026-11-25T02:17:25Z · gate-opened · plan → plan · Scope approval of chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071 ^t-63ef54
  - native_change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
  - revision: 61479feb1e88a57bc45716ff4e42ce84fb2b463b
  - holds: plan
  - drives: approve
  - by: unknown agent (hc t17v)
