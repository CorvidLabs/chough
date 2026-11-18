---
change: fix-307
artifact: context
---

# Context

GitHub #307, found while working #301. `chough change ready` printed "Nothing is
missing." for a `--no-spec-change` change with `src/checkpoint.rs` and
`src/main.rs` in its affected paths and no affected spec. Closing the scope gate
then ran `specsync change approve`, which refused: "no declared module
canonically owns these affected paths". Both files are in the cli spec's
`files:` list. The human's decision failed at the moment it was recorded.

REQ-cli-067 says the report names a finding for a fixed list "and for nothing
else", and that list has no entry for a missing owning module. So this is a
contract change: one more finding in the list. The same refusal met four other
changes on 2026-11-17 (fix-272, fix-267, fix-262, fix-260).

Declaring the module can also add an interview question (`constraints`, for a
module with recorded lessons). The report already lists every question SpecSync
asks, so the new finding only says that declaring the module can add questions;
the next report lists them.

This change is built on `feat/change-ready` (ea159cf), which is not on `main`.
