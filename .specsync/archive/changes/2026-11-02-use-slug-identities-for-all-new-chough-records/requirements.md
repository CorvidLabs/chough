---
change: use-slug-identities-for-all-new-chough-records
artifact: requirements
---

# Requirements

One requirement is added and four are modified; see `deltas/cli.md` for the complete normative text.

- REQ-cli-023 defines the common identity behavior for automatic case observations, repeated validation runs and librarian snapshots. Their forms are `OBS-<slug>-<24-lowercase-hex>`, `VAL-<slug>-<24-lowercase-hex>` and `SNAP-<slug>-<24-lowercase-hex>`. Each suffix is 96 fresh random bits from direct use of the existing getrandom 0.4.3 package, encoded as 24 lowercase hexadecimal digits. No runtime or service is introduced.
- Identical repeated runs remain distinct history. Timestamps stay in their existing fields and determine chronology. Atomic create-new refuses same-ID different-content creation without overwrite, timestamp, numeric or digest fallback, another random draw or retry. Random-source and write failures preserve existing records.
- REQ-cli-009 adds curated `TC-<module>-<title-slug>` case IDs with lowercase ASCII slug syntax, module-aware use of the existing 100-byte bound and global duplicate refusal. Exact legacy `TC-<module>-NNNN` cases remain valid with unchanged definition digests and no renumbering or migration. New regression cases for this change use slug IDs themselves.
- REQ-cli-011 applies the common allocation to observations and validation runs. Explicit observation IDs remain optional, old-style IDs supplied for new writes receive an omit-ID remedy, and automated artifacts remain exactly `.chough/artifacts/<observation-id>.log`, inheriting the observation ID.
- REQ-cli-015 applies the common allocation to snapshots while retaining source links, digests, indexes and legacy dated snapshot reads.
- Legacy case, observation, validation and snapshot IDs remain readable, linkable and resumable where supported, with no bulk migration or schema rewrite. Mixed legacy and new IDs work through bug links, checkpoint references, status and next, timing, retention and pruning, native manifests, validation resume and librarian indexes. Documentation states that older Chough versions require an upgrade for new-form IDs.
- REQ-cli-012 retains the complete bug generator, schema and known-answer behavior. Only its obsolete statement that all other ID schemes remain unchanged, and the corresponding acceptance wording, are removed. Transition, preference, consent, stop-marker and pruning-ledger entry shapes gain no IDs.

Failure-first independent-root and repeated-run evidence must use slug case IDs and link local bugs before implementation, followed by later passes for the generators and compatibility. Bug known-answer vectors remain unchanged.

Every unrelated requirement and all noncontradictory existing acceptance criteria are preserved in the full replacements. No HI criterion is added or reworded. No canonical requirement is materialized by this artifact.
