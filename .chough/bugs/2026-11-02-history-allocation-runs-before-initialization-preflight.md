---
kind: bug
id: 2026-11-02-history-allocation-runs-before-initialization-preflight
aliases: [BUG-history-allocation-runs-before-initialization-preflight-cd2ea3e4eb06d54d1bb019ba]
title: History allocation runs before initialization preflight
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: [LIFE-4]
requirements: [REQ-cli-001]
cases: []
evidence: [OBS-product-test-c08495b2fd395eecff258213, OBS-product-test-fca5c5e603111f6dc31f5f4d]
receipts:
  - id: 2026-11-02-both-existing-before-write-initialization-regressions-failed
    aliases: OBS-product-test-c08495b2fd395eecff258213
    claim: "Both existing before-write initialization regressions failed after immutable history integration: a history directory was created before rejecting an oversized checkpoint and an unsafe installed-skill path. Rust source is unchanged since this targeted run."
    result: failed
    revision: 9a8b9b11e16bedac4808c56d48307fa29ad9cfe3
    environment: local-macos-arm64
    content: 356655434f611fd57fc7291dd802120b1fd6bb233e31a718a677fd9dd1851e96
    fingerprint: sha256:9a32de1925d03b7e300a1d8c9e14ea4404108b63b070871433571a353a053bf4
    observed: 2026-11-02T14:17:45Z
  - id: 2026-11-02-the-mandatory-fledge-verify-lane-passed-all-9-steps-on-the
    aliases: OBS-product-test-fca5c5e603111f6dc31f5f4d
    claim: "The mandatory Fledge verify lane passed all 9 steps on the final implementation: 219 Rust tests passed with no failures or exclusions, format/lint/HI/structural/native audit/case definitions passed, and the disposable real CLI feature-and-bug exercise passed. The log includes the initialization preflight and interrupted-byte-count regressions."
    result: passed
    revision: 9a8b9b11e16bedac4808c56d48307fa29ad9cfe3
    environment: local-macos-arm64
    content: 0706650c13e025a3e3af738b2b3a63ed9e32dcdf22d03c217ee27d913c34e031
    fingerprint: sha256:38778d7538086fb1b8d4fe3e3089014191c33964209ce1668a2e5cb6b33e51d5
    observed: 2026-11-02T15:02:09Z
fixture: tests/cli.rs
environment: local-macos-arm64
revision: 9a8b9b11e16bedac4808c56d48307fa29ad9cfe3
found_by: Current-source targeted reproduction before the preflight fix
related: []
created: 2026-11-02T14:18:32Z
updated: 2026-11-02T15:02:10Z
---
# History allocation runs before initialization preflight

**What happened:** Rejected initialization leaves a new .chough directory behind
Actual: History reservation creates .chough/history before existing validation rejects initialization

**Steps:** Run the existing CLI before-write tests after adding new-project history allocation

**Expected:** Oversized checkpoints and unsafe skill destinations are rejected before creating any managed file or directory

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T14:18:32Z · Reported (Current-source targeted reproduction before the preflight fix); evidence: OBS-product-test-c08495b2fd395eecff258213
- 2026-11-02T15:02:10Z · status: open -> verified. The later full lane passes the original before-write assertions and empty-checkpoint package contract; event directories are deferred until the first event.; evidence: OBS-product-test-fca5c5e603111f6dc31f5f4d
