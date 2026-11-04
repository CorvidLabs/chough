# Lesson bundle: search-a-module-context-md-for-its-lessons-heading-at-any-size-in-chough-next-as-req-cli-034-requires-instead-of

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Search a module context.md for its Lessons heading at any size in chough next, and remove next's named 4 MiB exception from REQ-cli-034 while library snapshot's exception stays
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/main.rs, tests/next.rs, docs/cli.md, CHANGELOG.md, specs/cli/, .chough/cases/
- **Acceptance**: `chough next` reports the 1-based line of `## Lessons` in a module context.md larger than 4 MiB, CRLF included, and `no Lessons section` for one without it, never a size note
- **Acceptance**: tests/next.rs::next_searches_a_context_md_larger_than_4_mib fails before the fix and passes after it, and the existing lessons pointer test still passes

## Evidence

- Verification commit: `0df760fb7354821ad841fa3756d52e7fd628b8d9`
- Base commit: `45f7012197e0aa5c2ffdb163cc0025fd09b37b1a`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The review on #61 (https://github.com/CorvidLabs/chough/pull/61#pullrequestreview-5756873687)
found that `chough next` still read a module `specs/<module>/context.md` within the
4 MiB NATIVE_RECORD_LIMIT when it pointed at the Lessons heading, and reported
"larger than 4194304 bytes, not searched" for a larger file. Leif (0xLeif), 2026-11-03
10:05 AM MT: "Spec files can grow to whatever number they want."

Review 5757495438 on #61 narrowed REQ-cli-034 to name `next`'s 4 MiB search and
`library snapshot`'s 4 MiB refusal as known exceptions. Leif then closed #61 for #66
(2026-11-03 1:47 PM MST, "Close #61 and open a fresh replacement PR"). Change
`read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies`
is accepted and archived on #66 with that narrowed wording.

Review 5759349208 on #66 found this change's earlier definition stale against the
narrowed requirement: its paths covered only src/main.rs, and its records said
"as REQ-cli-034 requires". The change ID keeps that earlier wording because IDs are
immutable. The definition was edited and re-approved, as SpecSync 6.1.2 supports for
an approved change: it now declares the cli spec, the test, docs, changelog and cases,
and it MODIFIES REQ-cli-034 to remove only the `next` exception. `library snapshot`
stays a named exception for its own later change. LIFE-15 is not part of this change.

## From the change's testing.md

# Testing

- tests/next.rs `next_searches_a_context_md_larger_than_4_mib` (case
  TC-cli-next-searches-large-context-md) and the existing
  `next_points_to_the_lesson_source_and_counts_archived_bundles` (TC-cli-0129).
- fmt, lint, the full tests and `fledge lanes run verify`.

## Where these lessons go

- `specs/cli/context.md`
