---
id: 2026-11-20-a-closed-standard-output-ends-a-command-quietly-with-its-own
title: A closed standard output ends a command quietly with its own exit code
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T17:34:18Z
kind: shipped
links:
  - 425
  - .chough/archive/sessions/2026-11-20-closed-pipe-and-help.md
  - .specsync/archive/changes/2026-11-20-closed-pipe-and-help/
record: .chough/archive/sessions/2026-11-20-closed-pipe-and-help.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# A closed standard output ends a command quietly with its own exit code

A closed standard output ends a command quietly with its own exit code, and help works after global options, with -V documented as the top-level version flag (#383's closed pipe, #399)

Why: 0xLeif decided "Approve the scope of closed-pipe-and-help": "Standing OK, tell me after" (Leif's standing OK for scope approvals, from his Claude Code question dialog in his planning session (hc t4c), 2026-11-20 ('Standing OK, tell me after'), relayed via t4c, which confirmed on 2026-11-20 that E1 closed-pipe-and-help at 8e562c06 has only the recommended choice, changes no requirement wording and adds no refusal; recorded after one interview correction SpecSync required for a no-spec…

Verified: Ran cargo test --locked --test cli -- --exact help_after_the_global_options_prints_the_top_level_help for TC-cli-help-after-the-global-options: passed (exit 0, tests run 1).

Read more: the pull request 425 · [the story](../../.chough/archive/sessions/2026-11-20-closed-pipe-and-help.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-closed-pipe-and-help/)
