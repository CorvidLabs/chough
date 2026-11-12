---
kind: bug
id: 2026-10-31-malformed-sdd-policy-is-treated-as-deliberately-disabled
aliases: [BUG-0008]
title: Malformed SDD policy is treated as deliberately disabled
status: verified
classification: product-defect
scope: in-scope
impact: A damaged policy file silently removes every native gate
criteria: [FLOW-5]
requirements: [REQ-cli-008, REQ-cli-003]
cases: []
evidence: [bug-0008-repro-1, bug-0008-fixed-1]
receipts:
  - id: 2026-10-31-truncated-sdd-json-a-string-enabled-value-and-a-missing
    aliases: bug-0008-repro-1
    claim: Truncated sdd.json, a string enabled value and a missing enabled key all produced gate ungated:sdd-disabled, the same as a valid enabled false
    result: failed
    revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
    environment: local-macos-arm64
    fingerprint: sha256:0da22e69a333fcb7614c97ce7dcb4d2f704bcb16dada9259b371a9649fcdafa8
    observed: 2026-10-31T20:44:39Z
  - id: 2026-10-31-truncated-and-ill-typed-policies-refuse-as-refused-sdd-policy
    aliases: bug-0008-fixed-1
    claim: Truncated and ill-typed policies refuse as refused:sdd-policy-invalid; a missing enabled gates; a valid enabled false stays ungated
    result: passed
    revision: 80757487bddf9172bd757f78fcfa798e1d7e40b3
    environment: local-macos-arm64
    fingerprint: sha256:e298a6cfaee6a89012d246be4b9ecf516e0a7470ed16bf712bd003bff322b076
    observed: 2026-10-31T21:24:02Z
environment: local-macos-arm64
revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
found_by: Review finding C, reproduced 2026-10-31 against main f19823d
related: []
created: 2026-10-31T20:45:00Z
updated: 2026-10-31T21:24:03Z
---
# Malformed SDD policy is treated as deliberately disabled

**What happened:** Truncated sdd.json, enabled as a string, or no enabled key all gave ungated:sdd-disabled
Actual: Parse failures and wrong types fall through to the ungated path

**Steps:** An unreadable or ill-typed .specsync/sdd.json
1. Write { "enabled": true, (truncated) to .specsync/sdd.json
2. chough stage advance from plan

**Expected:** The gate is refused as unknown policy; only a valid enabled false is ungated

**Why it matters:** A damaged policy file silently removes every native gate

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T20:45:00Z · Reported (Review finding C, reproduced 2026-10-31 against main f19823d); evidence: bug-0008-repro-1
- 2026-10-31T21:24:03Z · status: open -> verified. Fixed in 8075748 (fail-closed stage gates); regression TC-cli-0043 passed (observation tc-cli-0043-1793481649).; evidence: bug-0008-fixed-1
