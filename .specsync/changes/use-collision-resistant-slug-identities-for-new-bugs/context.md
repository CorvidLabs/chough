---
change: use-collision-resistant-slug-identities-for-new-bugs
artifact: context
---

# Context

## Observed collision

The current allocator in `src/bugs.rs` scans one root's legacy numeric IDs and chooses the maximum plus one. Independent roots with the same numeric high-water mark therefore allocate the same next ID for different reports.

This happened during the Kimi integration. The Kimi line of work recorded the stale archived-resume defect as `BUG-0020` after `BUG-0019`, while main independently recorded the older-workflow approval-matching defect as `BUG-0020` after the same high-water mark. Merge commit `b2e2f19d904aec2a5acd73569c8439f5583c01f9` had a real `.chough/bugs/BUG-0020.json` conflict. Its resolution kept main's record as `BUG-0020` and moved the Kimi record to `BUG-0024`. The archived change `align-kimi-integration-with-new-cycle-done-guidance` records that history. Manual renaming preserved both reports, but it does not make the original identities collision-resistant.

## Constraints and rejected alternatives

Max-plus-one depends on local state that independent roots and racing creators do not share. Same-second timestamps are not a safe substitute: independent creations can occur within one clock second, clocks can be skewed or move backward, and equal titles remain equal. A timestamp may contribute creation data to a digest, but it cannot be the uniqueness mechanism.

Only IDs for newly created bugs change. Existing `BUG-NNNN` records, filenames, evidence links and history must remain readable without migration or renumbering, and bug schema 1 stays unchanged. Rewriting old identities would break durable references and frozen history without fixing future allocation.

Manual `bug add` and automatic failure capture must use one generator. Creation must remain atomic and fail closed on a same-ID different-content result, with no overwrite, numeric fallback or repeated ID generation. `TC-cli-0125` is reserved for a pre-fix independent-root collision reproduction and a later passing verification.

Evidence, artifact, validation, snapshot, transition, case, HI, requirement and SpecSync identities are outside this change. No new dependency, integration, runtime, service, release or deployment is needed. The overall direction has human support, but this native definition remains draft until its own scope approval.
