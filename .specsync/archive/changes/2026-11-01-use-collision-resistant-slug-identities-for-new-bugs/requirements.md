---
change: use-collision-resistant-slug-identities-for-new-bugs
artifact: requirements
---

# Requirements

One requirement is modified (see `deltas/cli.md`):

- REQ-cli-012 retains every existing local backlog, duplicate search, regression, evidence, verification and report rule. New bugs use `BUG-<lowercase-title-slug>-<24-lowercase-hex>` identities from one generator shared by manual `bug add` and automatic failure capture. The generator uses the existing `sha2` 0.10 dependency and a versioned, domain-separated SHA-256 canonical creation payload containing the fixed initial creation fields named in the design, excluding the ID and update timestamp. The first 24 lowercase hexadecimal digest digits form the suffix.
- Title normalization uses lowercase ASCII alphanumeric runs separated by single hyphens, with `bug` as the empty-slug fallback. A 71-byte slug budget keeps the complete ID within the existing 100-byte bound without truncating the digest suffix. IDs are generated once and are not recomputed or renamed during updates. Bug records remain schema 1.
- Legacy `BUG-NNNN` records remain readable and usable without migration or renumbering. Duplicate and related links work in either direction across legacy and new forms. Documentation must tell users of older Chough versions to upgrade for new-form IDs.
- Atomic create-new refuses a same-ID different-content result, preserves the existing record and never falls back to numeric allocation. Evidence, artifact, validation, snapshot, transition, case, HI, requirement and SpecSync identity schemes are unchanged.

`TC-cli-0125` is reserved for the collision and compatibility case. Verification must retain the failure-first observation from independent roots at the same legacy numeric high-water mark before implementation, then link the later pass after the fix.

The delta contains the full replacement text, including all six existing acceptance criteria unchanged. No HI criterion is added or reworded; the trace remains FLOW-8 and LIFE-5.
