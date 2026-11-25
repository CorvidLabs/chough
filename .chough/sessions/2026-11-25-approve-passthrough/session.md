---
kind: session
id: 2026-11-25-approve-passthrough
aliases: [approve-passthrough]
objective: chough change approve relays a human's approval of a change definition to SpecSync through chough, with a required citation, so no agent calls specsync change approve directly (DRIVE-20)
intent_scope: ""
modules: []
stage: build
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
  - gate: Implementation review of chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
    holds: build
    drives: review
    opened: 2026-11-25T04:11:59Z
run_consents: []
next: "Stop before the review gate. After review: at materialization (chough change check) add .chough/sessions/2026-11-25-approve-passthrough/agent/cli.approve-passthrough.cases.toml as .chough/cases/cli.approve-passthrough.cases.toml, run chough cases run, then open the review gate. The phantom affected_paths entry needs a human-authorized specsync change send-back or stays declared"
---
# chough change approve relays a human's approval of a change definition to SpecSync through chough, with a required citation, so no agent calls specsync change approve directly (DRIVE-20)

## Goal

chough change approve relays a human's approval of a change definition to SpecSync through chough, with a required citation, so no agent calls specsync change approve directly (DRIVE-20)

## Where it stands

## Decisions and why

- Code, tests, docs and the changelog fragment are written and green before the scope approval, per the orchestrator's dispatch (quest q-14-1); the definition approval of this change stays Leif's and comes after the handoff.
- leif decided "Scope approval of chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071": "I approvwe" (pi chat t14z, 2026-11-25T03:07Z, orchestrator session (typo for 'I approve', intent unambiguous)); recorded by pi t14z orchestrator (GLM-5.3-Flash-oQ4e-mtp); a relayed claim, not authenticated; ran specsync change approve chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071 as 0xLeif
- Fix round for the q-14 pre-gate checker: reverted the out-of-scope .gitattributes line to origin/main's pinned bytes; approve now runs in the shared steps() table so the .chough/ invariance AC (REQ-cli-071) rests on a real test; the phantom affected_paths entry (changelog.d/2026-11-25-approve-passthrough.toml) cannot be removed while the change is approved — specsync refuses interview answers (expected draft), and send-back needs Leif's authorization

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
- 2026-11-25T03:08:20Z · gate-closed · plan → plan · Scope approval of chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071 ^t-d3ea47
  - native_change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
  - revision: e3de457c5443d991db03dc303663614b73644236
  - relayed_by:
    - decided_by: leif
    - words: I approvwe
    - where: pi chat t14z, 2026-11-25T03:07Z, orchestrator session (typo for 'I approve', intent unambiguous)
    - recorded_by: pi t14z orchestrator (GLM-5.3-Flash-oQ4e-mtp)
    - native: specsync change approve chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-25T03:20:22Z · advance · plan → build · native:approved ^t-057625
  - native_change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
  - gate: native:approved
  - revision: 966c8888a1aa7583fc4faf4c0a535a81dc08d6d6
  - by: unknown agent (hc t17v)
- 2026-11-25T04:11:59Z · gate-opened · build → build · Implementation review of chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071 ^t-4a6b0e
  - native_change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
  - revision: 576e98db9873213a8fb70cdc2bdadb8a89898d91
  - holds: build
  - drives: review
  - by: unknown agent (hc t14z)
