---
change: use-collision-resistant-slug-identities-for-new-bugs
artifact: research
---

# Research

## Observed problem

The Kimi branch and main independently allocated BUG-0020 for different reports after BUG-0019. Their merge at `b2e2f19` required preserving main's record and moving the Kimi record to BUG-0024, including live reference updates. This is direct evidence that max-plus-one allocation is not safe for parallel branches.

`src/bugs.rs` currently accepts exactly four numeric digits and allocates the highest existing suffix plus one. Both manual add and automatic failure capture use that allocator. Atomic create-new prevents replacement within one root but cannot prevent identical candidates in independent roots. Evidence observations use a separate timestamp scheme and are not part of this change.

## Implementation inputs

The existing bug record has every creation field needed for a deterministic digest without adding a schema field. `sha2 = "0.10"` is already a direct dependency. Existing `write_new` persistence supplies atomic creation. The existing `valid_name` bound is 100 bytes. No dependency, storage format, runtime or external service is needed.

## Alternatives considered

- Bare title slugs: rejected because distinct reports often share titles.
- Timestamp IDs: rejected because independent roots can create the same case or report in one second, and a timestamp alone does not describe the report.
- Numeric reservation across branches: rejected because it requires coordination and does not fix the underlying allocator.
- Random identifiers: not needed; the existing creation payload and SHA-256 provide deterministic collision resistance without another dependency.
- Bulk migration to slugs: rejected because permanent legacy references and old evidence must remain valid.

## Compatibility conclusion

Keep schema 1 and every legacy BUG-NNNN record. Accept both forms in loading, search, reporting, updates and links. Generate new IDs once from the canonical creation payload and never recompute them. Older Chough versions must upgrade before using repositories with new-form IDs. A same-ID different-content candidate still refuses through atomic creation rather than retrying or overwriting.

No external repository or upstream change is required. No open research question remains; the concrete payload and normalization rules are in `design.md`.
