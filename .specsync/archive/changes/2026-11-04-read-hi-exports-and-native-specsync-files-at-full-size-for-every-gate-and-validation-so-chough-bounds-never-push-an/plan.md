---
change: read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an
artifact: plan
---

# Plan

## Approach

Switch each native reader to the native bound or the full parse, keep display bounds and
make their markers point to the source, and state the rule in the skill. Failing-first
tests in tests/cli.rs and tests/timing.rs.

## Out of scope

`library snapshot`'s 4 MiB copy limit; undoing earlier spec splits; any change to
Chough's own record bounds; release, tag or publish.

## Steps

1. Failing-first tests on main 91a1fdd.
2. records.rs, context.rs, evidence.rs, library.rs, stage.rs, timing.rs.
3. Skill bullets and `KNOWN_PREVIOUS` digests; refresh the installed copy.
4. LIFE-15, D-020, docs/cli.md, docs/architecture.md, docs/FRD.md, CHANGELOG.md, version 0.3.0 (Leif chose 0.3.0 on 2026-11-04 because the release also carries new features).
5. `fledge lanes run verify`, `specsync change check`.

## Risks

A 4 MiB native read costs more memory than 64 KiB; it is SpecSync's own bound and is
already used for change ledgers. The SKILL.md entry bound forced tighter Host use wording.

## Constraints consulted

REQ-cli-032 (Chough record bounds stay), REQ-cli-034 (spec files at any size), LIFE-15.
