---
id: raise-the-checkpoint-session-byte-limit-to-131072-and-the-observation-pointer-limit-to-400
state: archived
type: feature
base_commit: 747a12cc6fc396f0e8646fee1f2801e02b7d3035
---

# Raise the checkpoint session byte limit to 131072 and the observation pointer limit to 400

## Intent

Raise the checkpoint session byte limit to 131072 and the observation pointer limit to 400

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The checkpoint (.chough/session.json) bound is 131072 bytes for reading, validating, writing, init and checkpoint --merge inputs and output; a checkpoint at exactly 131072 bytes is accepted and one over it is refused with a message naming 131072, leaving records byte-identical
- The checkpoint observations list accepts 400 pointers and refuses 401; every other checkpoint list keeps its 200-item bound and every other record keeps its 65536-byte bound
- Existing refusal tests keep their assertions with only the limit value updated; fledge lanes run verify passes; hosted CI is reported only as observed

## No-spec Rationale

Not applicable
