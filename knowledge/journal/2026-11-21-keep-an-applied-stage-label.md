---
id: 2026-11-21-keep-an-applied-stage-label
title: Keep an applied stage label
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-21
time: 2026-11-21T18:34:24Z
kind: shipped
links:
  - https://github.com/CorvidLabs/chough/pull/443
  - .chough/archive/sessions/2026-11-21-keep-an-applied-stage-label.md
  - .specsync/archive/changes/2026-11-21-a-pull-request-that-already-has-its-stage-label-keeps-that-label/
record: .chough/archive/sessions/2026-11-21-keep-an-applied-stage-label.md
actor:
  - terminal: t10j
---
# Keep an applied stage label

A pull request that already has its stage label keeps that label. A different stage label is still removed, and a missing one is still added.

Verified on this machine: the keep-label case passed, cargo test --test github_labels passed, and the verify lane completed 11 steps with SpecSync 6.4.0 and Fledge 1.8.0.

Next: archive the open-session change on pull request 443. Merging was not requested.

Read more: [the pull request](https://github.com/CorvidLabs/chough/pull/443) · [the story](../../.chough/archive/sessions/2026-11-21-keep-an-applied-stage-label.md) · [the archived change](../../.specsync/archive/changes/2026-11-21-a-pull-request-that-already-has-its-stage-label-keeps-that-label/)
