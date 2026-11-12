---
kind: bug
id: 2026-11-12-at-accept-next-s-review-guidance-stales-verification-in-a
aliases:
  - BUG-at-accept-next-s-review-guidance-stales-verification-in-a-commit-first-a16f254c6ca366e7299030fe
title: At accept, next's review guidance stales verification in a commit-first project, so the ship is refused
status: verified
classification: product-defect
scope: in-scope
impact: A human's decision to ship is refused in commit-first projects
criteria: []
requirements: [REQ-cli-021, REQ-records-014]
cases: []
evidence:
  - OBS-tc-cli-accept-ships-through-change-ship-e01b26958b75e704dc2c98e5
  - OBS-tc-cli-accept-ships-through-change-ship-7ce40b4ea91bc33056aa1d1e
receipts:
  - id: 2026-11-12-tc-cli-accept-ships-through-change-ship
    aliases: OBS-tc-cli-accept-ships-through-change-ship-e01b26958b75e704dc2c98e5
    claim: "Ran cargo test --locked --test next -- --exact at_accept_the_decision_ships_through_change_ship_and_commit_first_commits_the_gate for TC-cli-accept-ships-through-change-ship: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test next -- --exact at_accept_the_decision_ships_through_change_ship_and_commit_first_commits_the_gate
    revision: caf49681a0cf25f565331987adf71af6861c9004
    environment: local-macos-arm64
    content: 9374038c120addeb17f6d6dfb3d148571ed3accdcf4d047ed56ed952641ffb73
    fingerprint: sha256:c5a347dd7498802f50326f12612305886f5cfd7afbef99f78f65a7a93c3454db
    case: TC-cli-accept-ships-through-change-ship
    case_digest: afe04a706f4ce3acb6f37fc370827fc61cf339fe59b4a0f277c9437bea273487
    observed: 2026-11-12T07:14:51Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-accept-ships-through-change-ship-63b8ca
    aliases: OBS-tc-cli-accept-ships-through-change-ship-7ce40b4ea91bc33056aa1d1e
    claim: "Ran cargo test --locked --test next -- --exact at_accept_the_decision_ships_through_change_ship_and_commit_first_commits_the_gate for TC-cli-accept-ships-through-change-ship: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact at_accept_the_decision_ships_through_change_ship_and_commit_first_commits_the_gate
    revision: 4efba39fde7dbbedefd1e7228354035217afb77d
    environment: local-macos-arm64
    content: 7e3accdef3b31e574d2f3dc584a68f552fafa43da6ade4d94c5f0e18db765b67
    fingerprint: sha256:05fb8ed6eefd7d667720f1a9d498ca5b9d5d779a1d095f000f15f9dbeee02d84
    case: TC-cli-accept-ships-through-change-ship
    case_digest: afe04a706f4ce3acb6f37fc370827fc61cf339fe59b4a0f277c9437bea273487
    observed: 2026-11-12T07:15:08Z
    duration_seconds: 2
environment: local-macos-arm64, tester build at 596ced67, debug, SpecSync 6.3.0
revision: 4efba39fde7dbbedefd1e7228354035217afb77d
found_by: t37j, 6.3.0 re-run, real native lifecycle (N1)
related: []
created: 2026-11-12T07:14:46Z
updated: 2026-11-12T07:15:08Z
---
# At accept, next's review guidance stales verification in a commit-first project, so the ship is refused

**What happened:** Following rule 6 (open the review gate, close it with checkpoint --close-gate --verdict pass, then change ship) writes .chough/ between change check --commit and finalize; SpecSync then refuses finalize and chough change ship as stale verification
Actual: Rule 6 names checkpoint --close-gate with the verdict before the ship

**Steps:** A project whose sdd.json does not leave Chough's records out of verification, the default after specsync change adopt, at accept while verifying

**Expected:** next points at the ship that writes nothing under .chough/ until after the native ship (REQ-cli-059, REQ-records-014)

**Why it matters:** A human's decision to ship is refused in commit-first projects

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T07:14:46Z · Reported (t37j, 6.3.0 re-run, real native lifecycle (N1))
- 2026-11-12T07:15:08Z · status: open -> investigating. Red at caf49681 (before its fix, SpecSync 6.3.0 on PATH): rule 6 named checkpoint --close-gate before the ship; fix in 9c2ca469.; evidence: OBS-tc-cli-accept-ships-through-change-ship-e01b26958b75e704dc2c98e5
- 2026-11-12T07:15:08Z · status: investigating -> verified. Green at 4efba39f: TC-cli-accept-ships-through-change-ship passed; next, relay, stages, gates, humans, change and docs tests pass.; evidence: OBS-tc-cli-accept-ships-through-change-ship-7ce40b4ea91bc33056aa1d1e
