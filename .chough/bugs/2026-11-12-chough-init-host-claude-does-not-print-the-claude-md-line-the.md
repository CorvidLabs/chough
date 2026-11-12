---
kind: bug
id: 2026-11-12-chough-init-host-claude-does-not-print-the-claude-md-line-the
aliases: []
title: chough init --host claude does not print the CLAUDE.md line the README documents
status: verified
classification: product-defect
scope: in-scope
impact: A first run in Claude Code misses the workflow skill (t38m F12)
criteria: []
requirements: [REQ-cli-016]
cases: [TC-cli-init-claude-host-prints-the-claude-md-line]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-init-claude-host-prints-the-claude-md-line
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-init-claude-host-prints-the-claude-md-line-e06091
receipts: []
environment: local-macos-arm64
revision: de7893f8aad7c737300c7aa841f8737100d77f77
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report, F12 (relayed by t370); fixed by t38c)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T22:14:57Z
updated: 2026-11-12T22:18:25Z
---
# chough init --host claude does not print the CLAUDE.md line the README documents

**What happened:** chough init --host claude installs the shared skill copy but prints nothing about the CLAUDE.md line Claude Code needs to find it, which the README says to add
Actual: No hint; the skill stays undiscovered until a person finds the README line

**Steps:** Trigger: Run chough init --host claude in a new project
1. chough init --objective <o> --host claude
2. Read its output

**Expected:** The exact line to add to CLAUDE.md, and whether CLAUDE.md already names the skill, with nothing written to CLAUDE.md

**Why it matters:** A first run in Claude Code misses the workflow skill (t38m F12)

**Fix:** 70468c0a (t38c's 822e4725): init --host claude prints the line to add to CLAUDE.md as claude_md, with whether CLAUDE.md already names the skill, and writes nothing to CLAUDE.md (REQ-cli-016)

## Log

- 2026-11-12T22:14:57Z · Reported (t38m's third fresh-eyes round report, F12 (relayed by t370); fixed by t38c); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-init-claude-host-prints-the-claude-md-line · by claude-code (hc t382)
- 2026-11-12T22:18:25Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-init-claude-host-prints-the-claude-md-line-e06091; fix: 70468c0a (t38c's 822e4725): init --host claude prints the line to add to CLAUDE.md as claude_md, with whether CLAUDE.md already names the skill, and writes nothing to CLAUDE.md (REQ-cli-016). Red at de7893f8, green at 70468c0a · by claude-code (hc t382)
