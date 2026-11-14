---
id: 2026-11-14-the-leak-check-reads-an-scp-style-host-path
title: The leak check reads an scp-style host path
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-14
time: 2026-11-14T17:45:12Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-14-275-scp-host-path.md
  - .specsync/archive/changes/2026-11-14-the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275/
record: .chough/archive/sessions/2026-11-14-275-scp-host-path.md
actor:
  - agent: claude-code
---
# The leak check reads an scp-style host path

The leak check reads an scp-style host path, host:/path, like the path it carries (#275)

Why: 0xLeif decided "Scope approval of the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275": "0xLeif ran: specsync change approve the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275 --actor 0xLeif" (0xLeif's own terminal in the Claude Code desktop app, 2026-11-14; the native approval is already recorded); recorded by claude-code; a relayed claim, not authenticated; ran specsync change approve the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275 as 0xLeif

Verified: Ran cargo test --locked --bin chough -- --exact leaks::tests::an_scp_style_remote_path_is_checked_like_the_path_it_carries for TC-records-an-scp-style-host-path-is-checked-like-the-path-it-carries: passed (exit 0).

Read more: [the story](../../.chough/archive/sessions/2026-11-14-275-scp-host-path.md) · [the archived change](../../.specsync/archive/changes/2026-11-14-the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275/)
