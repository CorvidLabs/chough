---
id: 2026-11-18-cases-check-returns-early-for-a-retired-case-before-checking
title: cases check returns early for a retired case before checking its…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T23:19:26Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-283.md
  - .specsync/archive/changes/2026-11-18-fix-283/
record: .chough/archive/sessions/2026-11-18-fix-283.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# cases check returns early for a retired case before checking its…

cases check returns early for a retired case before checking its fledge-task binding, so the original binding stays as history (#283)

Why: 0xLeif decided "Scope approval of fix-283": "skip the check for retired cases. cases check returns early for a retired case before checking its fledge-task binding, so the original binding stays as history." (Leif's decision comment on CorvidLabs/chough#283 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for the 0.6.2 list); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-283 as 0xLeif

Verified: Ran cargo test --locked --test cases -- --exact a_retired_case_keeps_a_binding_to_a_removed_fledge_task for TC-cli-a-retired-case-keeps-its-fledge-task-binding: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-283.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-283/)
