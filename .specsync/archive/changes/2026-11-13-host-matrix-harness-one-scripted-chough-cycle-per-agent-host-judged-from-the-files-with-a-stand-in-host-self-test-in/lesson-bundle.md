# Lesson bundle: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Host-matrix harness: one scripted Chough cycle per agent host, judged from the files, with a stand-in-host self-test in hosted CI on macOS and Linux
- **Kind**: Feature
- **Specs**: cli
- **Paths**: .chough/cases/cli.host-matrix.cases.toml, .github/workflows/ci.yml, CHANGELOG.md, INTENT.md, docs/ci.md, docs/test-plan-1.0.md, fledge.toml, hi/life.md, tools/host-matrix/README.md, tools/host-matrix/acceptance.rs, tools/host-matrix/expected-shape.json, tools/host-matrix/prompts/common.md, tools/host-matrix/prompts/turn-1.md, tools/host-matrix/prompts/turn-2.md, tools/host-matrix/prompts/turn-3.md, tools/host-matrix/prompts/turn-4.md, tools/host-matrix/prompts/turn-5.md, tools/host-matrix/run.sh, tools/host-matrix/selftest.sh, tools/host-matrix/shape.sh, tools/host-matrix/stand-in/adapter.sh, tools/host-matrix/stand-in/agent.sh
- **Acceptance**: sh tools/host-matrix/selftest.sh prints HOST MATRIX SELFTEST PASSED on macOS and on Linux: the stand-in good run is PASS and each known-bad run is FAIL on the check that names it
- **Acceptance**: Hosted CI runs the self-test as Host matrix self-test on ubuntu-latest and macos-latest
- **Acceptance**: The harness never records a decision in a person name: every stand-in gate decision and native approval or review names SIMULATED-host-matrix-not-a-human

## Evidence

- Verification commit: `419ded0cb5468251d7b2a08c89f9377e053e06fd`
- Base commit: `598c4bc527007f9bebe53f7291e31130b2891561`
- Verified by: `specsync check --spec cli`

## From the change's context.md

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

The questions and option text Leif saw, as claude-code-t370 relayed them to
claude-code-t38q (coordinator chat, Claude Code hc t370, about 03:32Z):

- Q1 'Chough 1.0 agent-host harness (#218): how should it run each agent?' ->
  "Plain runs, Herdr optional (Recommended)" (option text: 'One run per step, so the
  same script works on Linux. A Herdr version is optional, for watching from your
  phone.').
- Q2 'Where should the Linux harness runs happen?' -> "Docker on the Mac
  (Recommended)" (option text: 'Agent logins are passed in only for the run.').
- Q3 'Which local agent are you getting tomorrow?' -> "Not sure: keep it generic
  (Recommended)" (option text: 'Any command plus a model name, set in one small
  settings file.').
- Q4 'When should a failed or missing agent run start blocking Chough releases?' ->
  "Next weekly release (Recommended)" (option text: 'For agents already passing. The
  local agent joins once it's qualified.').

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

## From the change's design.md

# Design

Nothing visual or interactive changes. The harness is a set of shell scripts:

- `run.sh` drives one host through five turns and judges them;
- `shape.sh` reduces a finished sample to a host-independent shape;
- `selftest.sh` runs the stand-in host correctly and with each fault;
- `stand-in/adapter.sh` and `stand-in/agent.sh` are the scripted host.

An adapter is a sourced shell file that defines `host_check`, `host_version` and
`host_turn <prompt> <log> <reply>`. It may also set `HOST_LABEL`, `HOST_INIT_ARGS`,
`HOST_SKILL`, `HOST_SKILL_READ` and `HOST_TURN_TIMEOUT`. The result is one JSON object:
the verdict, each failed check and a receipt. The receipt holds the host and its
version, the platform, the chough version and commit, the command, the SHA-256 of the
turn logs and the key failing lines. Raw logs stay in a temporary directory outside
the repository.

## From the change's testing.md

# Testing

REQ-cli-066 is verified by TC-cli-host-matrix-selftest, bound to
`sh tools/host-matrix/selftest.sh` with `expect_output` `HOST MATRIX SELFTEST PASSED`,
in `.chough/cases/cli.host-matrix.cases.toml`. It runs the harness eight times with the
stand-in host:

- the good run is PASS;
- uncommitted work fails "turn 1: the tree is committed";
- a skipped scope gate fails "turn 2: the scope gate is open";
- the host approving the change itself fails "turn 2: the change waits in draft";
- an extra product file fails "turn 3: only the allowed product files changed";
- a wrong cold resume fails "turn 5: the cold resume states stage done";
- a hung turn fails on its time limit, and its whole process group ends;
- a host that cannot run reads UNAVAILABLE.

Platforms: the case runs locally on macOS arm64, and the hosted job runs it on
`ubuntu-latest` and `macos-latest`. It is run once with `GIT_CONFIG_GLOBAL=/dev/null`,
as in hosted CI. The full `cargo test`, fmt, clippy and the verify lane run at the
final head.

## Where these lessons go

- `specs/cli/cli.3md`
