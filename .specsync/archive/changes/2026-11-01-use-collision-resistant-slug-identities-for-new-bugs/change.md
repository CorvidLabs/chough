---
id: use-collision-resistant-slug-identities-for-new-bugs
state: archived
type: feature
base_commit: 7357d41ab92e737fea1ad1e733c38f5447226fb9
---

# Use collision-resistant slug identities for new bugs

## Intent

Use collision-resistant slug identities for new bugs

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Manual and automatic creation of distinct bug reports in independent roots at the same legacy numeric high-water mark produces different IDs in the form BUG-<lowercase-title-slug>-<24-lowercase-hex>; the pre-fix identical-ID failure is recorded and linked before implementation and the later pass verifies the fix.
- Title normalization produces bounded lowercase ASCII slugs and preserves the complete 24-hex digest within the existing 100-byte ID limit. IDs are generated once from a versioned domain-separated SHA-256 creation payload and are never recomputed during updates.
- Every legacy BUG-NNNN record remains readable and usable without bulk migration or renumbering. Bug schema 1 is unchanged; duplicate and related links work across legacy and new forms, and the docs state that older Chough versions need an upgrade for new IDs.
- Atomic create-new refuses a same-ID different-content result without overwriting or falling back to numeric allocation. Evidence, artifact, validation, snapshot, transition, case, HI, requirement and SpecSync ID schemes are unchanged.
- TC-cli-0125 covers the collision and compatibility behavior if still free on the base. Focused Rust tests, affected cases, cargo fmt, Clippy with warnings denied, full cargo test, fledge verify and pre-push pass with CI-matching tools. No new integration, runtime, service, release or deployment is introduced.

## No-spec Rationale

Not applicable
