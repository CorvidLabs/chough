---
kind: evidence
id: 2026-11-21-tc-cli-optional-github-stage-labels-match-the-main-stages
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
revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
content: 58eff05268add768c550480edc21525594184aa7e3d8c0a4307da857d3224498
environment: local-macos-arm64
fingerprint: sha256:912839e73e2fd2fdee9248a2a88c97e5cb458ac5901538573597116b73cd8226
criteria: [LIFE-31]
requirements: [REQ-cli-070]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-21T17:02:31Z
actor:
  - terminal: t10j
---
**Ran cargo test --locked --test github_labels -- --exact the_template_follows_the_main_stages_and_chough_runs_that_copy for TC-cli-optional-github-stage-labels-match-the-main-stages: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9f265d9fbb3a, observed 2026-11-21T17:02:31Z. The raw log stays in the machine-local store.
