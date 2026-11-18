---
id: 2026-11-18-a-test-asserts-the-gate-summary-s-change-line-separator
title: A test asserts the gate summary's Change line separator
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T17:10:31Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-294.md
  - .specsync/archive/changes/2026-11-18-fix-294/
record: .chough/archive/sessions/2026-11-18-fix-294.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# A test asserts the gate summary's Change line separator

A test asserts the gate summary's Change line separator, with no em dash (REQ-cli-061, #294)

Why: Test-only: REQ-cli-061 already names the plain-dash Change line and src/gates.rs already writes it, so no requirement or product code changes. The test's guard is shown by failing against a local, uncommitted em-dash edit of src/gates.rs.

Verified: Ran cargo test --locked --test gates -- --exact a_gate_summary_names_its_change_with_a_plain_dash for TC-cli-gate-summary-names-its-change-with-a-plain-dash: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-294.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-294/)
