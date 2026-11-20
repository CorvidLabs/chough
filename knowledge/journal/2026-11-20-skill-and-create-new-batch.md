---
id: 2026-11-20-skill-and-create-new-batch
title: Skill and create-new batch
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T15:33:35Z
kind: shipped
links:
  - 420
  - .chough/archive/sessions/2026-11-19-skill-and-create-races.md
  - .specsync/archive/changes/2026-11-20-skill-and-create-races/
record: .chough/archive/sessions/2026-11-19-skill-and-create-races.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# Skill and create-new batch

Skill and create-new batch: skill update writes CRLF copies and names a file it cannot write (#373), parallel skill update and init runs all succeed (#386), a host folder that cannot be inspected is skipped and reported while the rest update (#403), and doctor reads an uncommitted session that fails validation (#408)

Why: 0xLeif decided "Approve the scope of skill-and-create-races": "Approve" (Claude Code chat answer, relayed by his planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-19 about 14:12Z, on D2 skill-and-create-races at 2bf6318 with the four recommended choices; recorded at the batch start on that text, the draft carried onto main 056cbb80 by a merge with no delta…

Verified: Ran cargo test --locked --test skill_releases -- --exact recognition_requires_the_filename_and_exact_unnormalized_bytes for TC-cli-release-name-byte-matching: passed (exit 0, tests run 1).

Read more: the pull request 420 · [the story](../../.chough/archive/sessions/2026-11-19-skill-and-create-races.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-skill-and-create-races/)
