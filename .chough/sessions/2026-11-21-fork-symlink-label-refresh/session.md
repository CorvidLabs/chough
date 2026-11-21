---
kind: session
id: 2026-11-21-fork-symlink-label-refresh
aliases: [fork-symlink-label-refresh]
objective: A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated.
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
pr: "https://github.com/CorvidLabs/chough/pull/443"
criteria: [LIFE-31]
requirements: [REQ-cli-070]
cases:
  - TC-cli-optional-github-stage-labels
  - TC-cli-optional-github-stage-labels-match-the-main-stages
  - TC-cli-optional-github-stage-labels-init-does-not-install
design: []
files: []
observations:
  - 2026-11-21-tc-cli-optional-github-stage-labels-099eb9
  - 2026-11-21-tc-cli-optional-github-stage-labels-match-the-main-stages-1bd3ff
  - 2026-11-21-tc-cli-optional-github-stage-labels-init-does-not-install-6ea2e8
  - 2026-11-21-cargo-test-locked-test-github-labels-passed-5-tests-including
  - 2026-11-21-fledge-lanes-run-verify-completed-all-11-steps-specsync-6-4-0
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
    holds: accept
    drives: review
    opened: 2026-11-21T17:36:14Z
run_consents: []
next: The implementation review is open. Do not merge.
---
# A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated.

## Goal

A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated.

## Where it stands

- Fork pull requests skip the label job, symlinked session files are not read, and a matching stage label is not recreated.
- Recorded passing runs of the three bound label cases, the five github_labels tests, and the verify lane.

## Decisions and why

- The review of pull request 443 authorizes three fixes only: skip the label job for a fork pull request, do not read a symlinked session file, and create a stage label only when it is missing or its color or description differs. Reopening the archived change is out of scope. Merging is not authorized.

## What was verified

- [Ran cargo test --locked --test github_labels -- --exact the_script_labels_one_main_stage_and_refuses_anything_else for TC-cli-optional-github-stage-labels: passed (exit 0, tests run 1).](evidence/2026-11-21-tc-cli-optional-github-stage-labels-099eb9.md)
- [Ran cargo test --locked --test github_labels -- --exact the_template_follows_the_main_stages_and_chough_runs_that_copy for TC-cli-optional-github-stage-labels-match-the-main-stages: passed (exit 0, tests run 1).](evidence/2026-11-21-tc-cli-optional-github-stage-labels-match-the-main-stages-1bd3ff.md)
- [Ran cargo test --locked --test github_labels -- --exact chough_init_does_not_install_the_label_workflow for TC-cli-optional-github-stage-labels-init-does-not-install: passed (exit 0, tests run 1).](evidence/2026-11-21-tc-cli-optional-github-stage-labels-init-does-not-install-6ea2e8.md)
- [cargo test --locked --test github_labels passed 5 tests, including the_script_ignores_a_symlinked_session and the_workflow_does_not_recreate_matching_labels. Those two have no case binding: adding one now would touch a case file outside the approved paths.](evidence/2026-11-21-cargo-test-locked-test-github-labels-passed-5-tests-including.md)
- [fledge lanes run verify completed all 11 steps. SpecSync 6.4.0 and Fledge 1.8.0 were placed first on PATH because the copies found first on this machine were newer than the CI pins.](evidence/2026-11-21-fledge-lanes-run-verify-completed-all-11-steps-specsync-6-4-0.md)

## Open threads

## Timeline

- 2026-11-21T17:09:54Z · start · intent → intent · A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated. ^t-83738b
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
- 2026-11-21T17:17:22Z · advance · intent → design ·  ^t-c7fe91
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
- 2026-11-21T17:17:22Z · skip · design → plan · This is a workflow and script safety fix. Nothing visual changes. ^t-331716
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
- 2026-11-21T17:17:45Z · direct-native-write · plan → plan · a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state draft → approved ^t-b60064
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - direct_write:
    - change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: draft
    - after_state: approved
  - by: unknown agent (hc t10j)
- 2026-11-21T17:17:45Z · advance · plan → build · native:approved ^t-cf128b
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - gate: native:approved
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
- 2026-11-21T17:30:01Z · advance · build → test · native:approved ^t-134ef0
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - gate: native:approved
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
- 2026-11-21T17:36:07Z · advance · test → accept · native:verifying ^t-103582
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - gate: native:verifying
  - revision: 178b66a1607e3d830bbae4de41ce33996f057ef7
  - by: unknown agent (hc t10j)
- 2026-11-21T17:36:14Z · gate-opened · accept → accept · Implementation review of a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not ^t-ced641
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - revision: 178b66a1607e3d830bbae4de41ce33996f057ef7
  - holds: accept
  - drives: review
  - by: unknown agent (hc t10j)
