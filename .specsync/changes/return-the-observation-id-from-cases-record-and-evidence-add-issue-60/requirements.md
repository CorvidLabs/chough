---
change: return-the-observation-id-from-cases-record-and-evidence-add-issue-60
artifact: requirements
---

# Requirements

One `cli` delta modifies existing requirements; no new HI criterion or
requirement ID is introduced.

## MODIFIED REQ-cli-011

Add the successful `cases record` response contract: retain every existing key
and add `case` containing the requested case ID and `observation` containing the
stored observation ID. The observation ID equals the `recorded` evidence file's
stem and the stored record's `id`. Existing execution, recording, exit-code,
identity and legacy behavior stays unchanged.

## MODIFIED REQ-cli-017

Add the successful `evidence add` response contract: retain every existing key
and add `observation` containing the stored observation ID, equal to the
`recorded` evidence file's stem and the stored record's `id`. Observation schema
2, schema 1 reads and freshness semantics stay unchanged.

REQ-cli-023 remains unchanged: this fix allocates no IDs and changes no identity
format. Bug-output envelopes, structured stage refusals and `next` listing
unreferenced observations remain out of scope.
