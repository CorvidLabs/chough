---
change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
artifact: plan
---

# Plan

## Approach

Add a POSIX shell harness under `tools/host-matrix/`, in the style of
`examples/feature-bug-loop.sh` (sh and jq, the real chough, hi, specsync, cargo and
git). `run.sh <host>` seeds a throwaway Git copy of `examples/shipping` without its
`patches/`, then runs five fresh host turns from fixed prompts: init and intent; plan;
build, test and accept; done; and a cold resume. After each turn it checks the
sample's files, Git and `chough next --json`, and stands in for the human gate as
`SIMULATED-host-matrix-not-a-human`. At the end `shape.sh` reduces the records to a
host-independent shape compared with `expected-shape.json`. A scripted stand-in host
(`stand-in/`) takes each turn from the project's own records with no model, and
`selftest.sh` runs it once correctly and once per known fault.

## Out of scope

- Adapters for Claude Code, Codex, Kimi, Grok and the local agent (later changes, with
  their first recorded legs).
- The migration scenario, the Linux legs in Docker (LIFE-28), the local-agent profile
  (LIFE-29, #220) and the release gate (LIFE-30).
- Any change to `src/`, the skill or the verify lane; Windows.

## Steps

1. Red: add the case TC-cli-host-matrix-selftest and `selftest.sh`, run the case and
   record it failing (the harness does not exist yet).
2. Add the harness, the stand-in host, the prompts, the acceptance test and the
   expected shape; rerun the case to green.
3. Add the Fledge tasks, the hosted CI job on Ubuntu and macOS, the docs and the
   change log entry.
4. Scoped check, the full suite, change audit, then the review gate.

## Risks

- A slow self-test: it runs eight harness passes; about three minutes on macOS arm64.
  It has its own job with a 20-minute limit.
- macOS and Linux shell differences (BSD and GNU tools); the CI job runs both.
- The shape could be too strict for real hosts. The later adapter changes will show
  this, and any loosening needs its own reviewed change.

## Constraints consulted

The cli lessons in `specs/cli/cli.3md`: a local Git identity in disposable
repositories; gates and post-verification steps as plain text in tasks; the delta is
frozen at approval; new REQ IDs need the scoped check before cases link them; confirm
an external agent can start before relying on it.
