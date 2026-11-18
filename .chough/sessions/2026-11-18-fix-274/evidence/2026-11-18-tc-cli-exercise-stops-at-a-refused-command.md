---
kind: evidence
id: 2026-11-18-tc-cli-exercise-stops-at-a-refused-command
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact exercise_stops_at_a_refused_bug_update for TC-cli-exercise-stops-at-a-refused-command: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-exercise-stops-at-a-refused-command]
case_digest: [482f86e8f90324d76aae8eca4f1812350188a29e6de1f961fda3c01036941b56]
command: [cargo, test, --locked, --test, cli, --, --exact, exercise_stops_at_a_refused_bug_update]
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: be6c9f126e448bd33945a52261d55db2a9677d3d
content: 7970ad66ef82420b200d66290b1de8f8db7041c5e92c34777466ec5966250a22
environment: local-macos-arm64
fingerprint: sha256:74a1dce2c754ff42ee74c841fc412d7e7f616bbc3bd7c4b4f997c69bf125c5b0
criteria: [DRIVE-19]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T16:07:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test cli -- --exact exercise_stops_at_a_refused_bug_update for TC-cli-exercise-stops-at-a-refused-command: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision be6c9f126e44, observed 2026-11-18T16:07:47Z. The raw log stays in the machine-local store.
