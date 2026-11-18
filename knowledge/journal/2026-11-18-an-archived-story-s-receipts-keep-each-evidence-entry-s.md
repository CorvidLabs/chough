---
id: 2026-11-18-an-archived-story-s-receipts-keep-each-evidence-entry-s
title: An archived story's receipts keep each evidence entry's criteria
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T20:45:02Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-286.md
  - .specsync/archive/changes/2026-11-18-fix-286/
record: .chough/archive/sessions/2026-11-18-fix-286.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# An archived story's receipts keep each evidence entry's criteria

An archived story's receipts keep each evidence entry's criteria, requirements and limitations (REQ-history-006, #286)

Why: 0xLeif decided "Scope approval of fix-286": "keep all three. Archived receipts keep the entry's criteria, requirements and limitations, so the story says what was verified against what and its honest limits stay visible. This adds the three to REQ-history-006's receipt field list." (Leif's decision comment on GitHub issue #286, 2026-11-18, from his planning interview; his planning session (hc t4c) relayed it to fable-t4b as his scope approval); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-286 as 0xLeif

Verified: Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-receipt-keeps-its-entrys-criteria-requirements-and-limitations: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-286.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-286/)
