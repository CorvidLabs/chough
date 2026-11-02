---
id: bound-transition-history-and-preserve-referenced-proof
state: verifying
type: feature
base_commit: 825a3805530456005e361dfa6d96d93eee52cd09
---

# Bound transition history and preserve referenced proof

## Intent

Bound transition history and preserve referenced proof

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Legacy logs and recorded proof remain byte-identical; explicit adoption writes independent immutable transition events with a cursor in the sole checkpoint; compatible branch events integrate without loss and divergent current-cycle cursors refuse; normal reads and per-cycle storage have tested byte and event limits; timing keeps lineages separate and reports missing or pruned history as partial or unknown; retention previews exact protected and eligible sets and applies only with explicit authorization and fresh protection checks; crash, collision, unsafe-path and interruption fixtures preserve active state; roadmap and backlog mappings distinguish delivered work from open criteria

## No-spec Rationale

Not applicable
