---
kind: evidence
id: 2026-11-21-tc-cli-optional-github-stage-labels-match-the-main-stages-1bd3ff
aliases: []
claim: "Ran cargo test --locked --test github_labels -- --exact the_template_follows_the_main_stages_and_chough_runs_that_copy for TC-cli-optional-github-stage-labels-match-the-main-stages: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-optional-github-stage-labels-match-the-main-stages]
case_digest: [d1365a11846e1f2e378afd7d39a56fb5380c214d502914b54262ca4ee468863b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - github_labels
  - "--"
  - "--exact"
  - the_template_follows_the_main_stages_and_chough_runs_that_copy
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 178b66a1607e3d830bbae4de41ce33996f057ef7
content: fa88d8509c33bb33f34fb1a4445868ab8afad994e194116155fdd1d326830d07
environment: local-macos-arm64
fingerprint: sha256:bc3e23dca03e3ceb9ca3198bf71a2c3387ad856c23d2bf21dc8d7c8ff56a0945
criteria: [LIFE-31]
requirements: [REQ-cli-070]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-21T17:35:21Z
actor:
  - terminal: t10j
---
**Ran cargo test --locked --test github_labels -- --exact the_template_follows_the_main_stages_and_chough_runs_that_copy for TC-cli-optional-github-stage-labels-match-the-main-stages: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 178b66a1607e, observed 2026-11-21T17:35:21Z. The raw log stays in the machine-local store.
