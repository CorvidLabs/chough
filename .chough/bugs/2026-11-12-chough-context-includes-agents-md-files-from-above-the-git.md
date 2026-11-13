---
kind: bug
id: 2026-11-12-chough-context-includes-agents-md-files-from-above-the-git
aliases: []
title: chough context includes AGENTS.md files from above the Git root, with their absolute path in the heading
status: verified
classification: product-defect
scope: blocking
impact: Unrelated instructions and a home path in the agent's packet
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-reads-agents-md-from-the-git-root
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-reads-agents-md-from-the-git-root-bad891
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:40Z
updated: 2026-11-12T17:45:15Z
---
# chough context includes AGENTS.md files from above the Git root, with their absolute path in the heading

**What happened:** The packet pulled in a home-folder AGENTS.md (about 560 of its 760 lines) with the absolute home path in the section heading; included files' ## headings sit at the packet's own level
Actual: Every ancestor's AGENTS.md is included, headed by its absolute path

**Steps:** chough context in a project inside a home folder that has an AGENTS.md

**Expected:** The walk stops at the repository root, or is documented, and no absolute home path is printed

**Why it matters:** Unrelated instructions and a home path in the agent's packet

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:40Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
- 2026-11-12T17:43:16Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-reads-agents-md-from-the-git-root. Reproduced: t37f's red test (afaaf0c0, cherry-picked as 429a28ca) failed at dd3e2ab5: context read an AGENTS.md above the Git root, headed by its absolute path · by claude-code (hc t382)
- 2026-11-12T17:45:15Z · status: investigating -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-reads-agents-md-from-the-git-root-bad891. Green after t37f's fix 1d06f3cf (cherry-picked as 5f7cc1a8): context reads AGENTS.md from the Git root down to the project only, named relative to the project, with no absolute path · by claude-code (hc t382)
