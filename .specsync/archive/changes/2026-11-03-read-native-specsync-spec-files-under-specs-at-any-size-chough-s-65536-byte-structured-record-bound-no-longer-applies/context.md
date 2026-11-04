---
change: read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies
artifact: context
---

# Context

PR #55 (knowledge directory) could not verify after merging main because the cli
requirements companion `specs/cli/requirements.md` grew past Chough's 65,536-byte
structured-record bound. #59 split the cli spec to make room, but that only moved
the limit. The bound is Chough's own: `src/evidence.rs` read `requirements.md`
through `records::read_record` (RECORD_LIMIT 65,536) when `chough evidence add`
checks a requirement ID's owning module. `chough next` also skips a module
`context.md` over 4 MiB (NATIVE_RECORD_LIMIT) when pointing at its Lessons section,
and `chough library snapshot` refuses files over 4 MiB; REQ-cli-034 names both as
known exceptions, and a later native change on a follow-up PR lifts them.
SpecSync itself does not cap spec files.

Leif (0xLeif), 2026-11-03 10:05 AM MT: "Spec files can grow to whatever number they
want. It's mainly just all the artifacts and all of the investigations or evidence
and all the stuff that you're storing. we shouldn't really care about file size for
specs." And: "Ideally, we should have a guidance to try to keep stuff lean and not
try to, you know, just have a single spec file for everything, and we should break
stuff apart and decouple it if possible but if things need to be coupled, that's
fine. and then same thing for files too, we need to make sure that we're not just
making, you know, thousand plus line files and going to ten thousand lines and
everything there, so we need to have some, you know, good sense of composability but
not be crazy about it."

The parent agent relayed both at 10:06 AM MT and asked for this change: remove the
cap for spec files under `specs/`, keep it for Chough's own records, add advisory
guidance that is never a gate.

Ruled out: raising RECORD_LIMIT (it still protects Chough's records); a size gate or
automatic splitting for specs or source files (Leif: "not be crazy about it").
