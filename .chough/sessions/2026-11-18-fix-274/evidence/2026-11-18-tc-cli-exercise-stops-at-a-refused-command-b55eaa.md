---
kind: evidence
id: 2026-11-18-tc-cli-exercise-stops-at-a-refused-command-b55eaa
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
revision: c8963ffe0a3c319648675c435b89fa459cc998ec
content: 2ca2bbdc7593da4bc57e391572f68ad9a9ddc891e96de0cd2a7a77930fcd0e47
environment: local-macos-arm64
fingerprint: sha256:f09b0f4f42ad134332e0910a1f63c42ea2146559cd579c7c4a8626607a63f905
criteria: [DRIVE-19]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T16:54:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact exercise_stops_at_a_refused_bug_update for TC-cli-exercise-stops-at-a-refused-command: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c8963ffe0a3c, observed 2026-11-18T16:54:18Z. The raw log stays in the machine-local store.
