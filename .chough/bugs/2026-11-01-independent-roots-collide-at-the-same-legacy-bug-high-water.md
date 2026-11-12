---
kind: bug
id: 2026-11-01-independent-roots-collide-at-the-same-legacy-bug-high-water
aliases: [BUG-0026]
title: Independent roots collide at the same legacy bug high-water mark
status: verified
classification: product-defect
scope: in-scope
impact: Independent branches can allocate the same bug ID for different records, causing merge conflicts and ambiguous durable references
criteria: [FLOW-8, LIFE-5]
requirements: [REQ-cli-012]
cases: [TC-cli-0125]
evidence:
  - tc-cli-0125-collision-pre-fix
  - tc-cli-0125-collision-post-fix
  - tc-cli-0125-collision-final
receipts:
  - id: 2026-11-01-tc-cli-0125
    aliases: tc-cli-0125-collision-pre-fix
    claim: "Ran cargo test --locked --test bugs -- --exact independent_roots_at_the_same_high_water_mark_get_distinct_ids_without_breaking_legacy_links for TC-cli-0125: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test bugs -- --exact independent_roots_at_the_same_high_water_mark_get_distinct_ids_without_breaking_legacy_links
    revision: 041531cdeecc3447523ddf459f46258070e29247
    environment: local-macos-arm64
    content: 617c5be40538bf7444d95cdff219387e5557e8676bd0cd910223be72d6b0752f
    fingerprint: sha256:89f6de1a01637efc42def296dac4cd7d370cd42c8cf622e055bc49965aec7cda
    case: TC-cli-0125
    case_digest: 8f19cd1848d96598121e3c64b3a420404956a7fd42ee5ac7117aedc1462c083a
    observed: 2026-11-01T21:40:21Z
    duration_seconds: 1
  - id: 2026-11-01-tc-cli-0125-fa3abf
    aliases: tc-cli-0125-collision-post-fix
    claim: "Ran cargo test --locked --test bugs -- --exact independent_roots_at_the_same_high_water_mark_get_distinct_ids_without_breaking_legacy_links for TC-cli-0125: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact independent_roots_at_the_same_high_water_mark_get_distinct_ids_without_breaking_legacy_links
    revision: 041531cdeecc3447523ddf459f46258070e29247
    environment: local-macos-arm64
    content: f725c85024ac86bf524fb1cec3349217b84789f48731800811e3ed35cbc26348
    fingerprint: sha256:c85091a6497593d7d57b1ba2398c56f1ca37fe339a88411496e7ced132b9067d
    case: TC-cli-0125
    case_digest: 8f19cd1848d96598121e3c64b3a420404956a7fd42ee5ac7117aedc1462c083a
    observed: 2026-11-01T22:23:09Z
    duration_seconds: 2
  - id: 2026-11-01-tc-cli-0125-783e2e
    aliases: tc-cli-0125-collision-final
    claim: "Ran cargo test --locked --test bugs -- --exact independent_roots_at_the_same_high_water_mark_get_distinct_ids_without_breaking_legacy_links for TC-cli-0125: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact independent_roots_at_the_same_high_water_mark_get_distinct_ids_without_breaking_legacy_links
    revision: 041531cdeecc3447523ddf459f46258070e29247
    environment: local-macos-arm64
    content: f5424d685bf8a7b4d9d24d685991fe8d4e33a11e51c446b5e3103dc2a89181db
    fingerprint: sha256:a91c9ac2aaf3e317e1073b4fc49aa83429b848de6b459e3b1f4e4b161a48e325
    case: TC-cli-0125
    case_digest: 8f19cd1848d96598121e3c64b3a420404956a7fd42ee5ac7117aedc1462c083a
    observed: 2026-11-01T22:26:02Z
    duration_seconds: 2
environment: local-macos-arm64
revision: 041531cdeecc3447523ddf459f46258070e29247
found_by: TC-cli-0125 pre-fix reproduction
related: []
created: 2026-11-01T21:41:00Z
updated: 2026-11-01T22:26:03Z
---
# Independent roots collide at the same legacy bug high-water mark

**What happened:** Distinct manual and automatic reports receive identical IDs in independent roots
Actual: At reproduction revision 041531cdeecc3447523ddf459f46258070e29247, manual IDs were BUG-0020 and BUG-0020, and automatic IDs were BUG-0020 and BUG-0020, at equal legacy high-water marks

**Steps:** Seed independent roots at BUG-0019, then create distinct manual reports in one pair and distinct automatic validation-capture reports in another pair
1. cargo test --locked --test bugs -- --exact independent_roots_at_the_same_high_water_mark_get_distinct_ids_without_breaking_legacy_links
2. cargo run --locked -- cases run TC-cli-0125 --environment local-macos-arm64 --observation-id tc-cli-0125-collision-pre-fix

**Expected:** Each distinct report receives a collision-resistant identity without changing legacy BUG-0019

**Why it matters:** Independent branches can allocate the same bug ID for different records, causing merge conflicts and ambiguous durable references

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T21:41:00Z · Reported (TC-cli-0125 pre-fix reproduction); evidence: tc-cli-0125-collision-pre-fix
- 2026-11-01T22:23:10Z · status: open -> verified. Post-fix TC-cli-0125 passed after the retained independent-root collision; distinct manual and automatic reports received new-form IDs, legacy behavior and cross-form links passed; evidence: tc-cli-0125-collision-post-fix
- 2026-11-01T22:26:03Z · Final-content TC-cli-0125 passed with the same independent-root collision coverage after all source, documentation and native task edits; evidence: tc-cli-0125-collision-final
