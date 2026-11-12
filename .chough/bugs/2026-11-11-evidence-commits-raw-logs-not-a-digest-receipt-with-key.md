---
kind: bug
id: 2026-11-11-evidence-commits-raw-logs-not-a-digest-receipt-with-key
aliases:
  - BUG-evidence-commits-raw-logs-not-a-digest-receipt-with-key-failing-lines-2-143a995780130675707fcb2c
title: Evidence commits raw logs, not a digest receipt with key failing lines (#215)
status: verified
classification: product-defect
scope: in-scope
impact: Committed raw logs carry local paths and noise, and a reviewer cannot see why a run failed without the log
criteria: [KNOW-35]
requirements: [REQ-records-005]
cases: [TC-records-proof-split-and-external]
evidence:
  - OBS-tc-records-proof-split-and-external-71d4096f0611298620fd0ce6
  - OBS-tc-records-proof-split-and-external-c809538dffc28708c420244f
receipts:
  - id: 2026-11-11-tc-records-proof-split-and-external
    aliases: OBS-tc-records-proof-split-and-external-71d4096f0611298620fd0ce6
    claim: "Ran cargo test --locked --test records -- --exact logs_stay_local_and_failures_keep_key_lines for TC-records-proof-split-and-external: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test records -- --exact logs_stay_local_and_failures_keep_key_lines
    revision: 69b561549360301bc3379d3834e06de2e739cd43
    environment: local-macos-arm64
    content: 389ccff3c31e9305d2653db2f423dedfe01b3575dfb1d6620ca035fc4cbbf055
    fingerprint: sha256:2fd45d8ea8dedbadd4a539b999775fe472a1cc0e0e4b164cfe8abc76362534e1
    case: TC-records-proof-split-and-external
    case_digest: d4f582c9de490b3f79c25b98d1269423ce2628516e7a907b23567ed3372bf8e2
    observed: 2026-11-11T23:43:34Z
    duration_seconds: 1
  - id: 2026-11-12-tc-records-proof-split-and-external
    aliases: OBS-tc-records-proof-split-and-external-c809538dffc28708c420244f
    claim: "Ran cargo test --locked --test records -- --exact logs_stay_local_and_failures_keep_key_lines for TC-records-proof-split-and-external: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact logs_stay_local_and_failures_keep_key_lines
    revision: 56165795ef2e22accc19ec00c1d19dccf4ac0516
    environment: local-macos-arm64
    content: f4706304eb513cc26319b7fb72787977781ccff31bfe4ba11e6a37108ffc82c5
    fingerprint: sha256:ea7a77a94b5b3a8e494fc27e8ccbb832b30b404667ad5a6b431d2559495b3014
    case: TC-records-proof-split-and-external
    case_digest: d4f582c9de490b3f79c25b98d1269423ce2628516e7a907b23567ed3372bf8e2
    observed: 2026-11-12T00:17:55Z
    duration_seconds: 2
environment: local-macos-arm64
revision: 1af0710fdf20fef56e90261cff3e7f5e5450c829
found_by: "GitHub issue #215, red on 0.5.0 by t38b at 69b56154 (leif/0.6.0-records-red)"
related: []
created: 2026-11-11T23:45:21Z
updated: 2026-11-12T00:18:04Z
---
# Evidence commits raw logs, not a digest receipt with key failing lines (#215)

**What happened:** A case run commits its raw log under .chough/artifacts/ beside the records, and its observation has no fingerprint and no key failing lines
Actual: The raw log is committed in .chough/artifacts/; no local log, fingerprint or key lines

**Steps:** Run the case, or evidence add --artifact <absolute path>, on Chough 0.5.0
1. Run tests/records.rs logs_stay_local_and_failures_keep_key_lines as TC-records-proof-split-and-external

**Expected:** The raw log stays in .chough/local/logs/, and the committed entry gives the command, the output's SHA-256 and at most 20 key failing lines (REQ-records-005)

**Why it matters:** Committed raw logs carry local paths and noise, and a reviewer cannot see why a run failed without the log

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T23:45:21Z · Reported (GitHub issue #215, red on 0.5.0 by t38b at 69b56154 (leif/0.6.0-records-red)); evidence: OBS-tc-records-proof-split-and-external-71d4096f0611298620fd0ce6
- 2026-11-12T00:18:04Z · status: open -> verified. Green on integration after t37j's 45cc9853 and ad496d21: TC-records-proof-split-and-external passes (local logs with the command and the output's SHA-256, key failing lines, and an outside artifact fingerprinted in place under ext-<hex>).; evidence: OBS-tc-records-proof-split-and-external-c809538dffc28708c420244f
