# Reporting context

## Decisions

2026-11-04: Split from the cli module so each requirements companion stays within
Chough's 65,536-byte structured record bound. After PR #71 merged main a33c747 the
cli companion was 65,698 bytes. Split rather than raise the limit, keep wording,
weaken nothing, as the history split (#59) did; Leif (0xLeif) on 2026-11-03 at
10:05 AM MT: "we should break stuff apart and decouple it if possible but if things
need to be coupled, that's fine." SpecSync 6.1.2 requires `REQ-<module>-<number>`
IDs in a module delta and keeps removed IDs as permanent tombstones, so the moved
requirements were renamed:

| Old ID | New ID | Subject |
|---|---|---|
| REQ-cli-020 | REQ-reporting-001 | Timing report |
| REQ-cli-024 | REQ-reporting-002 | Timing native reads |
| REQ-cli-028 | REQ-reporting-003 | Request-local reporting reads |

The bodies are byte-identical to main a33c747; tests/spec_split.rs pins their
SHA-256 digests. Their Trace lines still name LIFE-10 and the other criteria they
named. Archived native changes, observations, bug records and decisions that cite
the old IDs are historical records and are not rewritten. Earlier timing decisions
are in the cli context and docs/DECISIONS.md.

Chough checks a case's requirement links against its own module only. Six cli
cases linked REQ-cli-020 (two also REQ-cli-024) together with a requirement that
stays in cli; each keeps its cli ID and its staying links and has a TC-reporting
twin with the identical binding for the moved requirement (see testing.md).

## Working files

- src/timing.rs: the timing report, its native reads and budget.
- tests/timing.rs: timing regressions.
- The request-local case-suite reads (REQ-reporting-003) and the bounded native
  reader used by timing are implemented in cli-owned files (src/cases.rs and
  src/records.rs); the case-suite reads are also tested in tests/next.rs.

## Lessons

No module lessons recorded yet; earlier timing lessons are in the Lessons section
of specs/cli/context.md.
