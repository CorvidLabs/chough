---
id: 2026-11-10-keep-observations-current-across-knowledge-index-rebuilds-136
title: Keep observations current across knowledge index rebuilds (#136)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-10
time: 2026-11-10T22:48:57Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-10-keep-observations-current-when-the-generated-knowledge-index.md
  - .specsync/archive/changes/2026-11-10-keep-observations-current-when-the-generated-knowledge-index-is-rebuilt-136/
record: .chough/archive/sessions/2026-11-10-keep-observations-current-when-the-generated-knowledge-index.md
actor:
  - agent: claude-code
    terminal: t382
---
# Keep observations current across knowledge index rebuilds (#136)

Rebuilding the generated knowledge index after recording an observation no longer leaves that observation out of date, fixing #136 under criterion LIFE-7 and requirement REQ-cli-004.

The new test that rebuilding the knowledge index keeps the observation current failed first, then passed three times.

Scope approval and the implementation review came under the maintainer's merge rule for bug fixes, covering #136 only and relayed rather than authenticated. The native change was archived and the story closed done on 2026-11-10.

Read more: [the story](../../.chough/archive/sessions/2026-11-10-keep-observations-current-when-the-generated-knowledge-index.md) · [the archived change](../../.specsync/archive/changes/2026-11-10-keep-observations-current-when-the-generated-knowledge-index-is-rebuilt-136/)
