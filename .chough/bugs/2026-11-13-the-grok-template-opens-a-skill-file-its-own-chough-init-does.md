---
kind: bug
id: 2026-11-13-the-grok-template-opens-a-skill-file-its-own-chough-init-does
aliases: []
title: the Grok template opens a skill file its own chough init does not create
status: open
classification: product-defect
scope: blocking
impact: A Grok agent in a new project is pointed at a missing file; found before merge
criteria: [LIFE-22]
requirements: [REQ-cli-065]
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: 81eccae15ccf4aac8ba89692cabb5aa59740b782
found_by: claude-code (hc t38r) (manual report)
actor:
  - agent: claude-code
    terminal: t38r
related: []
created: 2026-11-13T05:34:36Z
updated: 2026-11-13T05:34:36Z
---
# the Grok template opens a skill file its own chough init does not create

**What happened:** integrations/grok/hc-template.md runs chough init --host grok, which installs only .grok/skills/chough-workflow/SKILL.md, then tells the agent to load .agents/skills/chough-workflow/SKILL.md
Actual: --host grok writes only .grok/skills; the named .agents path is missing (t384, real init with main's build)

**Steps:** Trigger: Follow the Grok template in a project with no .chough/ folder
1. chough init --objective Probe --host grok in an empty Git repository
2. Look for .agents/skills/chough-workflow/SKILL.md

**Expected:** Each template names a skill file that its own chough init --host value creates

**Why it matters:** A Grok agent in a new project is pointed at a missing file; found before merge

**Fix:** Not fixed yet.

## Log

- 2026-11-13T05:34:36Z · Reported (manual report) · by claude-code (hc t38r)
