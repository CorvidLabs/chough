---
kind: session
id: 2026-11-13-integrations-templates
aliases: [integrations-templates]
objective: "#219: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, with a README saying Chough has no runtime dependency on hc, Herdr or Rune (REQ-cli-065)"
intent_scope: LIFE
modules: [cli]
stage: build
stage_status: active
iteration: 1
callers: []
native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
pr: ""
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
cases:
  - TC-cli-integrations-templates
  - TC-cli-integrations-manifest
  - TC-cli-integrations-digest-detects-change
  - TC-cli-integrations-no-runtime-dependency
  - TC-cli-integrations-readme
design: []
files: [tests/integrations.rs, specs/cli/cli.spec.md]
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Record the red run, then add integrations/
---
# #219: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, with a README saying Chough has no runtime dependency on hc, Herdr or Rune (REQ-cli-065)

## Goal

#219: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, with a README saying Chough has no runtime dependency on hc, Herdr or Rune (REQ-cli-065)

## Where it stands

## Decisions and why

- Templates for Claude Code, Codex, Kimi, Grok and a local agent, the local one marked unqualified until #220 (Leif, Q4, 2026-11-13T03:30Z, relayed by t370; rejected: adding Antigravity, and shipping only Claude Code, Codex and Kimi for now)
- Layout integrations/<host>/hc-template.md for claude-code, codex, kimi, grok and local, agreed with ch10-harness (t38q, 2026-11-13), whose #218 adapter.sh sits beside each; every template keeps exactly hc's name, description and command front matter because the harness reads it as is
- Versions live in integrations/templates.toml with each template's SHA-256, not in template front matter: whether hc accepts extra front-matter keys could not be probed without writing to the machine's shared hc template folder, and the harness needs only the three keys
- Requirement ID REQ-cli-065 (t382, records owner, 2026-11-13)
- Leif decided "Approve the intent: integrations/ ships versioned example hc templates for five hosts (#219, LIFE-24)": "Claude, Codex, Kimi, Grok + local (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219 (local marked unqualified until #220); relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated
- Local template command stays the placeholder chough-local-agent, status unqualified; t38q sets the real command and bumps its version and digest in #220 (t38q, 2026-11-13; Leif chose a generic local setup, LIFE-29). The harness reads only adapter.sh and hc-template.md, so templates.toml is this lane's
- Leif decided "Approve the scope of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a": "Claude, Codex, Kimi, Grok + local (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219 (local marked unqualified until #220); this change's scope is that design's first #219 PR: the folder, five templates, the manifest and the README, REQ-cli-065; relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated; ran specsync change approve 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-13T03:53:10Z · start · intent → intent · #219: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, with a README saying Chough has no runtime dependency on hc, Herdr or Rune (REQ-cli-065) ^t-07efc9
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:10Z · advance · intent → design ·  ^t-da0183
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:11Z · skip · design → plan · No visuals or interactions change: Markdown templates, a manifest, a README and a test ^t-764aed
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:18Z · back · plan → intent · Advanced by mistake: the intent gate's checkpoint was refused (home-path leak in a decision) and a grep in the command chain hid the refusal; redo the intent gate before leaving intent ^t-0e156d
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - refresh: intent: HI criteria and FRD; append new criteria, never rewrite approved ones, design: canonical prototype and companion; their human approval covers the old version, plan: native definition; scope or contract changes need native reapproval
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:27Z · gate-opened · intent → intent · Approve the intent: integrations/ ships versioned example hc templates for five hosts (#219, LIFE-24) ^t-9b5dd3
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - holds: intent
  - drives: none
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:37Z · gate-closed · intent → intent · Approve the intent: integrations/ ships versioned example hc templates for five hosts (#219, LIFE-24) ^t-c345dc
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - relayed_by:
    - decided_by: Leif
    - words: Claude, Codex, Kimi, Grok + local (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219 (local marked unqualified until #220); relayed by claude-code-t370
    - recorded_by: claude-code-t38r
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:37Z · advance · intent → design ·  ^t-3517c8
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:37Z · skip · design → plan · No visuals or interactions change: Markdown templates, a manifest, a README and a test ^t-e6431c
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:55:24Z · gate-opened · plan → plan · Approve the scope of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-9befd6
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - holds: plan
  - drives: approve
  - by: claude-code (hc t38r)
- 2026-11-13T03:55:24Z · gate-closed · plan → plan · Approve the scope of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-86caf9
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - relayed_by:
    - decided_by: Leif
    - words: Claude, Codex, Kimi, Grok + local (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219 (local marked unqualified until #220); this change's scope is that design's first #219 PR: the folder, five templates, the manifest and the README, REQ-cli-065; relayed by claude-code-t370
    - recorded_by: claude-code-t38r
    - native: specsync change approve 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
    - actor: 0xLeif
  - by: claude-code (hc t38r)
- 2026-11-13T03:55:31Z · advance · plan → build · native:approved ^t-d0600f
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:approved
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
