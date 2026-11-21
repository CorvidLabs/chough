---
kind: session
id: 2026-11-21-optional-github-stage-labels
aliases: [optional-github-stage-labels]
objective: Ship an optional GitHub template that labels a pull request with the seven main Chough stages, in rainbow order, without Chough calling GitHub.
intent_scope: LIFE
modules: [cli]
stage: done
stage_status: archived
iteration: 1
native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
pr: "443"
criteria: [LIFE-31]
requirements: [REQ-cli-070]
cases:
  - TC-cli-optional-github-stage-labels
  - TC-cli-optional-github-stage-labels-match-the-main-stages
  - TC-cli-optional-github-stage-labels-init-does-not-install
observations:
  - 2026-11-21-tc-cli-optional-github-stage-labels
  - 2026-11-21-tc-cli-optional-github-stage-labels-match-the-main-stages
  - 2026-11-21-tc-cli-optional-github-stage-labels-init-does-not-install
title: Optional GitHub labels for the seven main stages
summary:
  - The optional integrations/github template labels a pull request with intent, design, plan, build, test, accept and done. Chough runs the same workflow and script. chough init does not install them, and the binary does not call GitHub.
  - "Three cases passed: the script labels one main stage and refuses anything else, the template matches the seven stages, and chough init does not install the workflow. Pull request 443's workflow run applied chough-stage:build. Raven draft 723 walked all seven colors."
  - Hosted CI is next, after the change is archived. The pull request is not merged.
closed: 2026-11-21
outcome: done
---
# Ship an optional GitHub template that labels a pull request with the seven main Chough stages, in rainbow order, without Chough calling GitHub.

## Goal

Ship an optional GitHub template that labels a pull request with the seven main Chough stages, in rainbow order, without Chough calling GitHub.

## Where it stands

- Optional GitHub stage-label template, dogfood copies, REQ-cli-070 and github_labels tests. change check passed.
- Pushed optional-github-stage-labels and opened pull request 443.

## Decisions and why

- Optional GitHub stage labels ship as a copied template. The user said to add that shape to Chough on 2026-11-21.
- Scope approved as grok-build under the user's 2026-11-21 words: ok go checkout the chough project and help add this. Limits: init does not install the workflow, src does not call GitHub.
- 0xLeif decided "Implementation review of optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages": "I approve continue" (Grok chat on 2026-11-21, relayed by grok-build); recorded by grok-build; a relayed claim, not authenticated; ran specsync change review optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages as 0xLeif with verdict pass

## What was verified

- Ran cargo test --locked --test github_labels -- --exact the_script_labels_one_main_stage_and_refuses_anything_else for TC-cli-optional-github-stage-labels: passed (exit 0, tests run 1). - fc6f4960bfe0; limits: inherited environment variables were not recorded ^2026-11-21-tc-cli-optional-github-stage-labels
- Ran cargo test --locked --test github_labels -- --exact the_template_follows_the_main_stages_and_chough_runs_that_copy for TC-cli-optional-github-stage-labels-match-the-main-stages: passed (exit 0, tests run 1). - 912839e73e2f; limits: inherited environment variables were not recorded ^2026-11-21-tc-cli-optional-github-stage-labels-match-the-main-stages
- Ran cargo test --locked --test github_labels -- --exact chough_init_does_not_install_the_label_workflow for TC-cli-optional-github-stage-labels-init-does-not-install: passed (exit 0, tests run 1). - b8e81b6b250e; limits: inherited environment variables were not recorded ^2026-11-21-tc-cli-optional-github-stage-labels-init-does-not-install

## Bugs

None.

## Open threads

## Timeline

- 2026-11-21T16:28:51Z · start · intent → intent · Ship an optional GitHub template that labels a pull request with the seven main Chough stages, in rainbow order, without Chough calling GitHub. ^t-683c1d
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - by: unknown agent (hc t10j)
- 2026-11-21T16:28:57Z · advance · intent → design · LIFE-31 records the intent: an optional copied template for the seven main stages. ^t-8abbd3
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - by: unknown agent (hc t10j)
- 2026-11-21T16:28:57Z · skip · design → plan · No visuals or interactions. This is an optional copied GitHub workflow and script. ^t-848271
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - by: unknown agent (hc t10j)
- 2026-11-21T16:35:09Z · direct-native-write · plan → plan · optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state draft → approved ^t-e3addf
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - direct_write:
    - change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: draft
    - after_state: approved
  - by: unknown agent (hc t10j)
- 2026-11-21T16:35:09Z · advance · plan → build · Definition approved for the optional GitHub stage-label template. ^t-cbc53d
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - gate: native:approved
  - revision: b5627125d1df640ae03882588e48b27f5f84f0c6
  - by: unknown agent (hc t10j)
