---
change: change-leaks
artifact: design
---

# Design

- `src/change.rs` (cli): before SpecSync runs, the description, `--rationale` and answer are checked as a written value is (`leaks::guard`); a leak refuses as `refused:leak` naming the argument and kind, or under `--redact` is replaced with `<redacted:<kind>>` before SpecSync gets it, with one count line on standard error; a control character refuses under `--redact` too. The `--close-gate` text in the relay refusal is named by what it holds (`leaks::shown`).
- `src/doctor.rs` (cli): `tracked` adds `.specsync/changes/` except each change's `deltas/`, which hold requirement text as `specs/` does (a delta that quotes REQ-records-006's own example paths would otherwise be a finding: measured, this draft's own records delta gives 5); `--fix --redact` skips a leak finding there, naming `chough change answer` or a send-back. `.specsync/archive/` is not read.
- `src/evidence.rs` (cli): an explicit evidence ID that is not a valid name says what it holds, as line 922 does.
- `src/leaks.rs` (records): only a shared helper, such as the count line, if one is needed.
