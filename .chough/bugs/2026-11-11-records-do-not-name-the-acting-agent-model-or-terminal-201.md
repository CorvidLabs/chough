---
kind: bug
id: 2026-11-11-records-do-not-name-the-acting-agent-model-or-terminal-201
aliases: [BUG-records-do-not-name-the-acting-agent-model-or-terminal-201-6efa997f8a66de3a6072fdd8]
title: Records do not name the acting agent, model or terminal (#201)
status: verified
classification: product-defect
scope: in-scope
impact: "Several agents' work cannot be told apart in the history (GitHub #201, folded from PR #233)"
criteria: [LIFE-17, FLOW-16]
requirements: [REQ-history-002]
cases: [TC-records-actor-recorded]
evidence:
  - OBS-tc-records-actor-recorded-556298a33aba7c34ff56307b
  - OBS-tc-records-actor-recorded-838ca2648f678ed5413a44a5
  - OBS-tc-records-actor-recorded-3926d24155e0fa587813f2f6
  - OBS-tc-records-actor-recorded-6675479880e75161b11ac3e3
  - OBS-tc-records-actor-recorded-9b46ed4d862ed4cb1bd0ad1f
receipts:
  - id: 2026-11-11-tc-records-actor-recorded
    aliases: OBS-tc-records-actor-recorded-556298a33aba7c34ff56307b
    claim: "Ran cargo test --locked --test actor for TC-records-actor-recorded: failed (exit 101)."
    result: failed
    command: cargo test --locked --test actor
    revision: 82c85c99483082130a7bb306fad29262c7578b2c
    environment: local-macos-arm64
    content: 1a60927d1ea0f042ab8f9e6a210a484b93756612fd5d4975f6d790ee833764f1
    fingerprint: sha256:b6a35b94b860da14e6a3d4f2dd8dab2d8ea1ba9e508ead1a8e006eaeedb5d451
    case: TC-records-actor-recorded
    case_digest: f394eb20565557b68877bbd777a833e4dd735c7bb72e6b7ef7088a18b5151361
    observed: 2026-11-11T19:07:03Z
    duration_seconds: 1
  - id: 2026-11-11-tc-records-actor-recorded-2b3034
    aliases: OBS-tc-records-actor-recorded-838ca2648f678ed5413a44a5
    claim: "Ran cargo test --locked --test actor for TC-records-actor-recorded: passed (exit 0)."
    result: passed
    command: cargo test --locked --test actor
    revision: 82a7392dda5b59925f1dad704d2c6c2778b23b8b
    environment: local-macos-arm64
    content: c0ad4a455cdd87cc042886e343d1ad1a31016c6ee2b83c35f413758823f87006
    fingerprint: sha256:a4205b46a7da5d7e6b46ba25a258196b88d617b3958f59096791ae4460778b0d
    case: TC-records-actor-recorded
    case_digest: f394eb20565557b68877bbd777a833e4dd735c7bb72e6b7ef7088a18b5151361
    observed: 2026-11-11T19:09:20Z
    duration_seconds: 1
  - id: 2026-11-11-tc-records-actor-recorded-c65451
    aliases: OBS-tc-records-actor-recorded-3926d24155e0fa587813f2f6
    claim: "Ran cargo test --locked --test actor --test bugs -- the_override_variables_name_the_agent_model_and_terminal claude_code_and_hc_fill_in_without_overrides with_nothing_set_no_entry_has_an_actor a_terminal_holding_a_home_path_is_left_out_with_a_warning bugs_record_the_acting_agent for TC-records-actor-recorded: passed (exit 0)."
    result: passed
    command: cargo test --locked --test actor --test bugs -- the_override_variables_name_the_agent_model_and_terminal claude_code_and_hc_fill_in_without_overrides with_nothing_set_no_entry_has_an_actor a_terminal_holding_a_home_path_is_left_out_with_a_warning bugs_record_the_acting_agent
    revision: 2e11a49df2c314f087abb61f2968cb74124a6020
    environment: local-macos-arm64
    content: cf2d6bdeab43a87c2e6354e6f42ebbedf17827e283fe58ecd3bb955fc1ec1b27
    fingerprint: sha256:21b6ac55604aca07e678e5981d9f65b47018ae768852a6d1d1d8916249111462
    case: TC-records-actor-recorded
    case_digest: 613bd06bf2995ed86f7ebe465c6ea5a4d197a4049521a53fb763b98d412fae6c
    observed: 2026-11-11T19:16:55Z
    duration_seconds: 1
  - id: 2026-11-11-tc-records-actor-recorded-cdf7c3
    aliases: OBS-tc-records-actor-recorded-6675479880e75161b11ac3e3
    claim: "Ran cargo test --locked --test actor -- --exact the_evidence_entry_names_the_acting_agent for TC-records-actor-recorded: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test actor -- --exact the_evidence_entry_names_the_acting_agent
    revision: 69b561549360301bc3379d3834e06de2e739cd43
    environment: local-macos-arm64
    content: 389ccff3c31e9305d2653db2f423dedfe01b3575dfb1d6620ca035fc4cbbf055
    fingerprint: sha256:aee4feded88cb121a20ad95e5990a9668a2bf868f9623c689b83fdb098c33905
    case: TC-records-actor-recorded
    case_digest: f81b7ba25b0c78862e9ead7eee54fbc27069a367ad1bb568e7f08e183b9cc23f
    observed: 2026-11-11T23:43:38Z
    duration_seconds: 1
  - id: 2026-11-11-tc-records-actor-recorded-50afbe
    aliases: OBS-tc-records-actor-recorded-9b46ed4d862ed4cb1bd0ad1f
    claim: "Ran cargo test --locked --test actor --test bugs -- the_override_variables_name_the_agent_model_and_terminal claude_code_and_hc_fill_in_without_overrides with_nothing_set_no_entry_has_an_actor a_terminal_holding_a_home_path_is_left_out_with_a_warning the_evidence_entry_names_the_acting_agent bugs_record_the_acting_agent for TC-records-actor-recorded: passed (exit 0)."
    result: passed
    command: cargo test --locked --test actor --test bugs -- the_override_variables_name_the_agent_model_and_terminal claude_code_and_hc_fill_in_without_overrides with_nothing_set_no_entry_has_an_actor a_terminal_holding_a_home_path_is_left_out_with_a_warning the_evidence_entry_names_the_acting_agent bugs_record_the_acting_agent
    revision: 9490c5f1eac3a5f0cc75fb13cb5f2fef50db94c7
    environment: local-macos-arm64
    content: 538d6cb893605c92e86ad45e97f59a674bc6cedef01ecd64432646ed2a6ee33e
    fingerprint: sha256:b322cc0dcacb094a1e960dee909c376e66a0fcc027513f7f7aeb6bb3dad0a929
    case: TC-records-actor-recorded
    case_digest: 3b5df40eba9a911d1e45e17d5a3b8e63fd9a4a4835d0ef0ae25d933c37caddb3
    observed: 2026-11-11T23:46:54Z
    duration_seconds: 2
environment: local-macos-arm64
revision: 99b2664f637137a402777a2e023ee66321ffce62
found_by: "GitHub issue #201; red on 0.5.0 at leif/0.6.0-features-red 82c85c99"
related: []
created: 2026-11-11T19:07:33Z
updated: 2026-11-11T23:47:07Z
---
# Records do not name the acting agent, model or terminal (#201)

**What happened:** with CHOUGH_ACTOR, CHOUGH_ACTOR_MODEL and CHOUGH_HC_TERMINAL set, no Chough record names the agent that wrote it
Actual: no record under .chough/ holds the agent or terminal

**Steps:** Set the actor variables and record a stage iteration
1. chough init
2. CHOUGH_ACTOR=<agent> CHOUGH_ACTOR_MODEL=<model> CHOUGH_HC_TERMINAL=<terminal> chough stage iterate --note round

**Expected:** the Timeline entry, evidence entry and bug record the acting agent, model and terminal, each only when known

**Why it matters:** Several agents' work cannot be told apart in the history (GitHub #201, folded from PR #233)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T19:07:33Z · Reported (GitHub issue #201; red on 0.5.0 at leif/0.6.0-features-red 82c85c99); evidence: OBS-tc-records-actor-recorded-556298a33aba7c34ff56307b
- 2026-11-11T19:09:40Z · Timeline entries fixed by plan steps 6 and 7 (PR #238): the 'by' field names agent, model and hc terminal. TC-records-actor-recorded passed for Timeline entries; evidence entries (step 4) and bugs (step 5) join the case at integration, so this stays open until then.; evidence: OBS-tc-records-actor-recorded-838ca2648f678ed5413a44a5
- 2026-11-11T19:17:10Z · Bugs joined at integration of plan step 5 (t38b): TC-records-actor-recorded now also runs bugs_record_the_acting_agent, and passed. Evidence entries (step 4) still to join.; evidence: OBS-tc-records-actor-recorded-3926d24155e0fa587813f2f6
- 2026-11-11T23:45:43Z · The evidence part of #201: on 0.5.0 neither evidence add nor cases run records the actor (TC-records-actor-recorded, red at 69b56154 on leif/0.6.0-records-red).; evidence: OBS-tc-records-actor-recorded-6675479880e75161b11ac3e3
- 2026-11-11T23:47:07Z · status: open -> verified. Green on integration: TC-records-actor-recorded passes with the evidence part (5 actor tests and the bug test); Timeline entries, evidence entries, case runs and bugs name the acting agent.; evidence: OBS-tc-records-actor-recorded-9b46ed4d862ed4cb1bd0ad1f
