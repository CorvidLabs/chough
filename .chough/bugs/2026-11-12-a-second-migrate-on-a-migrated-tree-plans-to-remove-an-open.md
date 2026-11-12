---
kind: bug
id: 2026-11-12-a-second-migrate-on-a-migrated-tree-plans-to-remove-an-open
aliases:
  - BUG-a-second-migrate-on-a-migrated-tree-plans-to-remove-an-open-bug-s-kept-93ef973bf38a70bc70488198
title: A second migrate on a migrated tree plans to remove an open bug's kept fixture, and a later-day apply deletes it
status: verified
classification: product-defect
scope: blocking
impact: A routine second migrate deletes an open bug's reproduction fixture
criteria: []
requirements: [REQ-records-009]
cases: []
evidence:
  - OBS-tc-records-a-second-migrate-keeps-an-open-bugs-fixture-22e39cd74b59abbfc4bedf98
  - OBS-tc-records-a-second-migrate-keeps-an-open-bugs-fixture-d83be6277bef94b5137ee0dc
receipts:
  - id: 2026-11-12-tc-records-a-second-migrate-keeps-an-open-bugs-fixture
    aliases: OBS-tc-records-a-second-migrate-keeps-an-open-bugs-fixture-22e39cd74b59abbfc4bedf98
    claim: "Ran cargo test --locked --test migrate -- --exact a_second_migrate_keeps_an_open_bugs_fixture_and_has_nothing_to_remove for TC-records-a-second-migrate-keeps-an-open-bugs-fixture: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate -- --exact a_second_migrate_keeps_an_open_bugs_fixture_and_has_nothing_to_remove
    revision: 748bca6dce62e009fb8f0e021349f9add9e9a9cb
    environment: local-macos-arm64
    content: b1beeb0347fd8b54768ac13fc8e5cfc8729c9d23d48d745771ae44e7de7c919a
    fingerprint: sha256:98db177de3b461f30a4d4503dc9db7830631b863c6f82fcefa37220e1d3edd77
    case: TC-records-a-second-migrate-keeps-an-open-bugs-fixture
    case_digest: d0252276cdf160e850e119abb3d97f4d2c2831800cecf1333d8324a92bae2921
    observed: 2026-11-12T02:21:30Z
    duration_seconds: 4
  - id: 2026-11-12-tc-records-a-second-migrate-keeps-an-open-bugs-fixture-1191de
    aliases: OBS-tc-records-a-second-migrate-keeps-an-open-bugs-fixture-d83be6277bef94b5137ee0dc
    claim: "Ran cargo test --locked --test migrate -- --exact a_second_migrate_keeps_an_open_bugs_fixture_and_has_nothing_to_remove for TC-records-a-second-migrate-keeps-an-open-bugs-fixture: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_second_migrate_keeps_an_open_bugs_fixture_and_has_nothing_to_remove
    revision: 96fab11a6748c945c7c1a8d7ecd95b1b42870233
    environment: local-macos-arm64
    content: 25da085413e0fa9d821a95f14c21d8f1e2c760b43684eb36faf13dfc18efe65d
    fingerprint: sha256:21f1ca7f497d4f55ba8c722f5dc9d59140c611a42e0bf32cfa215b8482c61412
    case: TC-records-a-second-migrate-keeps-an-open-bugs-fixture
    case_digest: d0252276cdf160e850e119abb3d97f4d2c2831800cecf1333d8324a92bae2921
    observed: 2026-11-12T02:28:13Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 0ee26ac0, debug build, SpecSync 6.2.0
revision: e71b11b32167cad7ff8e298f9eff04a13069c3eb
found_by: t384, test stage, area upgrade (full pass)
related: []
created: 2026-11-12T02:19:35Z
updated: 2026-11-12T02:28:27Z
---
# A second migrate on a migrated tree plans to remove an open bug's kept fixture, and a later-day apply deletes it

**What happened:** On a migrated tree whose open bug names a .chough/artifacts fixture, chough migrate prints a non-empty plan removing that fixture; the same day the apply refuses only because the dated receipt exists; on a later day it applies and deletes the fixture while the bug stays open
Actual: The kept-fixture rule reads only 0.5.x JSON bugs, so the migrated Markdown bug's fixture reads as an orphan artifact

**Steps:** Migrate a 0.5.0 project with an open bug's fixture, commit, then chough migrate again on a later day

**Expected:** A tree with no 0.5.x record has nothing to do, and an open bug's fixture is kept (REQ-records-009)

**Why it matters:** A routine second migrate deletes an open bug's reproduction fixture

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T02:19:35Z · Reported (t384, test stage, area upgrade (full pass))
- 2026-11-12T02:21:41Z · status: open -> investigating. Red at 748bca6d (leif/0.6.0-test1-red: d854ce1a plus the regression tests): the second plan carried the fixture as a source; fix in 9fd9e701.; evidence: OBS-tc-records-a-second-migrate-keeps-an-open-bugs-fixture-22e39cd74b59abbfc4bedf98
- 2026-11-12T02:28:27Z · status: investigating -> verified. Green at 96fab11a: TC-records-a-second-migrate-keeps-an-open-bugs-fixture passed; full suite 627 passed, clippy and fmt clean at 35586853; t384's black-box retest on odd50 passed, the later-day apply included.; evidence: OBS-tc-records-a-second-migrate-keeps-an-open-bugs-fixture-d83be6277bef94b5137ee0dc
