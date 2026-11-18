---
id: 2026-11-18-a-leaking-id-reference-s-refusal-names-the-recovery
title: A leaking ID reference's refusal names the recovery
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T20:05:38Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-320.md
  - .specsync/archive/changes/2026-11-18-fix-320/
record: .chough/archive/sessions/2026-11-18-fix-320.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# A leaking ID reference's refusal names the recovery

A leaking ID reference's refusal names the recovery, and doctor --fix --redact never rewrites an ID (REQ-history-012, REQ-cli-055, #320)

Why: The refusal's recovery fits REQ-records-006 as it stands; the doctor's never rewriting an ID changes REQ-history-012's leak fix and REQ-cli-055's --fix, since the doctor code is cli's. session new --module's leak is left on #320, outside Leif's decision.

Verified: Ran cargo test --locked --test leaks -- --exact doctor_redact_never_rewrites_an_id for TC-cli-doctor-redact-never-rewrites-an-id: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-320.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-320/)
