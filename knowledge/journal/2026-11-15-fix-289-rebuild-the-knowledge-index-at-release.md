---
id: 2026-11-15-fix-289-rebuild-the-knowledge-index-at-release
title: "Fix #289: rebuild the knowledge index at release"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-15
time: 2026-11-15T21:20:26Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-15-fix-289-index-at-release.md
  - .specsync/archive/changes/2026-11-15-the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289/
record: .chough/archive/sessions/2026-11-15-fix-289-index-at-release.md
actor:
  - agent: claude-code
    terminal: t28
---
# Fix #289: rebuild the knowledge index at release

Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml

Why: 0xLeif decided "Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289": "yes I approve" (Claude Code chat (hc t28), 2026-11-15, on the #289 scope summary; list refreshed only for Flint's two announced tests (7321283) and a re-run of TC-cli-documented-merge-drivers-match); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve…

Verified: Ran cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated for TC-cli-build-reference-from-main-known-outdated: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-15-fix-289-index-at-release.md) · [the archived change](../../.specsync/archive/changes/2026-11-15-the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289/)
