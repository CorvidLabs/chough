---
id: 2026-11-20-a-change-s-description
title: A change's description
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T22:44:55Z
kind: shipped
links:
  - 430
  - .chough/archive/sessions/2026-11-20-change-leaks.md
  - .specsync/archive/changes/2026-11-20-change-leaks/
record: .chough/archive/sessions/2026-11-20-change-leaks.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# A change's description

A change's description, rationale and answers pass the leak check before SpecSync writes them, doctor --leaks scans the open changes, and the last echo sites name what a value holds (#412, rest of #367)

Why: 0xLeif decided "Approve the scope of change-leaks": "A: refuse, --redact (Recommended); 1: open changes (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 15:08Z, relayed by hc t4c; the questions as shown: 'Chough #412: a change's description, rationale and answers aren't leak-checked today ...

Verified: Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).

Read more: the pull request 430 · [the story](../../.chough/archive/sessions/2026-11-20-change-leaks.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-change-leaks/)
