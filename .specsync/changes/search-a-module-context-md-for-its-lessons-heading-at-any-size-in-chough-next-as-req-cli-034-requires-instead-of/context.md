---
change: search-a-module-context-md-for-its-lessons-heading-at-any-size-in-chough-next-as-req-cli-034-requires-instead-of
artifact: context
---

# Context

The review on #61 (https://github.com/CorvidLabs/chough/pull/61#pullrequestreview-5756873687)
found that `chough next` still read a module `specs/<module>/context.md` within the
4 MiB NATIVE_RECORD_LIMIT when pointing at its Lessons heading, and reported
"larger than 4194304 bytes, not searched" for a larger one. Leif (0xLeif), 2026-11-03
10:05 AM MT: "Spec files can grow to whatever number they want." The parent agent
asked for the fix at 10:39 AM MT.

REQ-cli-034 (change `read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies`, same pull request) states the behavior. That change's
approved paths did not list src/main.rs, and SpecSync 6.1.2 accepts new paths only
while a change is a draft, so this change covers src/main.rs with no spec change.
