---
id: 2026-11-07-zombie-aware-native-process-test-probes-187-132
title: "Zombie-aware native process test probes (#187, #132)"
type: journal
tags: [chough/journal, chough/closed]
date: 2026-11-07
time: 2026-11-07T03:40:39Z
kind: closed
links:
  - .chough/archive/sessions/2026-11-07-make-native-process-probes-zombie-aware-187-132-drive-19.md
  - .specsync/archive/changes/2026-11-10-make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit/
record: .chough/archive/sessions/2026-11-07-make-native-process-probes-zombie-aware-187-132-drive-19.md
actor:
  - agent: claude-code
    terminal: t382
---
# Zombie-aware native process test probes (#187, #132)

Test probes treated a killed but unreaped child process as still alive (#187). The agreed scope was one test-only, zombie-aware liveness helper, with no change to the product's process stopping because the descendant was already killed. Issue #132 was to stay open without code changes, since it did not reproduce.

The new probe test failed first and then passed, and the helper and all three migrated probes passed their focused cases on macOS arm64.

Loaded macOS and Linux repetitions and the full Linux verify lane were still to run when the story was closed without reaching done on 7 November 2026.

Read more: [the story](../../.chough/archive/sessions/2026-11-07-make-native-process-probes-zombie-aware-187-132-drive-19.md) · [the archived change](../../.specsync/archive/changes/2026-11-10-make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit/)
