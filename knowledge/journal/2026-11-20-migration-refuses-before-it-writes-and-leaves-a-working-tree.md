---
id: 2026-11-20-migration-refuses-before-it-writes-and-leaves-a-working-tree
title: Migration refuses before it writes and leaves a working tree
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T06:25:08Z
kind: shipped
links:
  - 417
  - .chough/archive/sessions/2026-11-19-m1-migrate-refuses-before-writing.md
  - .specsync/archive/changes/2026-11-20-fix-m1-migrate-refuses-before-writing/
record: .chough/archive/sessions/2026-11-19-m1-migrate-refuses-before-writing.md
actor:
  - agent: t4c
    model: claude-opus-5-5
    terminal: t4c
---
# Migration refuses before it writes and leaves a working tree

chough migrate reads every target and receipt entry without following a link or waiting and refuses anything in the way before it writes (#337); judges returning 0.5.x files by the committed receipt that lists them (#370); takes the writer lock before it plans, so one apply of a digest applies (#378); removes no 0.5.x file before every planned file is on disk and gives knowledge/index.md its name by moving the 0.5.x INDEX.md aside, on every file system including a Docker Desktop mount (#338); removes folders left holding only macOS metadata and names them (#375); and doctor --fix keeps a pointer that names an open session by alias (#376).

Verified at 75264e00: all eleven verify steps and fourteen cases on macOS; the tester's PASS on macOS (25 set rows fixed, nine real repositories); tester2's PASS on Linux and a Docker Desktop mount; corvid-agent's approval; fable-lead's read.

Read more: the pull request 417 · [the story](../../.chough/archive/sessions/2026-11-19-m1-migrate-refuses-before-writing.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-fix-m1-migrate-refuses-before-writing/)
