---
kind: bug
id: 2026-11-12-bug-update-observation-by-an-entry-s-alias-stores-the-alias
aliases:
  - BUG-bug-update-observation-by-an-entry-s-alias-stores-the-alias-session-qua-967b8638577e657a1250058a
title: bug update --observation by an entry's alias stores the alias session-qualified instead of the entry's canonical ID
status: verified
classification: product-defect
scope: in-scope
impact: Fixing an alias conflict doctor reported breaks the bug's evidence link
criteria: []
requirements: [REQ-cli-012]
cases: []
evidence:
  - OBS-tc-cli-alias-link-stored-canonical-6f34e0b6fbd87f144506730e
  - OBS-tc-cli-alias-link-stored-canonical-478c1e6fd12838bdc5e0dbd9
receipts:
  - id: 2026-11-12-tc-cli-alias-link-stored-canonical
    aliases: OBS-tc-cli-alias-link-stored-canonical-6f34e0b6fbd87f144506730e
    claim: "Ran cargo test --locked --test bugs -- --exact a_link_by_an_entrys_alias_is_stored_as_its_canonical_session_qualified_id for TC-cli-alias-link-stored-canonical: failed (exit 101)."
    result: failed
    command: cargo test --locked --test bugs -- --exact a_link_by_an_entrys_alias_is_stored_as_its_canonical_session_qualified_id
    revision: 1071f972500e69d7d121755472ba07bedb4efab3
    environment: local-macos-arm64
    content: 8073e6ec1029b538aac9c2e9968ad952d8ba7f3cccb3e2f154c2fb59cb0ce0da
    fingerprint: sha256:a87cb23f89bb01aab3d49f9715a169e1bcd55b8f085aa625e6594c6b60353804
    case: TC-cli-alias-link-stored-canonical
    case_digest: 7242a5de347808124ffc4fbdfdc1f7a3beeae7189366dd452ae9d2440c86137f
    observed: 2026-11-12T02:02:20Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-alias-link-stored-canonical-461844
    aliases: OBS-tc-cli-alias-link-stored-canonical-478c1e6fd12838bdc5e0dbd9
    claim: "Ran cargo test --locked --test bugs -- --exact a_link_by_an_entrys_alias_is_stored_as_its_canonical_session_qualified_id for TC-cli-alias-link-stored-canonical: passed (exit 0)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_link_by_an_entrys_alias_is_stored_as_its_canonical_session_qualified_id
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:b6b7e5ae95c2d913445660bb9435c86928be6fbb81c62bae332a56aea9a28b1e
    case: TC-cli-alias-link-stored-canonical
    case_digest: 7242a5de347808124ffc4fbdfdc1f7a3beeae7189366dd452ae9d2440c86137f
    observed: 2026-11-12T02:17:38Z
    duration_seconds: 0
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 81d2bf8c3bc2a1705e67113cb95c14ce0a3b4c59
found_by: t38b, test stage, area records
related: []
created: 2026-11-12T01:49:25Z
updated: 2026-11-12T02:18:08Z
---
# bug update --observation by an entry's alias stores the alias session-qualified instead of the entry's canonical ID

**What happened:** evidence holds <session>/<alias>, while --related stores the canonical bug ID
Actual: The alias is stored

**Steps:** chough bug update --observation <alias of an open session's entry>

**Expected:** The link stores <session>/<id>, so it survives an alias being dropped

**Why it matters:** Fixing an alias conflict doctor reported breaks the bug's evidence link

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:49:25Z · Reported (t38b, test stage, area records)
- 2026-11-12T02:02:52Z · status: open -> investigating. Red at 1071f972 (TC-cli-alias-link-stored-canonical, on t37x's branch before its fix): failed for this bug's reason; fix in 2944a6f2.; evidence: OBS-tc-cli-alias-link-stored-canonical-6f34e0b6fbd87f144506730e
- 2026-11-12T02:18:08Z · status: investigating -> verified. Green at 0ee26ac0: TC-cli-alias-link-stored-canonical passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-cli-alias-link-stored-canonical-478c1e6fd12838bdc5e0dbd9
