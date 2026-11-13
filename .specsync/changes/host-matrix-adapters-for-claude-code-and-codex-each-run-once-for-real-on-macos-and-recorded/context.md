---
change: host-matrix-adapters-for-claude-code-and-codex-each-run-once-for-real-on-macos-and-recorded
artifact: context
---

# Context

PR 2 of 5 for #218 (bar item 4 of the 1.0 bar, LIFE-21): the first real agent hosts in
the host-matrix harness that #254 added (REQ-cli-066). It is stacked on #254's branch,
`claude/218-host-matrix-selftest`, and is retargeted to main once #254 merges.

## Leif's decisions

- Scope, at 2026-11-13T14:30Z, in the coordinator chat (Claude Code, hc t370), relayed by
  claude-code-t370. Asked: 'Chough 1.0 harness PR 2 (#218): plug Claude Code and Codex
  into the harness and run each once for real on this Mac, 5 turns each on your
  subscriptions. Each runs only inside a throwaway folder, with no permission prompts,
  and Codex's sandbox off there (it blocks git commits). Failures get recorded honestly,
  not tuned until they pass. Note: Codex hit its usage limit earlier today. Approve this
  scope?' He chose "Approve (Recommended)" (option text: 't38q builds it after #254
  merges.'). t370 added that a Codex usage limit is recorded as UNAVAILABLE, with its
  message, rather than waited on.
- Start, at 2026-11-13T14:48Z, directly in Claude Code hc t38q. Asked by claude-code-t38q:
  'Start PR 2 now on top of #254?' He chose "Start now, stacked (Recommended)" (option
  text: 't38q builds PR 2 on a branch stacked on #254 and retargets it to main once #254
  merges. Real Claude Code and Codex runs happen now.').

## Engineering choices by claude-code-t38q (not Leif's)

- Hosts run with the user's own configuration, as they are used day to day. Claude
  Code's `--bare` mode skips that configuration but accepts only an API key, not the
  subscription login.
- A turn that cannot run now (no login, a usage limit) exits 75, and the leg reads
  UNAVAILABLE with the reason, so a usage limit is not reported as a product failure.
- The shared turn prompt tells the host not to start other agents, terminals or hc: the
  user's configuration includes Herdr guidance to delegate through `hc run`, which would
  take work outside the throwaway sample.
- Real legs are recorded as agent-assisted case runs with their receipts. A bound case
  run may last at most 3600 s, and a real five-turn leg can take longer.
