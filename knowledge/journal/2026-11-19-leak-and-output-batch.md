---
id: 2026-11-19-leak-and-output-batch
title: Leak and output batch
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T14:44:53Z
kind: shipped
links:
  - https://github.com/CorvidLabs/chough/pull/407
  - .chough/archive/sessions/2026-11-19-leaks-and-output.md
  - .specsync/archive/changes/2026-11-19-leaks-and-output/
record: .chough/archive/sessions/2026-11-19-leaks-and-output.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# Leak and output batch

Leak and output batch: --answer is leak-checked and bounded (#364), conflict findings name only the marker (#365), errors show a value only when it holds no leak and no control characters (#367), the leak guard adds the listed forms (#368), the user name comes from the account (#379), and gate record says Error once (#380)

Why: 0xLeif decided "Scope approval of leaks-and-output": "Yes, as each issue states … approved. Errors name the field and the kind, never the offending text … Show it only if clean … approved.

Verified: Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-history-session-use-names-a-session-only-when-clean: passed (exit 0, tests run 1).

Read more: [the pull request](https://github.com/CorvidLabs/chough/pull/407) · [the story](../../.chough/archive/sessions/2026-11-19-leaks-and-output.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-leaks-and-output/)