- 2026-11-21T17:02:25Z · direct-native-write · build → build · optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-b83d47
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
  - direct_write:
    - change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: unknown agent (hc t10j)
- 2026-11-21T17:02:25Z · direct-native-write · build → build · optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state draft → approved ^t-ffd06c
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
  - direct_write:
    - change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: draft
    - after_state: approved
  - by: unknown agent (hc t10j)
- 2026-11-21T17:02:25Z · advance · build → test · native:verifying ^t-228404
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - gate: native:verifying
  - revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
  - by: unknown agent (hc t10j)
- 2026-11-21T17:02:42Z · advance · test → accept · native:verifying ^t-d11584
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - gate: native:verifying
  - revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
  - by: unknown agent (hc t10j)
- 2026-11-21T17:02:42Z · gate-opened · accept → accept · Implementation review of optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages ^t-6b67d8
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
  - holds: accept
  - drives: review
  - by: unknown agent (hc t10j)
- 2026-11-21T17:03:58Z · gate-closed · accept → accept · Implementation review of optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages ^t-f0a108
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - revision: 9e08bfa97251b22247dc2ffda9a9328105450ba1
  - relayed_by:
    - decided_by: 0xLeif
    - words: I approve continue
    - where: Grok chat on 2026-11-21, relayed by grok-build
    - recorded_by: grok-build
    - native: specsync change review optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
    - actor: 0xLeif
  - by: unknown agent (hc t10j)
- 2026-11-21T17:03:59Z · advance · accept → done · native:archived ^t-fa6002
  - native_change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
  - gate: native:archived
  - revision: 9e08bfa97251b22247dc2ffda9a9328105450ba1
  - by: unknown agent (hc t10j)

Closed 2026-11-21: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-21-tc-cli-optional-github-stage-labels
    claim: "Ran cargo test --locked --test github_labels -- --exact the_script_labels_one_main_stage_and_refuses_anything_else for TC-cli-optional-github-stage-labels: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test github_labels -- --exact the_script_labels_one_main_stage_and_refuses_anything_else
    revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
    environment: local-macos-arm64
    content: 58eff05268add768c550480edc21525594184aa7e3d8c0a4307da857d3224498
    fingerprint: sha256:fc6f4960bfe0d73b6a5133b306a9b026eaaa80a5846d7e204ecb704bba96710e
    case: TC-cli-optional-github-stage-labels
    case_digest: 48b35d451c81afec5ef2bdf8cf17f5737f53137f36ae3a5ec0c66ac5a290ba83
    criteria: LIFE-31
    requirements: REQ-cli-070
    limitations: inherited environment variables were not recorded
    observed: 2026-11-21T17:02:31Z
    duration_seconds: 1
    actor: unknown agent (hc t10j)
  - id: 2026-11-21-tc-cli-optional-github-stage-labels-match-the-main-stages
    claim: "Ran cargo test --locked --test github_labels -- --exact the_template_follows_the_main_stages_and_chough_runs_that_copy for TC-cli-optional-github-stage-labels-match-the-main-stages: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test github_labels -- --exact the_template_follows_the_main_stages_and_chough_runs_that_copy
    revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
    environment: local-macos-arm64
    content: 58eff05268add768c550480edc21525594184aa7e3d8c0a4307da857d3224498
    fingerprint: sha256:912839e73e2fd2fdee9248a2a88c97e5cb458ac5901538573597116b73cd8226
    case: TC-cli-optional-github-stage-labels-match-the-main-stages
    case_digest: d1365a11846e1f2e378afd7d39a56fb5380c214d502914b54262ca4ee468863b
    criteria: LIFE-31
    requirements: REQ-cli-070
    limitations: inherited environment variables were not recorded
    observed: 2026-11-21T17:02:31Z
    duration_seconds: 0
    actor: unknown agent (hc t10j)
  - id: 2026-11-21-tc-cli-optional-github-stage-labels-init-does-not-install
    claim: "Ran cargo test --locked --test github_labels -- --exact chough_init_does_not_install_the_label_workflow for TC-cli-optional-github-stage-labels-init-does-not-install: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test github_labels -- --exact chough_init_does_not_install_the_label_workflow
    revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
    environment: local-macos-arm64
    content: 58eff05268add768c550480edc21525594184aa7e3d8c0a4307da857d3224498
    fingerprint: sha256:b8e81b6b250eb5331a169dce918587c48c7535d4a072eaa1b33c9b43ff59231b
    case: TC-cli-optional-github-stage-labels-init-does-not-install
    case_digest: 8760f9980d16046a3b64e7f9b6ec481f6aa572808c6703224596e00a9efce75d
    criteria: LIFE-31
    requirements: REQ-cli-070
    limitations: inherited environment variables were not recorded
    observed: 2026-11-21T17:02:32Z
    duration_seconds: 0
    actor: unknown agent (hc t10j)
```
