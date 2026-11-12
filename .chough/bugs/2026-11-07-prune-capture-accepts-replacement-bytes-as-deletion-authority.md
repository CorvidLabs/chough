---
kind: bug
id: 2026-11-07-prune-capture-accepts-replacement-bytes-as-deletion-authority
aliases:
  - BUG-prune-capture-accepts-replacement-bytes-as-deletion-authority-cdfb288128633d57b0c29089
title: Prune capture accepts replacement bytes as deletion authority
status: verified
classification: product-defect
scope: blocking
impact: Not yet assessed.
criteria: [LIFE-8]
requirements: [REQ-history-004]
cases: [TC-history-prune-capture-byte-authority]
evidence:
  - OBS-tc-history-prune-capture-byte-authority-21c076beb9fc717b19bb1aac
  - OBS-tc-history-prune-capture-byte-authority-0d8f9c5f414d1458c254a22a
receipts:
  - id: 2026-11-07-tc-history-prune-capture-byte-authority
    aliases: OBS-tc-history-prune-capture-byte-authority-21c076beb9fc717b19bb1aac
    claim: "Ran cargo test --locked --bin chough evidence::tests::pr197_captured_replacement_bytes_survive -- --nocapture for TC-history-prune-capture-byte-authority: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough evidence::tests::pr197_captured_replacement_bytes_survive -- --nocapture
    revision: 8f7574956f2f8b8789022e68d6a02e9ef0567062
    environment: local-macos-arm64
    content: 5fff028745dd4b812f2c44ac362e5e5a280224bb9d24c534e611c5a9d923fcae
    fingerprint: sha256:6f0894aee6b6a94e155c0cc19e8bf9b2fb72d46c9a1373133438bf9f1bef4fd0
    case: TC-history-prune-capture-byte-authority
    case_digest: 7daf701f1a779f5139332e20511f3e5f5fc18b2bd9f52d23d64269ddd16fc8b3
    observed: 2026-11-07T17:05:34Z
    duration_seconds: 3
  - id: 2026-11-07-tc-history-prune-capture-byte-authority-617828
    aliases: OBS-tc-history-prune-capture-byte-authority-0d8f9c5f414d1458c254a22a
    claim: "Ran cargo test --locked --bin chough evidence::tests::pr197_captured_replacement_bytes_survive -- --nocapture for TC-history-prune-capture-byte-authority: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough evidence::tests::pr197_captured_replacement_bytes_survive -- --nocapture
    revision: 6748c3d30a005e46abe468fc00eb43900eda374f
    environment: local-macos-arm64
    content: e71b8b6ed7758d1321609e1f534c702307429f0664fc85405e469508db66a807
    fingerprint: sha256:01806fb141cafb6d2df44a90fcd48ef37de96637dd1a29a6c7ddc1def0848a24
    case: TC-history-prune-capture-byte-authority
    case_digest: 7daf701f1a779f5139332e20511f3e5f5fc18b2bd9f52d23d64269ddd16fc8b3
    observed: 2026-11-07T17:07:15Z
    duration_seconds: 2
environment: local-macos-arm64
revision: 8f7574956f2f8b8789022e68d6a02e9ef0567062
found_by: PR 197 discussion_r4502629883
related: []
created: 2026-11-07T17:05:50Z
updated: 2026-11-07T17:09:08Z
---
# Prune capture accepts replacement bytes as deletion authority

**What happened:** Replacement artifact is deleted after eligibility digest validation
Actual: Both scenarios removed writer bytes and observations; earlier set remained removed; cargo exit 101

**Steps:** Replace artifact immediately before byte capture, including after a previous set was removed

**Expected:** Keep replacement bytes and observation; restore earlier removals

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-07T17:05:50Z · Reported (PR 197 discussion_r4502629883); evidence: OBS-tc-history-prune-capture-byte-authority-21c076beb9fc717b19bb1aac
- 2026-11-07T17:09:08Z · status: open -> verified. Same committed red case now passes; mutation removing captured-byte identity comparison fails the test; full lanes and fresh human acceptance still pending.; evidence: OBS-tc-history-prune-capture-byte-authority-0d8f9c5f414d1458c254a22a
