---
kind: bug
id: 2026-11-12-four-active-cases-link-requirements-this-change-removes-so
aliases:
  - BUG-four-active-cases-link-requirements-this-change-removes-so-cases-check-c6eb528a8fa8521f0b78c329
title: Four active cases link requirements this change removes, so cases check fails once the deltas are materialized
status: verified
classification: product-defect
scope: in-scope
impact: The verify and ci lanes fail after the archive
criteria: []
requirements: [REQ-cli-009]
cases: []
evidence:
  - OBS-tc-cli-every-active-case-links-a-current-requirement-20ad4bde2f4fa2c181290eca
  - OBS-tc-cli-every-active-case-links-a-current-requirement-dbe8f8892c381de321e7bc70
receipts:
  - id: 2026-11-12-tc-cli-every-active-case-links-a-current-requirement
    aliases: OBS-tc-cli-every-active-case-links-a-current-requirement-20ad4bde2f4fa2c181290eca
    claim: "Ran cargo test --locked --test spec_split -- --exact every_active_case_links_a_requirement_the_specs_hold for TC-cli-every-active-case-links-a-current-requirement: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test spec_split -- --exact every_active_case_links_a_requirement_the_specs_hold
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:503f9dbc3e93e046b5a0e51794217ec60cba53e2591ee292f88172a4d2a16e24
    case: TC-cli-every-active-case-links-a-current-requirement
    case_digest: 1fffb3463da2a8b0e8291fa03f51d0c2144ae174857f86f3344ac5ddb4a33c29
    observed: 2026-11-12T01:59:29Z
    duration_seconds: 0
  - id: 2026-11-12-tc-cli-every-active-case-links-a-current-requirement-1a2f10
    aliases: OBS-tc-cli-every-active-case-links-a-current-requirement-dbe8f8892c381de321e7bc70
    claim: "Ran cargo test --locked --test spec_split -- --exact every_active_case_links_a_requirement_the_specs_hold for TC-cli-every-active-case-links-a-current-requirement: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test spec_split -- --exact every_active_case_links_a_requirement_the_specs_hold
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:a4eed2d7ae17a020ce510b6dd621abff3f81124183efcebd7c75ff55f69863da
    case: TC-cli-every-active-case-links-a-current-requirement
    case_digest: 1fffb3463da2a8b0e8291fa03f51d0c2144ae174857f86f3344ac5ddb4a33c29
    observed: 2026-11-12T02:17:21Z
    duration_seconds: 0
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 18025ad47c730af7589317f76608634b2487896f
found_by: t37j, test stage, areas sessions and bugs, rehearsal at 367dc86b
related: []
created: 2026-11-12T01:41:11Z
updated: 2026-11-12T02:18:07Z
---
# Four active cases link requirements this change removes, so cases check fails once the deltas are materialized

**What happened:** After materialization (and so after archive) chough cases check refuses TC-history-session-use-waits-for-stage-advance naming REQ-history-008 not found; TC-history-0054 and TC-history-split-keeps-requirement-wording link REQ-history-001 and REQ-history-004
Actual: The cases still link REQ-history-001, REQ-history-004 and REQ-history-008

**Steps:** The verify or ci lane's cases step on a tree where the change's REMOVED requirements are gone

**Expected:** Every active case links requirements that exist after the change (REQ-cli-009)

**Why it matters:** The verify and ci lanes fail after the archive

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:41:11Z · Reported (t37j, test stage, areas sessions and bugs, rehearsal at 367dc86b)
- 2026-11-12T02:01:23Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-cli-every-active-case-links-a-current-requirement failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-cli-every-active-case-links-a-current-requirement-20ad4bde2f4fa2c181290eca
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-cli-every-active-case-links-a-current-requirement passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-cli-every-active-case-links-a-current-requirement-dbe8f8892c381de321e7bc70
