---
kind: bug
id: 2026-11-11-records-are-json-for-machines-not-markdown-a-person-can-read
aliases:
  - BUG-records-are-json-for-machines-not-markdown-a-person-can-read-and-edit-2-87f499ca404f2f39290af001
title: Records are JSON for machines, not Markdown a person can read and edit (#235)
status: verified
classification: product-defect
scope: in-scope
impact: "People cannot use the records directly (GitHub #235)"
criteria: [DRIVE-25, KNOW-32]
requirements: [REQ-cli-007]
cases: [TC-records-files-are-markdown-and-tolerant]
evidence:
  - OBS-tc-records-files-are-markdown-and-tolerant-8df622c4c2bdbdbf0930c6d7
  - OBS-tc-records-files-are-markdown-and-tolerant-9f961221bc2961b934e3ddfe
receipts:
  - id: 2026-11-11-tc-records-files-are-markdown-and-tolerant
    aliases: OBS-tc-records-files-are-markdown-and-tolerant-8df622c4c2bdbdbf0930c6d7
    claim: "Ran cargo test --locked --test records -- --exact every_record_is_markdown_and_keeps_human_edits for TC-records-files-are-markdown-and-tolerant: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test records -- --exact every_record_is_markdown_and_keeps_human_edits
    revision: 370d221d68840f2e263e90b927a200438a7a0173
    environment: local-macos-arm64
    content: 843a083f60a67703f38514e1fcffa147b0e3c500ec307ca879614ca5ee672a4c
    fingerprint: sha256:f7f49d9706909344a5e8f7e5cc5cda54c9731f28888b6d877e54c7c129c394f9
    case: TC-records-files-are-markdown-and-tolerant
    case_digest: 53f3c34cfa1df4cacf5e93820e9c30f22bde0596fcc28d6c3c1ef0fb96cde237
    observed: 2026-11-11T20:36:05Z
    duration_seconds: 0
  - id: 2026-11-11-tc-records-files-are-markdown-and-tolerant-b0dbe0
    aliases: OBS-tc-records-files-are-markdown-and-tolerant-9f961221bc2961b934e3ddfe
    claim: "Ran cargo test --locked --test records -- --exact every_record_is_markdown_and_keeps_human_edits for TC-records-files-are-markdown-and-tolerant: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact every_record_is_markdown_and_keeps_human_edits
    revision: 7b4cd3eb827f70c821255c0efb0326f17d7ca23a
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    fingerprint: sha256:a0969c97565e0e8fd94c8c0f88ecb40d997981b778bab88004eb465fa7bca8b7
    case: TC-records-files-are-markdown-and-tolerant
    case_digest: 53f3c34cfa1df4cacf5e93820e9c30f22bde0596fcc28d6c3c1ef0fb96cde237
    observed: 2026-11-11T20:39:01Z
    duration_seconds: 4
environment: local-macos-arm64
revision: 67d001e55e59fa271166d92c95ea44fac6d4f304
found_by: "GitHub issue #235; red on 0.5.0 at leif/0.6.0-batch-red 370d221d"
related: []
created: 2026-11-11T20:36:57Z
updated: 2026-11-11T20:39:42Z
---
# Records are JSON for machines, not Markdown a person can read and edit (#235)

**What happened:** sessions, evidence and bugs are JSON files; a person cannot read or edit them safely and there is no single human-first source of truth
Actual: records written as JSON, such as .chough/sessions/alpha.json

**Steps:** chough session new alpha --objective <outcome>, then look under .chough/
1. chough session new alpha --objective <outcome>

**Expected:** one Markdown file per record with front matter for agents and a body for people, edits kept

**Why it matters:** People cannot use the records directly (GitHub #235)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T20:36:57Z · Reported (GitHub issue #235; red on 0.5.0 at leif/0.6.0-batch-red 370d221d); evidence: OBS-tc-records-files-are-markdown-and-tolerant-8df622c4c2bdbdbf0930c6d7
- 2026-11-11T20:39:42Z · status: open -> verified. Fixed by plan steps 2 to 5 (PR #238): sessions and bugs are Markdown records whose human edits survive; evidence entries (step 4) join the case when they land. The case passed after its 0.5.0 failure.; evidence: OBS-tc-records-files-are-markdown-and-tolerant-9f961221bc2961b934e3ddfe
