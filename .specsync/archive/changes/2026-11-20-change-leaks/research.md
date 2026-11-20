---
change: change-leaks
artifact: research
---

# Research

Choices, each the lead's recommended one (draft-412):
- (A, chosen) A description, `--rationale` or answer holding a leak is refused before SpecSync runs, with `--redact` as the way out, as a checkpoint decision is. (B) Always redacting would silently change the text a person gave; (C) refusing with no way out would leave prose like a repro path only rewordable.
- (1, chosen) `doctor --leaks` scans `.specsync/changes/` too, and `--fix --redact` skips findings there, naming the way out, since rewriting SpecSync's record would invalidate the change's approval. (1b) redacting in place would force a re-approval; (2) scanning `.specsync/archive/` would fail CI for good on 7 of the 22 repositories, Chough included.
- Every other place Chough hands SpecSync free text for a committed record is already checked: an approval's `--note` comes from the guarded gate text and relay; `--words`, `--where`, `--recorded-by` go through `guard_relay`; `--verdict` is pass or block; `--actor` and `--reviewer` come from the identity file; `ship`, `check` and `status` take only an ID (#413).
- `--redact` is the existing global flag, so `src/main.rs` is not a path; the workflow skill already says a record never holds a local path or credential, so no skill file is a path.
