---
change: return-the-observation-id-from-cases-record-and-evidence-add-issue-60
artifact: requirements
---

# Requirements

This is a no-spec-change bug fix. REQ-cli-011, REQ-cli-017 and REQ-cli-023 remain
byte-identical; no new requirement or HI criterion is introduced.

The concrete output contract for this change is:

- A successful `cases record` response keeps every existing key and adds `case`
  containing the requested case ID and `observation` containing the stored
  observation ID.
- A successful `evidence add` response keeps every existing key and adds
  `observation` containing the stored observation ID.
- In both responses, `observation` equals the `recorded` evidence file's stem
  and that record's `id`.
- Existing exit codes, failure handling, observation schemas, record contents,
  ID allocation and unrelated command envelopes stay unchanged.

These bullets narrow the implementation and verification of issue #60; they do
not replace or amend the canonical requirements.
