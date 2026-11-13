---
kind: evidence
id: 2026-11-13-with-the-base-set-to-the-commit-itself-as-a-scheduled-run-now
aliases: []
claim: "With the base set to the commit itself, as a scheduled run now passes, the archive inspector exited 0 on main 38fe24c0: shard 1/4 walked 1 of 1 PR revision and authenticated 45 of 177 archives. This shows a github.sha base is a real base the inspector accepts; the other three shards were not run locally"
result: passed
evidence_kind: structural
method: python3 -B .github/scripts/check-specsync-archive.py --base <HEAD> --shard 1/4 in a clean detached worktree of main 38fe24c0, under the heavy-tests lock
revision: a2d271c9c60126932d4dd03a40ddc5433419eb25
content: ccd87ffdd45b3166049b6bc37bf8988c0bc7e17e3c1886e60f4e743bd4f3ae3d
environment: local-macos-arm64
criteria: [LIFE-21]
requirements: [REQ-cli-064]
contracts: []
limitations: []
observed: 2026-11-13T06:18:36Z
actor:
  - agent: claude-code
    terminal: t38r
---
**With the base set to the commit itself, as a scheduled run now passes, the archive inspector exited 0 on main 38fe24c0: shard 1/4 walked 1 of 1 PR revision and authenticated 45 of 177 archives. This shows a github.sha base is a real base the inspector accepts; the other three shards were not run locally.**

Recorded as passed in local-macos-arm64 at revision a2d271c9c601, observed 2026-11-13T06:18:36Z.
