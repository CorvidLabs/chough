---
change: use-slug-identities-for-all-new-chough-records
artifact: context
---

# Context

## Problem

Chough still allocates several durable identities from local counters or second-resolution timestamps. `src/cases.rs` accepts only `TC-<module>-NNNN` and derives automatic observation names from a lowercase case ID, the current second and, after a collision, a numeric suffix. `src/validate.rs` uses the same timestamp and counter pattern for repeated runs. `src/library.rs` scans dated snapshot directories and selects the next numeric suffix. Independent roots can choose the same identity for different history, and identical work repeated within one second has no intrinsic distinction before local counter allocation.

The recently archived bug-identity change fixed the same independent-root problem for bugs with readable slugs and deterministic creation digests. It deliberately left cases, observations, validation runs, artifacts and snapshots unchanged. This change completes that identity work without reopening bug generation or introducing another store.

## Collision sources and consequences

Bare slugs collide whenever titles, case names or reasons repeat. Timestamps collide at their clock resolution and can move backward or differ across machines. Local numeric counters depend on state that independent roots and racing creators do not share. A deterministic hash of the complete observation payload also collides intentionally when repeated runs have identical inputs and timestamps, so it cannot preserve each repeated attempt as separate history. Random generation can still collide, so atomic no-replace creation remains necessary rather than optional.

A case ID is different: it is a curated durable definition identity, not an allocation for each execution. Numeric legacy cases must remain stable, while newly authored cases need readable slugs and exact global duplicate refusal. Case definition digests and execution observations remain separate.

## Constraints

All current record names pass through the existing 100-byte `valid_name` ceiling. New forms must fit that ceiling without truncating their random suffixes. Legacy case, observation, validation and snapshot names remain readable, linkable and resumable at their existing paths. There is no migration, renumbering, schema rewrite or edit to frozen archives.

Automatic observations, validation runs and snapshots need independent random 96-bit suffixes so identical repeated runs remain distinct. Their existing timestamp fields remain the chronology source. Automated logs keep the exact path `.chough/artifacts/<observation-id>.log` and inherit the observation ID rather than generating another identity.

The lockfile already contains getrandom 0.4.3. Chough will use that package directly, with no runtime, service, central allocator or persistent random state. Creation makes one candidate and one no-replace attempt. Existing content is never overwritten, and no timestamp, numeric, digest or random fallback follows a collision or write failure.

Explicit observation IDs remain available in the new form but become optional. A newly written old-style observation ID receives an omit-ID remedy; old IDs on stored records remain valid for reads and resume. Bug generation, bug known-answer vectors, transitions, preference and consent keys, stop markers and pruning-ledger entry shapes stay unchanged. HI, requirement and SpecSync identities remain with their native owners.

This native change is draft. This assignment covers only `context.md`, `design.md`, `plan.md` and `research.md`; no approval, implementation, canonical spec edit or Chough record write is part of it.
