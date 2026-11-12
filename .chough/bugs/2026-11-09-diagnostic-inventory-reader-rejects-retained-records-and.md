---
kind: bug
id: 2026-11-09-diagnostic-inventory-reader-rejects-retained-records-and
aliases:
  - BUG-diagnostic-inventory-reader-rejects-retained-records-and-admits-noncano-f62f58d2fe4bbdd22d83e8b4
title: Diagnostic inventory reader rejects retained records and admits noncanonical aliases
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: [LIFE-2, LIFE-4]
requirements: [REQ-cli-004, REQ-cli-006]
cases: [TC-cli-inventory-historical, TC-cli-inventory-aliases, TC-cli-inventory-bookkeeping-alias]
evidence:
  - OBS-tc-cli-inventory-historical-58c5786905be80e97b73ffe0
  - OBS-tc-cli-inventory-aliases-af56a469e54a2db6ce7b3a29
  - OBS-tc-cli-inventory-historical-6aade693da0b1ee6cfee1b3a
receipts:
  - id: 2026-11-09-tc-cli-inventory-historical
    aliases: OBS-tc-cli-inventory-historical-58c5786905be80e97b73ffe0
    claim: "Actual retained HCt2sm 2026-11-09T06:15:40.553322..06:16:10.933744Z, sourcebe17ade8/test17add: exact Fledge test --locked --test evidence_inventory_compatibility compiled6,1PASS5unknown-input_inventory FAIL after all valid legacy/corpus controls. No retrospective pass. Raw original compiled-red packet preserved; this observation is capture-only."
    result: failed
    revision: 4b35b15064a0d5d7172163a5501ffc95c3bc018d
    environment: local-macos-arm64
    content: 3cdf63286b570568c89a615b60e171598e6d30bce8c184c3d4ebd71a328d3a30
    case: TC-cli-inventory-historical
    case_digest: 558662abf69282b368c9e36a626b767bcb23cbe2d65dff8924a0aee3387d2de0
    observed: 2026-11-09T07:44:36Z
  - id: 2026-11-09-tc-cli-inventory-aliases
    aliases: OBS-tc-cli-inventory-aliases-af56a469e54a2db6ce7b3a29
    claim: "Actual retained HCt2vg 2026-11-09T07:27:22.697184..07:27:35.699021Z, sourcea7b2543/reader97cdf/test5dab: Fledge filter inventory_noncanonical_ ran2,0PASS2intended invalid-alias-accepted FAIL,6filtered after valid canonical controls. src//input.txt and .specsync//changes/test-change/state.json accepted; later variants NOTREACHED then. Diagnostic validation only, no authority exploit."
    result: failed
    revision: 4b35b15064a0d5d7172163a5501ffc95c3bc018d
    environment: local-macos-arm64
    content: 3cdf63286b570568c89a615b60e171598e6d30bce8c184c3d4ebd71a328d3a30
    case: TC-cli-inventory-aliases
    case_digest: d50014639b54b107b2bc9715648e3e78dea98e9fab4a428c62e3706ff45463cb
    observed: 2026-11-09T07:44:39Z
  - id: 2026-11-09-tc-cli-inventory-historical-6141d4
    aliases: OBS-tc-cli-inventory-historical-6aade693da0b1ee6cfee1b3a
    claim: Actual retained HCt2vj atd0125eacfafcb16b6dd9eefb065667246471d625 (2026-11-09T07:32Z):8/8 compatibility (original6 with ONE documented uppercase oracle correction plus SAME2aliases),14/14 evidence module,30/30 existing evidence integration=52PASS exact names. All9 original historical/partial/outcomes/bytes and complete boundary/alias loops executed. Source-equivalent formatted tests at4b35 proved exact rustfmt projection; NOT fresh52execution there. Initial fmt1/lintNOTRUN retained separately.
    result: passed
    revision: 4b35b15064a0d5d7172163a5501ffc95c3bc018d
    environment: local-macos-arm64
    content: 3cdf63286b570568c89a615b60e171598e6d30bce8c184c3d4ebd71a328d3a30
    case: TC-cli-inventory-historical
    case_digest: 558662abf69282b368c9e36a626b767bcb23cbe2d65dff8924a0aee3387d2de0
    observed: 2026-11-09T07:44:41Z
environment: local-macos-arm64
revision: 4b35b15064a0d5d7172163a5501ffc95c3bc018d
found_by: Retained actual HCt2sm/t2vg/t2vj/t2vn raw logs; not a fresh run
related: []
created: 2026-11-09T07:44:46Z
updated: 2026-11-09T07:44:46Z
---
# Diagnostic inventory reader rejects retained records and admits noncanonical aliases

**What happened:** Nine retained input_inventory observations blocked public reader; first fix accepted raw path aliases.
Actual: Original decoder runtime1PASS5FAIL, separate alias runtime0PASS2FAIL; corrected8+14+30PASS and finalfmtlint0 retained.

**Steps:** Read original9 records, then candidate inventory with src//input.txt or .specsync//changes/test-change/state.json.

**Expected:** Read strict bounded v1 diagnostics without changing freshness; reject noncanonical metadata spellings.

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-09T07:44:46Z · Reported (Retained actual HCt2sm/t2vg/t2vj/t2vn raw logs; not a fresh run); evidence: OBS-tc-cli-inventory-historical-58c5786905be80e97b73ffe0, OBS-tc-cli-inventory-aliases-af56a469e54a2db6ce7b3a29
- 2026-11-09T07:44:46Z · Added despite similar reports: BUG0010/0011 concern subprocess deadlines/output; this is strict observation diagnostic-schema/canonical-path validation.
- 2026-11-09T07:44:46Z · status: open -> verified. Retained actual before/after runtime assertions (be17 decoderRED; a7 aliasRED; d0125 full8+14+30GREEN); format-equivalent4b35 finalfmt/lint. Records are agent-assisted capture now, not fresh execution or human acceptance. One corrected uppercase oracle, original receipts retained.; evidence: OBS-tc-cli-inventory-historical-6aade693da0b1ee6cfee1b3a
