---
id: use-slug-identities-for-all-new-chough-records
state: draft
type: feature
base_commit: 66bef80464cc8f6b7ba5e7e8023dfd6352f9d9ec
---

# Use slug identities for all new Chough records

## Intent

Use slug identities for all new Chough records

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- New case definitions accept curated `TC-<module>-<title-slug>` IDs with lowercase ASCII normalization, module-aware length bounds and global duplicate refusal. Exact legacy `TC-<module>-NNNN` cases remain valid with unchanged definition digests and no renumbering.
- Automatic observations use `OBS-<slug>-<24-lowercase-hex>`, validation runs use `VAL-<slug>-<24-lowercase-hex>`, and librarian snapshots use `SNAP-<slug>-<24-lowercase-hex>`. Random 96-bit suffixes allow identical repeated runs to remain distinct history; timestamps remain in their existing fields and determine chronology.
- Automated artifact names remain exactly `.chough/artifacts/<observation-id>.log`. Observation, validation, snapshot and artifact creation is atomic and refuses same-ID different-content creation without overwrite, timestamp or numeric fallback, or digest retry. Random-source and write failures preserve existing records.
- Explicit observation IDs are optional for automatic allocation. New writes supplied an old-style ID fail with an omit-ID remedy, while every legacy case, observation, validation and snapshot ID remains readable, linkable and resumable with no bulk migration or schema rewrite.
- Mixed legacy and new IDs work through bug links, checkpoint references, status and next, timing, retention and pruning, native manifests, validation resume and librarian indexes. Older Chough versions require an upgrade for slug case IDs. Transitions, preference keys, consent keys, stop markers and pruning-ledger entry shapes do not gain IDs.
- Failure-first independent-root and repeated-run evidence is retained and linked to local bugs before implementation, followed by later passes for the new generators and compatibility. Slug case definitions themselves use the new form. Focused and full Rust tests, real command exercises, formatting, Clippy with warnings denied, fledge verify and pre-push pass with CI-matching tools. Bug known-answer vectors remain unchanged.
- The existing getrandom package is used directly without a runtime or service. No change is made to bug identity generation, HI criteria, requirement IDs, SpecSync IDs, release or deployment.

## No-spec Rationale

Not applicable
