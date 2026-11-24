---
id: fix-436
state: archived
type: feature
base_commit: 71a5c468a92d609113efbd5382317a18f1948e6e
---

# fix-436

## Intent

Gate decisions show the configured display name instead of the raw --decided-by text

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- A gate close with an alias renders the configured display name in the decision text; an exact-name close is unchanged; the stored relay keeps the raw typed decided-by; tests pin both; full suite green

## No-spec Rationale

REQ-records-008 already defines name as 'the display name records show'; this makes decision_text honor it. No contract or refusal semantics change
