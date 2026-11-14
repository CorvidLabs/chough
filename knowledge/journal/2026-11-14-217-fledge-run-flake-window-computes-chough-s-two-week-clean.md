---
id: 2026-11-14-217-fledge-run-flake-window-computes-chough-s-two-week-clean
title: "#217: fledge run flake-window computes Chough's two-week clean…"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-14
time: 2026-11-14T17:34:54Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-flake-window.md
  - .specsync/archive/changes/2026-11-14-217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read/
record: .chough/archive/sessions/2026-11-13-flake-window.md
actor:
  - agent: claude-code
---
# #217: fledge run flake-window computes Chough's two-week clean…

#217: fledge run flake-window computes Chough's two-week clean window from CI history, flake issues and bug records (REQ-cli-062)

Why: The check is a repo script plus a fledge task, recorded as evidence at release; Chough itself gains no command and no GitHub dependency (Leif, Q1, 2026-11-13T03:30Z, relayed by t370; rejected: a chough command, a daily GitHub job posting to an issue)

Verified: Live run after the fix, with the flake labels on GitHub but this branch's records not seeded: verdict blocked by #132, #169 and #187 having no bug record naming them (the seed PR #255 adds those) and #247 open with its record on its fix branch; 814 runs, 136 main runs, 4 flake issues, 206 bug records. This shows the check on live data, not that the window is clean

Read more: [the story](../../.chough/archive/sessions/2026-11-13-flake-window.md) · [the archived change](../../.specsync/archive/changes/2026-11-14-217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read/)
