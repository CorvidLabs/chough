---
kind: bug
id: 2026-11-10-cases-record-and-evidence-add-do-not-return-the-observation-id
aliases:
  - BUG-cases-record-and-evidence-add-do-not-return-the-observation-id-they-rec-78216c78f69992204ef6d8da
title: cases record and evidence add do not return the observation ID they recorded
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: []
requirements: [REQ-cli-011, REQ-cli-017]
cases: [TC-cli-manual-record-returns-observation-id, TC-cli-evidence-add-returns-observation-id]
evidence:
  - OBS-tc-cli-manual-record-returns-observation-id-fa94fc389aac2ff426d36b44
  - OBS-tc-cli-evidence-add-returns-observation-id-3b80f0c5684eab9228a87a75
  - OBS-tc-cli-manual-record-returns-observation-id-d15d102dd6ec256479b40abd
  - OBS-tc-cli-evidence-add-returns-observation-id-7703216ed3aa2f20b3b2661d
receipts:
  - id: 2026-11-10-tc-cli-manual-record-returns-observation-id
    aliases: OBS-tc-cli-manual-record-returns-observation-id-fa94fc389aac2ff426d36b44
    claim: "Ran cargo test --locked --test cases -- --exact manual_case_record_returns_its_case_and_observation_ids for TC-cli-manual-record-returns-observation-id: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test cases -- --exact manual_case_record_returns_its_case_and_observation_ids
    revision: 906717d6ef6b717ff73d2de6c0e780176b651c01
    environment: local-macos-arm64
    content: 3143001ec8c8dcf30bad62e1e4fcc7749a536287b08a98ce1ebdda4ec1683994
    fingerprint: sha256:beb14a2b4ad787adb84637a3b3189bb6dec49d466b5934875a71f67a6d3c9c21
    case: TC-cli-manual-record-returns-observation-id
    case_digest: a5492899c46e689ef38b943f2ac7034362a1e33ad66830987f4aae5b65f3f4e2
    observed: 2026-11-10T21:19:02Z
    duration_seconds: 2
  - id: 2026-11-10-tc-cli-evidence-add-returns-observation-id
    aliases: OBS-tc-cli-evidence-add-returns-observation-id-3b80f0c5684eab9228a87a75
    claim: "Ran cargo test --locked --test cases -- --exact evidence_add_returns_its_observation_id for TC-cli-evidence-add-returns-observation-id: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test cases -- --exact evidence_add_returns_its_observation_id
    revision: 906717d6ef6b717ff73d2de6c0e780176b651c01
    environment: local-macos-arm64
    content: 3143001ec8c8dcf30bad62e1e4fcc7749a536287b08a98ce1ebdda4ec1683994
    fingerprint: sha256:905420fdda1c9e10e7a20ac343b0c2cbb791a337d31d96bd5553ef68bba7ac02
    case: TC-cli-evidence-add-returns-observation-id
    case_digest: 115dd5270fa4434aaad910853d68b79516859cd870feea971e0a55424526e7b5
    observed: 2026-11-10T21:19:07Z
    duration_seconds: 1
  - id: 2026-11-10-tc-cli-manual-record-returns-observation-id-c201a0
    aliases: OBS-tc-cli-manual-record-returns-observation-id-d15d102dd6ec256479b40abd
    claim: "Ran cargo test --locked --test cases -- --exact manual_case_record_returns_its_case_and_observation_ids for TC-cli-manual-record-returns-observation-id: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact manual_case_record_returns_its_case_and_observation_ids
    revision: 906717d6ef6b717ff73d2de6c0e780176b651c01
    environment: local-macos-arm64
    content: abf864cfed51aa90bccc2979007802a08e3f783c9971edcad1cc2721ea629445
    fingerprint: sha256:fccacd365c5b8858750a4623d87fbd249a4442dd01efb8400ddcfe9dfc155f60
    case: TC-cli-manual-record-returns-observation-id
    case_digest: a5492899c46e689ef38b943f2ac7034362a1e33ad66830987f4aae5b65f3f4e2
    observed: 2026-11-10T21:19:50Z
    duration_seconds: 1
  - id: 2026-11-10-tc-cli-evidence-add-returns-observation-id-b49744
    aliases: OBS-tc-cli-evidence-add-returns-observation-id-7703216ed3aa2f20b3b2661d
    claim: "Ran cargo test --locked --test cases -- --exact evidence_add_returns_its_observation_id for TC-cli-evidence-add-returns-observation-id: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact evidence_add_returns_its_observation_id
    revision: 906717d6ef6b717ff73d2de6c0e780176b651c01
    environment: local-macos-arm64
    content: abf864cfed51aa90bccc2979007802a08e3f783c9971edcad1cc2721ea629445
    fingerprint: sha256:a974e612abc23bfc76b45d922472912de33ddac59bccb05251f5cf5068eb88e4
    case: TC-cli-evidence-add-returns-observation-id
    case_digest: 115dd5270fa4434aaad910853d68b79516859cd870feea971e0a55424526e7b5
    observed: 2026-11-10T21:19:53Z
    duration_seconds: 1
environment: local-macos-arm64
revision: 906717d6ef6b717ff73d2de6c0e780176b651c01
found_by: manual report
related: []
created: 2026-11-10T21:19:16Z
updated: 2026-11-10T21:20:05Z
---
# cases record and evidence add do not return the observation ID they recorded

**What happened:** A successful cases record returns only schema_version, recorded, outcome and provenance, and evidence add only schema_version, recorded and provenance; neither returns the observation ID, unlike cases run
Actual: observation and case keys are absent; automation must parse the ID out of the recorded path (KeyError: 'observation' in Kimi's runner)

**Steps:** chough cases record <case> ... or chough evidence add ... with exit 0
1. chough cases record TC-x --environment e --method manual --verdict 1=pass --summary s
2. chough evidence add --kind product-test ... --outcome passed
3. Neither response has an observation key

**Expected:** cases record also returns case and observation, evidence add also returns observation, each equal to the recorded file stem and stored id (issue #60, REQ-cli-011, REQ-cli-017)

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-10T21:19:16Z · Reported (manual report); evidence: OBS-tc-cli-manual-record-returns-observation-id-fa94fc389aac2ff426d36b44, OBS-tc-cli-evidence-add-returns-observation-id-3b80f0c5684eab9228a87a75
- 2026-11-10T21:20:05Z · status: open -> verified. Both cases pass after the fix: the responses carry the IDs, equal to the recorded file stem and stored id.; evidence: OBS-tc-cli-manual-record-returns-observation-id-d15d102dd6ec256479b40abd, OBS-tc-cli-evidence-add-returns-observation-id-7703216ed3aa2f20b3b2661d
