---
id: 2026-11-20-no-git-rewrites-of-a-worktree-while-another-agent-works-there
title: No Git rewrites of a worktree while another agent works there
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T02:16:30Z
kind: shipped
links:
  - 416
  - .chough/archive/sessions/2026-11-19-docs-387-no-git-while-agents-write.md
  - .specsync/archive/changes/2026-11-20-docs-387-no-git-while-agents-write/
record: .chough/archive/sessions/2026-11-19-docs-387-no-git-while-agents-write.md
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
# No Git rewrites of a worktree while another agent works there

The workflow skill's checkpoint reference, in the bundle and in an installed copy, and docs/cli.md each say not to run git pull, merge, rebase, checkout or stash in a worktree while another agent works there, why (such a command can replace a record another agent's Chough command just saved), and that a lone agent may run one between its own commands; the 0.6.1 digest of the reference is known, so a 0.6.1 install updates (#387).

Verified at 560a8780: all eleven verify steps and the case on macOS; tester2's PASS; corvid-agent's review at ddc210fb and recheck at 560a8780 (955 tests on Linux).

Read more: the pull request 416 · [the story](../../.chough/archive/sessions/2026-11-19-docs-387-no-git-while-agents-write.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-docs-387-no-git-while-agents-write/)
