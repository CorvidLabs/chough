---
id: 2026-11-20-control-and-echo-batch
title: Control and echo batch
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T07:33:43Z
kind: shipped
links:
  - 418
  - .chough/archive/sessions/2026-11-19-control-and-echo.md
  - .specsync/archive/changes/2026-11-20-control-and-echo/
record: .chough/archive/sessions/2026-11-19-control-and-echo.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# Control and echo batch

Control and echo batch: control characters refused at write and escaped on print (#366), --pr takes a number or an http(s) URL and is shown in next, status, context and the checkpoint view (#353), the remaining sites echo no value (part of #367), and a file URL holds the user name

Why: 0xLeif decided "Standing OK (d): rebase control-and-echo's REQ-cli-007 onto main after #413": "yes to both" (Standing OK (d), Leif's Claude Code chat message relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:02Z; #413 changed REQ-cli-007 on main, so the delta was rebased onto main's text at 2968f061 with exactly the approved words (six body edits, one --pr criterion replaced and two added) and #413's (one sentence,…

Verified: Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1).

Read more: the pull request 418 · [the story](../../.chough/archive/sessions/2026-11-19-control-and-echo.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-control-and-echo/)
