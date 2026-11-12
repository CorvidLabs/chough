---
kind: bug
id: 2026-11-11-next-shows-no-last-failure-when-the-failure-is-held-only-as-a
aliases:
  - BUG-next-shows-no-last-failure-when-the-failure-is-held-only-as-a-receipt-423d2c20ddf04a9d5fe8452e
title: next shows no last failure when the failure is held only as a receipt
status: verified
classification: product-defect
scope: in-scope
impact: A fresh agent resuming after a migration or a finished session is not told the last failure
criteria: [LIFE-1]
requirements: [REQ-cli-021]
cases: [TC-cli-next-last-failure-from-receipts]
evidence:
  - OBS-tc-cli-next-last-failure-from-receipts-7b1c7fbab6a386220a89d52a
  - OBS-tc-cli-next-last-failure-from-receipts-ee676fb9ef0042aa355a9fd1
receipts:
  - id: 2026-11-11-tc-cli-next-last-failure-from-receipts
    aliases: OBS-tc-cli-next-last-failure-from-receipts-7b1c7fbab6a386220a89d52a
    claim: "Ran cargo test --locked --test next -- --exact next_names_the_latest_failure_held_only_as_a_receipt for TC-cli-next-last-failure-from-receipts: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test next -- --exact next_names_the_latest_failure_held_only_as_a_receipt
    revision: e7f5b4ccd1457c30925b0bf8ab151d3075b1df50
    environment: local-macos-arm64
    content: 7dd942d3909d12595dfcf13ef55257239ada8ae5602b926dacd0a4af90be9bbc
    fingerprint: sha256:855587a93ae2af7bdbd2250e1dd048c9dd03ef0d8868415d69d8ee6d19bacb89
    case: TC-cli-next-last-failure-from-receipts
    case_digest: 24a34fe926ef8547bf47bc07c4ee6401ee3fdf365ae3e115fe1542918551c4dc
    observed: 2026-11-11T23:27:11Z
    duration_seconds: 0
  - id: 2026-11-11-tc-cli-next-last-failure-from-receipts-7c3d6a
    aliases: OBS-tc-cli-next-last-failure-from-receipts-ee676fb9ef0042aa355a9fd1
    claim: "Ran cargo test --locked --test next -- --exact next_names_the_latest_failure_held_only_as_a_receipt for TC-cli-next-last-failure-from-receipts: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_the_latest_failure_held_only_as_a_receipt
    revision: bb8b177c14701feb7473fbd87a59ede9c96772e0
    environment: local-macos-arm64
    content: 6ee12999dacef43c2351ef24770adec0a47fef6e903110836f3237f4ca4b853b
    fingerprint: sha256:74030a8fbb27f9964bf04236be8324bc6629c1cc0d7b1c7ba4baf94e7a684053
    case: TC-cli-next-last-failure-from-receipts
    case_digest: 24a34fe926ef8547bf47bc07c4ee6401ee3fdf365ae3e115fe1542918551c4dc
    observed: 2026-11-11T23:28:21Z
    duration_seconds: 8
environment: local-macos-arm64
revision: 75b92a42788b86dc6669a49a11c31770c459b991
found_by: t37f's seed.sh capture on the migrated 0.1.0 handoff seed
related: []
created: 2026-11-11T23:26:47Z
updated: 2026-11-11T23:28:21Z
---
# next shows no last failure when the failure is held only as a receipt

**What happened:** On the migrated handoff seed, next says 'Last failure: none' although BUG-0001 holds the failed run as a receipt
Actual: last_failure reads only open sessions' entries, so it is null

**Steps:** Migrate a project whose failing observation a bug links, or finish the session that held it, then run chough next
1. Give a bug a failed receipt and no open entries
2. Run chough next --json

**Expected:** next names the latest failed observation by its recorded time, from open entries and from story and bug receipts, with the bugs that link it (REQ-cli-021)

**Why it matters:** A fresh agent resuming after a migration or a finished session is not told the last failure

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T23:26:47Z · Reported (t37f's seed.sh capture on the migrated 0.1.0 handoff seed)
- 2026-11-11T23:27:13Z · Red on the current next: TC-cli-next-last-failure-from-receipts fails, last_failure is null; revision e7f5b4cc on leif/0.6.0-last-failure-red.; evidence: OBS-tc-cli-next-last-failure-from-receipts-7b1c7fbab6a386220a89d52a
- 2026-11-11T23:28:21Z · status: open -> verified. Green: TC-cli-next-last-failure-from-receipts passes; next names a failure held only as a bug receipt.; evidence: OBS-tc-cli-next-last-failure-from-receipts-ee676fb9ef0042aa355a9fd1
