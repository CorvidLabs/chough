---
change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
artifact: context
---

# Context

Chough's 1.0 bar item 4 (LIFE-21) needs a scripted end-to-end cycle that passes on
each supported agent host (Claude Code, Codex, Kimi, Grok and one local agent) on
macOS and Linux before each release (issue #218; docs/test-plan-1.0.md, bar 4). Issue
#220 qualifies one local agent through the same harness. Nothing runs that cycle
today: `examples/feature-bug-loop.sh` drives the real tools but plays every agent
step itself, so no host's skill discovery, resume or gate handling is exercised.

This change is the first of five planned pull requests for #218 (the plan, sent to the
coordinator on 2026-11-12, is outside the repository). It adds the harness and a
scripted stand-in host with no model, and proves the harness in hosted CI. The real
host adapters (`integrations/<host>/adapter.sh`), the migration scenario, the Linux
legs in Docker and the release gate follow in their own changes.

## Leif's decisions this change rests on

Leif answered four design questions in the coordinator chat (Claude Code, hc t370) at
2026-11-13T03:32Z; claude-code-t370 relayed his choices to claude-code-t38q, and they
are captured as LIFE-27 to LIFE-30:

- Run mode: "Plain runs, Herdr optional (Recommended)" (LIFE-27).
- Linux legs: "Docker on the Mac (Recommended)", with logins passed in only for the
  run (LIFE-28; a later change).
- Local agent: "Not sure: keep it generic (Recommended)", meaning any command plus a
  model name in one settings file (LIFE-29; #220, a later change).
- Blocking: "Next weekly release (Recommended)", for agents already passing; the
  local agent joins once qualified (LIFE-30; the release gate, a later change).

The questions as claude-code-t38q sent them to t370, which batched them for Leif:

- Q1. How should the test run each agent? A) One run per step with no Herdr needed,
  so the same script works on Linux; a Herdr version is optional for watching
  (Recommended). B) Always as Herdr terminals you can watch from your phone.
- Q2. Where should the Linux runs happen? A) A Docker container on the Mac, logged in
  with tokens passed in only for the run (Recommended). B) A separate Linux machine
  over ssh/hc. C) GitHub Actions started by hand before a release, with API keys
  stored as repo secrets.
- Q3. Which local agent are you getting? A) Not sure yet: keep it generic, any
  command plus a model name (Recommended). B) Pi with a local model server (oMLX or
  llama.cpp), as in the Mac Studio plan. C) Codex CLI or Kimi Code pointed at a local
  model (Ollama or LM Studio).
- Q4. When should a failed or missing agent run start blocking releases? A) From the
  next weekly release, for agents already passing; the local agent joins once it is
  qualified (Recommended). B) Only from 1.0; until then releases just link the
  results. C) Every release right away, all five agents.

## Engineering choices by claude-code-t38q (not Leif's)

- The harness lives in `tools/host-matrix/`; per-host adapters will sit at
  `integrations/<host>/adapter.sh`, beside #219's `integrations/<host>/hc-template.md`
  (layout agreed with ch10-flake, hc t38r).
- The self-test runs as its own hosted CI job on Ubuntu and macOS, not as a verify-lane
  step: the verify lane's step list is pinned by README wording and tests/docs.rs, and
  a separate job covers both supported platforms (FLOW-21).
- Each host turn runs with the starting agent's and terminal's environment cleared
  (`CLAUDECODE`, `CLAUDE_CODE_*` session variables, `HC_*`) and `CHOUGH_ACTOR` set to
  the host. A first stand-in run recorded `claude-code (hc t38q)`, the harness's own
  terminal, on the sample's records.

## For a session picking this up

- Turns are fresh host runs; the harness stands in for each human gate between them
  and labels every such decision SIMULATED (FLOW-10, FLOW-16).
- The sample's `patches/` hold the solution, so the harness removes them; the stand-in
  host copies them from the repository instead.
- Agents run without permission prompts only inside the throwaway folder (relayed by
  t370 with Leif's answers).
