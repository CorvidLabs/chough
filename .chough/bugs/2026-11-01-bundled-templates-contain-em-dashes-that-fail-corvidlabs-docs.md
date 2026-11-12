---
kind: bug
id: 2026-11-01-bundled-templates-contain-em-dashes-that-fail-corvidlabs-docs
aliases: [BUG-0018]
title: Bundled templates contain em dashes that fail CorvidLabs docs gates
status: verified
classification: product-defect
scope: in-scope
impact: Host copies cannot be committed or linted cleanly
criteria: [LIFE-4, LIFE-9]
requirements: [REQ-cli-016]
cases: []
evidence: [tc-cli-0090-1793500284, bug-0018-repro-1, tc-cli-0090-1793500893, bug-0018-fixed-1]
receipts:
  - id: 2026-11-01-tc-cli-0090
    aliases: tc-cli-0090-1793500284
    claim: "Ran cargo test --locked --test package -- --exact the_shipped_bundle_holds_no_em_or_en_dash for TC-cli-0090: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test package -- --exact the_shipped_bundle_holds_no_em_or_en_dash
    revision: fce3d98d9b4d093c77513c09bb23f97706f11fbe
    environment: local-macos-arm64
    content: 2576e5f2f6b87c584c38e7fc14434299f7863a6f516aeadd71f8ad9ca7a194e7
    fingerprint: sha256:e7258c8de6c500529b775e7e5cb3ff3f44e0dae6531e46674039b5131909f228
    case: TC-cli-0090
    case_digest: 1f872622615fe10f0fa6922e02bbdc38c50f21b4ae4ba80857e5cd266daaf82d
    observed: 2026-11-01T02:31:24Z
  - id: 2026-11-01-tc-cli-0090-f0b704
    aliases: bug-0018-repro-1
    claim: At fce3d98 (unfixed), chough init --host claude --host openai copied templates/cases.toml:1 and templates/frd.md:23 with U+2014 into both host directories; the case TC-cli-0090 run tc-cli-0090-1793500284 also failed.
    result: failed
    revision: fce3d98d9b4d093c77513c09bb23f97706f11fbe
    environment: local-macos-arm64
    content: 2576e5f2f6b87c584c38e7fc14434299f7863a6f516aeadd71f8ad9ca7a194e7
    fingerprint: sha256:fa00bb34fb69b1318c515364515c0fc9f531b67e229c4d21941eecda036a9bac
    case: TC-cli-0090
    case_digest: 1f872622615fe10f0fa6922e02bbdc38c50f21b4ae4ba80857e5cd266daaf82d
    observed: 2026-11-01T02:32:06Z
  - id: 2026-11-01-tc-cli-0090-af933b
    aliases: tc-cli-0090-1793500893
    claim: "Ran cargo test --locked --test package -- --exact the_shipped_bundle_holds_no_em_or_en_dash for TC-cli-0090: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_shipped_bundle_holds_no_em_or_en_dash
    revision: 282c5bdfeb7bae5cc84f3ac8236652a4536080af
    environment: local-macos-arm64
    content: e9daf2396e35880630ab04c9ea0f4ccc9d1acd8a9b7b4b1f08954372cd329155
    fingerprint: sha256:96f600247b3a9ac4335d7b84d9cccabcf8e60547d4e1456aafbeb1b00ae8e657
    case: TC-cli-0090
    case_digest: 1f872622615fe10f0fa6922e02bbdc38c50f21b4ae4ba80857e5cd266daaf82d
    observed: 2026-11-01T02:41:33Z
  - id: 2026-11-01-tc-cli-0090-87c35c
    aliases: bug-0018-fixed-1
    claim: At 282c5bd (fixed), the same init reproduction found no U+2013 or U+2014 in either host directory; case run tc-cli-0090-1793500893 passed.
    result: passed
    revision: 282c5bdfeb7bae5cc84f3ac8236652a4536080af
    environment: local-macos-arm64
    content: e9daf2396e35880630ab04c9ea0f4ccc9d1acd8a9b7b4b1f08954372cd329155
    fingerprint: sha256:dc2af88327c5461d5608ec47a7de4b5235598ca15c25fb8fdae647fcc0b7544d
    case: TC-cli-0090
    case_digest: 1f872622615fe10f0fa6922e02bbdc38c50f21b4ae4ba80857e5cd266daaf82d
    observed: 2026-11-01T02:42:09Z
environment: local-macos-arm64
revision: 2756dc24a821bbb92021455c704ef7a177123ffa
found_by: CorvidLabs/chough#13 (Raven run); reproduced on main 2756dc2
related: []
created: 2026-11-01T00:57:54Z
updated: 2026-11-01T02:42:11Z
---
# Bundled templates contain em dashes that fail CorvidLabs docs gates

**What happened:** templates/frd.md:23 and templates/cases.toml:1 contain U+2014
Actual: Installed copies fail the docs lint

**Steps:** chough init --host ... in a CorvidLabs repository with a no-dash docs gate
1. grep U+2014 in skills/chough-workflow/templates

**Expected:** The bundle uses no em or en dashes, and existing installs update cleanly

**Why it matters:** Host copies cannot be committed or linted cleanly

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T00:57:54Z · Reported (CorvidLabs/chough#13 (Raven run); reproduced on main 2756dc2)
- 2026-11-01T02:32:13Z · Reproduced at fce3d98 before any fix: case TC-cli-0090 failed and the init reproduction found both dashes in the host copies.; evidence: tc-cli-0090-1793500284, bug-0018-repro-1
- 2026-11-01T02:42:11Z · status: open -> verified. Fixed by the bundle hygiene change: the unchanged case TC-cli-0090 and the init reproduction pass at 282c5bd, after the linked failures.; evidence: tc-cli-0090-1793500893, bug-0018-fixed-1
