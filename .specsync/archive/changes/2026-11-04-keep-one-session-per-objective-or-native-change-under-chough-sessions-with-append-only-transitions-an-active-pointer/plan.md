---
change: keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer
artifact: plan
---

# Plan

## Approach

Resolve the checkpoint path in one place and add a small `sessions` source file with
the `session` and `inbox` commands, keeping legacy single-session projects
byte-identical.

## Out of scope

Moving bugs, cases, evidence, consent records outside the checkpoint or validation
records per session; cross-worktree pointers; deleting archived sessions.

## Steps

1. hi/drive.md intent wording as in #68 (DRIVE-3 unchanged).
2. Failing-first tests in tests/sessions.rs, including a real-Git merge.
3. src/sessions.rs, path resolution in src/checkpoint.rs, commands and init refusal
   in src/main.rs, protection sources in src/history.rs.
4. .gitattributes, docs/cli.md, CHANGELOG.md, checkpoint skill reference and its
   installed copy.
5. `specsync change check --commit` and `fledge lanes run verify`.

## Risks

Adopting deletes `.chough/session.json`, so a branch still editing it conflicts once
(modify/delete); docs say to adopt on the main line first. Older Chough binaries do
not know sessions mode and read a missing `.chough/session.json`, so they refuse
rather than write a second checkpoint.

## Constraints consulted

One writable checkpoint at a time (REQ-cli-007, REQ-history-002); native SpecSync
state decides gates; Chough records no human approval or review; no new dependency.
