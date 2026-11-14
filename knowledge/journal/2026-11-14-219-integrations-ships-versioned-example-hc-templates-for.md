---
id: 2026-11-14-219-integrations-ships-versioned-example-hc-templates-for
title: "#219: integrations/ ships versioned example hc templates for Claude Code"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-14
time: 2026-11-14T17:34:03Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-integrations-templates.md
  - .specsync/archive/changes/2026-11-14-219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a/
record: .chough/archive/sessions/2026-11-13-integrations-templates.md
actor:
  - agent: claude-code
---
# #219: integrations/ ships versioned example hc templates for Claude Code

#219: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, with a README saying Chough has no runtime dependency on hc, Herdr or Rune (REQ-cli-065)

Why: Templates for Claude Code, Codex, Kimi, Grok and a local agent, the local one marked unqualified until #220 (Leif, Q4, 2026-11-13T03:30Z, relayed by t370; rejected: adding Antigravity, and shipping only Claude Code, Codex and Kimi for now)

Verified: The full test suite passed 743 of 743 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at 5219fe8d, #253's head with main (#242) merged and the narrowed REQ-cli-065 materialized as cli version 90; the six integrations cases passed at the same content

Read more: [the story](../../.chough/archive/sessions/2026-11-13-integrations-templates.md) · [the archived change](../../.specsync/archive/changes/2026-11-14-219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a/)
