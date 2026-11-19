---
id: 2026-11-18-a-refused-checkpoint-busy-evidence-add-leaves-nothing-behind
title: A refused:checkpoint-busy evidence add leaves nothing behind
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T19:39:19Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-325.md
  - .specsync/archive/changes/2026-11-18-fix-325/
record: .chough/archive/sessions/2026-11-18-fix-325.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# A refused:checkpoint-busy evidence add leaves nothing behind

A refused:checkpoint-busy evidence add leaves nothing behind, so a rerun does not duplicate the entry (REQ-cli-040, #325)

Why: 0xLeif decided "Scope approval of fix-325": "approved as a fix. A refused:checkpoint-busy evidence add leaves nothing behind, so a rerun doesn't duplicate the entry." (Leif's decision comment on CorvidLabs/chough#325 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for this change); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-325 as 0xLeif

Verified: Ran cargo test --locked --test evidence -- --exact a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved for TC-cli-a-busy-evidence-add-leaves-nothing: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-325.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-325/)
