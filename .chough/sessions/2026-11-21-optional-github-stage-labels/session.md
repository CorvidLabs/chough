---
kind: session
id: 2026-11-21-optional-github-stage-labels
aliases: [optional-github-stage-labels]
objective: Ship an optional GitHub template that labels a pull request with the seven main Chough stages, in rainbow order, without Chough calling GitHub.
intent_scope: LIFE
modules: [cli]
stage: build
stage_status: active
iteration: 1
callers: []
native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
pr: "443"
criteria: [LIFE-31]
requirements: [REQ-cli-070]
cases: [TC-cli-optional-github-stage-labels]
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Pull request 443 is open and not merged. Archive stays red until the change ships.
---
# Ship an optional GitHub template that labels a pull request with the seven main Chough stages, in rainbow order, without Chough calling GitHub.

## Goal

Ship an optional GitHub template that labels a pull request with the seven main Chough stages, in rainbow order, without Chough calling GitHub.

## Where it stands

- Optional GitHub stage-label template, dogfood copies, REQ-cli-070 and github_labels tests. change check passed.
- Pushed optional-github-stage-labels and opened pull request 443.

## Decisions and why

- Optional GitHub stage labels ship as a copied template. The user said to add that shape to Chough on 2026-11-21.
- Scope approved as grok-build under the user's 2026-11-21 words: ok go checkout the chough project and help add this. Limits: init does not install the workflow, src does not call GitHub.

## What was verified

## Open threads

## Timeline

- 2026-11-21T16:28:51Z · start · intent → intent · Ship an optional GitHub template that labels a pull request with the seven main Chough stages, in rainbow order, without Chough calling GitHub. ^t-683c1d
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - by: unknown agent (hc t10j)
- 2026-11-21T16:28:57Z · advance · intent → design · LIFE-31 records the intent: an optional copied template for the seven main stages. ^t-8abbd3
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - by: unknown agent (hc t10j)
- 2026-11-21T16:28:57Z · skip · design → plan · No visuals or interactions. This is an optional copied GitHub workflow and script. ^t-848271
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - by: unknown agent (hc t10j)
- 2026-11-21T16:35:09Z · direct-native-write · plan → plan · optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state draft → approved ^t-e3addf
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - direct_write:
    - change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: draft
    - after_state: approved
  - by: unknown agent (hc t10j)
- 2026-11-21T16:35:09Z · advance · plan → build · Definition approved for the optional GitHub stage-label template. ^t-cbc53d
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - gate: native:approved
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - by: unknown agent (hc t10j)
