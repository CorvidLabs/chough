---
id: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
state: archived
type: feature
base_commit: 598c4bc527007f9bebe53f7291e31130b2891561
---

# Host-matrix harness: one scripted Chough cycle per agent host, judged from the files, with a stand-in-host self-test in hosted CI on macOS and Linux

## Intent

Host-matrix harness: one scripted Chough cycle per agent host, judged from the files, with a stand-in-host self-test in hosted CI on macOS and Linux

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- sh tools/host-matrix/selftest.sh prints HOST MATRIX SELFTEST PASSED on macOS and on Linux: the stand-in good run is PASS and each known-bad run is FAIL on the check that names it
- Hosted CI runs the self-test as Host matrix self-test on ubuntu-latest and macos-latest
- The harness never records a decision in a person name: every stand-in gate decision and native approval or review names SIMULATED-host-matrix-not-a-human

## No-spec Rationale

Not applicable
