---
kind: bug
id: 2026-11-11-chough-migrate-leaves-session-v1-json-pruned-jsonl-and-the
aliases:
  - BUG-chough-migrate-leaves-session-v1-json-pruned-jsonl-and-the-chough-artif-0121b2c46990da05215de306
title: chough migrate leaves session.v1.json, pruned.jsonl and the .chough/artifacts files out of its accounting
status: verified
classification: product-defect
scope: in-scope
impact: A migrated tree would keep 0.5.x files the requirement removes, and a file a mapper misses under .chough/artifacts/ would never show as unaccounted
criteria: [DRIVE-27]
requirements: [REQ-records-009]
cases: [TC-records-migrate-accounts-every-file]
evidence:
  - OBS-tc-records-migrate-accounts-every-file-3feae26266cdf9c6b9d9ea90
  - OBS-tc-records-migrate-accounts-every-file-a83f107d1a247a8f6a34f157
receipts:
  - id: 2026-11-11-tc-records-migrate-accounts-every-file
    aliases: OBS-tc-records-migrate-accounts-every-file-3feae26266cdf9c6b9d9ea90
    claim: "Ran cargo test --locked --test migrate --test migrate_bugs -- every_legacy_file_and_artifact_is_accounted_for a_bug_of_each_id_form_carries_its_links_fixtures_and_redactions for TC-records-migrate-accounts-every-file: failed (exit 101)."
    result: failed
    command: cargo test --locked --test migrate --test migrate_bugs -- every_legacy_file_and_artifact_is_accounted_for a_bug_of_each_id_form_carries_its_links_fixtures_and_redactions
    revision: 398a73daae1f837fd5bb4ebba6bb6297cde9e772
    environment: local-macos-arm64
    content: d3ca133865d7d9825749832875a7ad39e1ca09626375d25c5525ac8589c9c7ab
    fingerprint: sha256:0b9c05fd92f4b2b5e7ac1bb9d0f7e76e2012663bfc650c61b642ae3ec5788ec5
    case: TC-records-migrate-accounts-every-file
    case_digest: 638713ef68023d45179b8cde9e9839c8298f0b8edfe7a4993c1aa4ca1e3b27e1
    observed: 2026-11-11T20:56:54Z
    duration_seconds: 1
  - id: 2026-11-11-tc-records-migrate-accounts-every-file-f1982a
    aliases: OBS-tc-records-migrate-accounts-every-file-a83f107d1a247a8f6a34f157
    claim: "Ran cargo test --locked --test migrate --test migrate_bugs -- every_legacy_file_and_artifact_is_accounted_for a_bug_of_each_id_form_carries_its_links_fixtures_and_redactions for TC-records-migrate-accounts-every-file: passed (exit 0)."
    result: passed
    command: cargo test --locked --test migrate --test migrate_bugs -- every_legacy_file_and_artifact_is_accounted_for a_bug_of_each_id_form_carries_its_links_fixtures_and_redactions
    revision: 3b7320020b167169429ac03b1cadd20127ad2dac
    environment: local-macos-arm64
    content: 09b15daa17f6e3d61256ce5c54ee52079960e77dd026cca9af70b674fb4ac4d6
    fingerprint: sha256:49877614c0f1b6a1cff4cb9d4dd13a800ef52ee41ec3281115a98c7d90808dca
    case: TC-records-migrate-accounts-every-file
    case_digest: 638713ef68023d45179b8cde9e9839c8298f0b8edfe7a4993c1aa4ca1e3b27e1
    observed: 2026-11-11T21:03:03Z
    duration_seconds: 2
environment: local-macos-arm64
revision: 87a49d823684498c3c9aec2bf10198bd853b054a
found_by: t37j's read-only migration preview rehearsal (plan Migration step 2) at 5c47bc7b, plan digest 79d47efd
related: []
created: 2026-11-11T20:55:56Z
updated: 2026-11-11T21:03:11Z
---
# chough migrate leaves session.v1.json, pruned.jsonl and the .chough/artifacts files out of its accounting

**What happened:** The migration preview of this repository lists neither .chough/session.v1.json nor any of the 643 tracked files under .chough/artifacts/ anywhere in the plan, and lists .chough/pruned.jsonl as unaccounted with no mapper removing it
Actual: session.v1.json and the artifacts are absent from sources, removed and unaccounted, so an apply would leave them in the migrated tree silently; pruned.jsonl is unaccounted

**Steps:** Build the 0.6.0 branch and run chough migrate (preview only) on a fresh clone of this repository
1. Clone the repository at the branch head into a scratch folder
2. Build it and run chough migrate
3. Compare the plan's sources, removed and unaccounted with every file Git tracks under .chough/

**Expected:** Every 0.5.x file REQ-records-009 removes is in the plan's sources and is carried, removed with why, kept as an open bug's fixture, or reported as unaccounted so the apply refuses

**Why it matters:** A migrated tree would keep 0.5.x files the requirement removes, and a file a mapper misses under .chough/artifacts/ would never show as unaccounted

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T20:55:56Z · Reported (t37j's read-only migration preview rehearsal (plan Migration step 2) at 5c47bc7b, plan digest 79d47efd)
- 2026-11-11T20:57:06Z · Red on the current framework: TC-records-migrate-accounts-every-file fails at the plan's sources assertion (session.v1.json absent), revision 398a73da on leif/0.6.0-migrate-accounting-red.; evidence: OBS-tc-records-migrate-accounts-every-file-3feae26266cdf9c6b9d9ea90
- 2026-11-11T21:03:11Z · status: open -> verified. Green on 3b732002: TC-records-migrate-accounts-every-file passes; the plan lists session.v1.json and pruned.jsonl as removed with why, a stray artifact as a source the apply refuses while unaccounted, and an open bug's fixture as kept.; evidence: OBS-tc-records-migrate-accounts-every-file-a83f107d1a247a8f6a34f157
