---
id: read-bounded-diagnostic-input-inventories-without-changing-freshness-authority
state: archived
type: bug_fix
base_commit: ada04832ac7e82bc2f52ba9011db0a7e03ad7382
---

# Read bounded diagnostic input inventories without changing freshness authority

## Intent

Read bounded diagnostic input inventories without changing freshness authority

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Read all nine exact retained schema2 observations with optional snapshot.input_inventory v1 while preserving their bytes, outcomes, provenance, historical aggregate freshness and incomplete inventory flags. Absent inventory preserves legacy behavior. Strict typed v1 fields, <=128 entries and <=32768 compact serialized bytes; invalid paths/digests/domain-key relations/duplicates/types/unknown fields and unsupported versions refuse explicitly. No inventory-based promotion or path attribution; no aggregate hash/freshness/writer/schema/policy change. Actual old candidate refusal followed by meaningful process RED and SAME GREEN is required under separate lease. Independent review/human acceptance remain pending.

## No-spec Rationale

Not applicable
