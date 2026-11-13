---
id: 2026-11-06-support-macos-and-linux-with-windows-through-wsl-flow-17
title: Support macOS and Linux, with Windows through WSL (FLOW-17)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T05:23:53Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-support-macos-and-linux-with-windows-through-wsl-and-drop-the.md
  - .specsync/archive/changes/2026-11-06-support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17/
record: .chough/archive/sessions/2026-11-06-support-macos-and-linux-with-windows-through-wsl-and-drop-the.md
actor:
  - agent: claude-code
    terminal: t382
---
# Support macOS and Linux, with Windows through WSL (FLOW-17)

Chough now supports macOS and Linux, with Windows through WSL 2 on Ubuntu. The native Windows CI job was dropped, leaving an Ubuntu and macOS product matrix, and the README, docs, roadmap and changelog say so (FLOW-17). The maintainer chose this to match SpecSync 6.0, which publishes no Windows binary, and because other PRs were waiting on the native Windows job.

The supported-platforms test failed first, then passed. The SpecSync change check passed, and after merging main the full verify lane passed on macOS: 10 steps and 459 tests, with the audit and the exercise green.

The maintainer accepted PR #139, choosing to record and merge, and the native change was finalized and archived. The story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-support-macos-and-linux-with-windows-through-wsl-and-drop-the.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17/)
