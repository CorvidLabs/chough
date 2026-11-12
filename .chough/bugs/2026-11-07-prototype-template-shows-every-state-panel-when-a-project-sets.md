---
kind: bug
id: 2026-11-07-prototype-template-shows-every-state-panel-when-a-project-sets
aliases:
  - BUG-prototype-template-shows-every-state-panel-when-a-project-sets-display-073394402cc923c41c0b501c
title: Prototype template shows every state panel when a project sets display on the state sections (#58)
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: []
requirements: []
cases: [TC-cli-prototype-hidden-panels-stay-hidden]
evidence:
  - OBS-tc-cli-prototype-hidden-panels-stay-hidden-f84e099781900ebb1a85f512
  - OBS-tc-cli-prototype-styles-0-4-0-known-outdated-73623b3db9dfcb5ff747c4b0
  - OBS-tc-cli-prototype-styles-0-4-0-known-outdated-264ebbad7b6036387c10a384
  - OBS-tc-cli-prototype-hidden-panels-stay-hidden-1ca60531271f1a597817af74
receipts:
  - id: 2026-11-07-tc-cli-prototype-hidden-panels-stay-hidden
    aliases: OBS-tc-cli-prototype-hidden-panels-stay-hidden-f84e099781900ebb1a85f512
    claim: "Ran cargo test --locked --test package -- --exact the_prototype_template_keeps_hidden_state_panels_hidden for TC-cli-prototype-hidden-panels-stay-hidden: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test package -- --exact the_prototype_template_keeps_hidden_state_panels_hidden
    revision: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
    environment: local-macos-arm64
    content: 118f451ed8dc4cb02b2f56565e973f6cbb34dc8e6afe323a3865e77d48567dd0
    fingerprint: sha256:af5692b53256bc7f0731e47d56fcdb55b2ece441253eaf27771b6d0d4e3e672e
    case: TC-cli-prototype-hidden-panels-stay-hidden
    case_digest: 5133db4628cfdddda84c9adfe39edc120a4c71b2fd20fb559031935c5ab7c966
    observed: 2026-11-07T02:29:03Z
    duration_seconds: 0
  - id: 2026-11-07-tc-cli-prototype-styles-0-4-0-known-outdated
    aliases: OBS-tc-cli-prototype-styles-0-4-0-known-outdated-73623b3db9dfcb5ff747c4b0
    claim: "Ran cargo test --locked --test package -- --exact the_prototype_styles_from_releases_0_1_0_to_0_4_0_are_known_outdated_and_edited_copies_stay for TC-cli-prototype-styles-0-4-0-known-outdated: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test package -- --exact the_prototype_styles_from_releases_0_1_0_to_0_4_0_are_known_outdated_and_edited_copies_stay
    revision: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
    environment: local-macos-arm64
    content: 118f451ed8dc4cb02b2f56565e973f6cbb34dc8e6afe323a3865e77d48567dd0
    fingerprint: sha256:60bb9927669c9cb3027f8e8f9c2ba09850dc5b024bec8c5b084966a149dce30b
    case: TC-cli-prototype-styles-0-4-0-known-outdated
    case_digest: d7bbbb193b4fe7068ec58676ad3929ba3e1949fda004c8dc7ccd5bb4baba0ac7
    observed: 2026-11-07T02:29:09Z
    duration_seconds: 1
  - id: 2026-11-07-tc-cli-prototype-styles-0-4-0-known-outdated-4207e9
    aliases: OBS-tc-cli-prototype-styles-0-4-0-known-outdated-264ebbad7b6036387c10a384
    claim: "Ran cargo test --locked --test package -- --exact the_prototype_styles_from_releases_0_1_0_to_0_4_0_are_known_outdated_and_edited_copies_stay for TC-cli-prototype-styles-0-4-0-known-outdated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_prototype_styles_from_releases_0_1_0_to_0_4_0_are_known_outdated_and_edited_copies_stay
    revision: 0d0233a082a9b181ac8aee5f44f2cac7a85df8d5
    environment: local-macos-arm64
    content: 8843444283725b49613a3f4143f993c9ec719d0d23aa7f70e08c5cd33cc9a35f
    fingerprint: sha256:375fc9afb9a333e32e2371759e26cbbb81082b38f4980af956e2cb5fc9de8258
    case: TC-cli-prototype-styles-0-4-0-known-outdated
    case_digest: d7bbbb193b4fe7068ec58676ad3929ba3e1949fda004c8dc7ccd5bb4baba0ac7
    observed: 2026-11-07T02:32:57Z
    duration_seconds: 1
  - id: 2026-11-07-tc-cli-prototype-hidden-panels-stay-hidden-f70106
    aliases: OBS-tc-cli-prototype-hidden-panels-stay-hidden-1ca60531271f1a597817af74
    claim: "Ran cargo test --locked --test package -- --exact the_prototype_template_keeps_hidden_state_panels_hidden for TC-cli-prototype-hidden-panels-stay-hidden: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_prototype_template_keeps_hidden_state_panels_hidden
    revision: 0d0233a082a9b181ac8aee5f44f2cac7a85df8d5
    environment: local-macos-arm64
    content: 8843444283725b49613a3f4143f993c9ec719d0d23aa7f70e08c5cd33cc9a35f
    fingerprint: sha256:01478642bfd3e14fd947136d18577d3b3cce101ab9a9de963b070813ab078d41
    case: TC-cli-prototype-hidden-panels-stay-hidden
    case_digest: 5133db4628cfdddda84c9adfe39edc120a4c71b2fd20fb559031935c5ab7c966
    observed: 2026-11-07T02:32:49Z
    duration_seconds: 0
environment: local-macos-arm64
revision: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
found_by: manual report
related: []
created: 2026-11-07T02:29:17Z
updated: 2026-11-07T02:33:09Z
---
# Prototype template shows every state panel when a project sets display on the state sections (#58)

**What happened:** In Leif's Raven session every prototype state (loading, empty, error, ready) showed at once; tests/package.rs the_prototype_template_keeps_hidden_state_panels_hidden fails on main f41e7ab4
Actual: templates/prototype/styles.css has no [hidden] rule, so every hidden state panel renders

**Steps:** A project rule that sets display on the template's state sections (for example .surface section { display: grid }) overrides the browser's [hidden] { display: none }

**Expected:** Only the selected state panel is visible whatever display rules a project adds

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-07T02:29:17Z · Reported (manual report); evidence: OBS-tc-cli-prototype-hidden-panels-stay-hidden-f84e099781900ebb1a85f512
- 2026-11-07T02:29:33Z · Also TC-cli-prototype-styles-0-4-0-known-outdated: the fix changes styles.css, so installs of the 0.1.0 to 0.4.0 bytes must read known-outdated; this run fails first because those bytes are still current; evidence: OBS-tc-cli-prototype-styles-0-4-0-known-outdated-73623b3db9dfcb5ff747c4b0
- 2026-11-07T02:33:09Z · Additional evidence.; evidence: OBS-tc-cli-prototype-styles-0-4-0-known-outdated-264ebbad7b6036387c10a384
- 2026-11-07T02:33:09Z · status: open -> verified; evidence: OBS-tc-cli-prototype-hidden-panels-stay-hidden-1ca60531271f1a597817af74
