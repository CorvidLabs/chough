---
id: 2026-11-05-stage-vault-copies-privately-so-a-failed-copy-deletes-nothing
title: Stage vault copies privately so a failed copy deletes nothing
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-05
time: 2026-11-05T17:57:42Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-05-stage-each-vault-copy-privately-so-a-failed-copy-never-deletes.md
  - .specsync/archive/changes/2026-11-05-stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review/
record: .chough/archive/sessions/2026-11-05-stage-each-vault-copy-privately-so-a-failed-copy-never-deletes.md
actor:
  - agent: claude-code
    terminal: t382
---
# Stage vault copies privately so a failed copy deletes nothing

A priority-one review finding on PR #115 asked that a failed vault copy never delete another writer's bytes. Each copy is now staged privately and then linked into place.

The unit test that a failed copy never deletes another writer's bytes passed on Linux.

The maintainer recorded the native review as a pass on 2026-11-05 with Record and merge, and the story closed done that day.

Read more: [the story](../../.chough/archive/sessions/2026-11-05-stage-each-vault-copy-privately-so-a-failed-copy-never-deletes.md) · [the archived change](../../.specsync/archive/changes/2026-11-05-stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review/)
