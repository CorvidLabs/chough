---
change: host-matrix-adapters-for-claude-code-and-codex-each-run-once-for-real-on-macos-and-recorded
artifact: plan
---

# Plan

## Approach

Two adapters, one file each:

- Claude Code: `claude -p --permission-mode bypassPermissions --output-format stream-json`.
- Codex: `codex exec --dangerously-bypass-approvals-and-sandbox --ephemeral --json`.

Each uses the user's own configuration, and its reply is the host's final message.
`run.sh` learns one exit code: 75 from `host_turn` means the host cannot run now, so the
leg ends UNAVAILABLE with the reason and the log's last lines. The shared prompt gains
one line keeping the host inside the sample. Then each host runs once for real on macOS,
and the result is recorded honestly.

## Out of scope

- Kimi and Grok (PR 3).
- The migration scenario (PR 4).
- The Linux legs and the release gate (PR 5).
- The local agent (#220).
- Tuning a host until it passes. A failed leg is recorded with its bugs; a harness fix
  that makes passing easier comes back to Leif.

## Steps

1. Red: the self-test expects a turn that exits 75 to read UNAVAILABLE. Add its case and
   record the failing run.
2. Teach `run.sh` exit 75, and add the two adapters and the prompt line. The self-test
   goes green under dash and bash.
3. Run Claude Code once for real, then Codex, and record each as an agent-assisted case
   run with its receipt.
4. Docs: the README, docs/integrations.md (observed results per host) and CHANGELOG.

## Risks

- Codex may be at its usage limit, in which case its leg reads UNAVAILABLE.
- A real host may fail a check the stand-in passes. That is recorded, not hidden.
- Cost: about five agent turns per host, on Leif's subscriptions.

## Constraints consulted

The cli lessons in `specs/cli/cli.3md`, and #254's review: the harness stays POSIX sh and
runs under dash.
