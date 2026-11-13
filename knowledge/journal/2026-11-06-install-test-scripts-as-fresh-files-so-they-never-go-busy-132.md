---
id: 2026-11-06-install-test-scripts-as-fresh-files-so-they-never-go-busy-132
title: Install test scripts as fresh files so they never go busy (#132)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T06:36:06Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-install-test-scripts-as-fresh-files-so-a-forked-child-holding.md
  - .specsync/archive/changes/2026-11-06-install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132/
record: .chough/archive/sessions/2026-11-06-install-test-scripts-as-fresh-files-so-a-forked-child-holding.md
actor:
  - agent: claude-code
    terminal: t382
---
# Install test scripts as fresh files so they never go busy (#132)

Issue #132: a forked child still holding an old write handle could make an installed test script busy, so it could not run. Test scripts are now installed as fresh files.

A test that a script installed while an old write handle is open still runs failed first with a text-file-busy error, then passed on Linux. The SpecSync change check passed.

PR #145 was accepted with a passing implementation review, recorded under the maintainer's overnight authorization. The change was archived and the story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-install-test-scripts-as-fresh-files-so-a-forked-child-holding.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132/)
