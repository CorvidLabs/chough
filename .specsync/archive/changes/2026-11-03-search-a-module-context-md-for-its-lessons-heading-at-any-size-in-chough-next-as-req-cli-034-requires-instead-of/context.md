---
change: search-a-module-context-md-for-its-lessons-heading-at-any-size-in-chough-next-as-req-cli-034-requires-instead-of
artifact: context
---

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
