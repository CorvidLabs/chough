---
change: split-the-cli-spec-move-the-retention-and-transition-history-requirements-req-cli-018-and-req-cli-025-to-req-cli-027-to
artifact: context
---

# Context

PR #55 (knowledge directory) cannot verify after merging main: its cli requirements
companion grows to 66,244 bytes, over Chough's 65,536-byte structured record bound
(`read_record`, RECORD_LIMIT in src/records.rs). Main already sits at 65,051 bytes,
so only 485 bytes remain for any future cli requirement change.

The parent agent chose, at 2026-11-03 9:46 AM MT, to split the cli spec module
rather than raise the limit, keeping every requirement's wording, documenting an ID
mapping if SpecSync needs one, and weakening nothing.

SpecSync 6.1.2 needs one: a requirement in a module delta must be named
`REQ-<module>-<number>`, and a removed requirement ID is a permanent tombstone. So
the moved requirements get new `REQ-history-` IDs. A source file may be owned by one
spec only, and every spec must own at least one file.

The retention and transition-history group (REQ-cli-018, REQ-cli-025, REQ-cli-026,
REQ-cli-027) was chosen because no case links one of them together with a
requirement that stays in cli, and it has natural owned files: src/history.rs,
tests/history.rs and tests/evidence.rs. Other candidate groups (timing, bugs,
stage gates) share cases with requirements that stay, which would mean dropping or
duplicating case links.

Already ruled out: raising the limit (parent decision); shortening requirement text
(would weaken requirements); keeping `REQ-cli-` IDs in another module (SpecSync
refuses it).
