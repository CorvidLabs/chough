---
id: 2026-11-18-a-slug-with-sk-is-not-a-model-provider-key-again
title: A slug with -sk- is not a model-provider key again
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T16:01:02Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-316.md
  - .specsync/archive/changes/2026-11-18-fix-316/
record: .chough/archive/sessions/2026-11-18-fix-316.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
# A slug with -sk- is not a model-provider key again

A slug with -sk- is not a model-provider key again, while the distinctive token forms stay refused after a hyphen (REQ-records-006, #316)

Why: Leif chose to narrow #303's boundary rather than revert it: sk- goes back to 0.6.0's rule, where a hyphen before the prefix continues a word, and the distinctive forms (GitHub, glpat-, xox*-, AKIA/ASIA) keep the hyphen boundary. No requirement text changes; REQ-records-006 does not fix the byte before a prefix.

Verified: Ran cargo test --locked --test leaks -- --exact a_slug_holding_sk_is_not_a_key_and_a_glued_token_still_is for TC-records-sk-after-a-hyphen-is-not-a-key: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-316.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-316/)
