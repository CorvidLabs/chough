---
change: read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an
artifact: research
---

# Research

Every reader of a file Chough does not own was listed from `read_record`,
`read_bounded_record`, `native(` and `file_section` call sites in src/:

| Site | Source | Bound before | Use |
|---|---|---|---|
| evidence.rs `validate_criteria` | `hi export -- <FAMILY>` | 16,384 (display section) | validation |
| evidence.rs `latest_archive`, `retention_protection` | archived `finalization.json` | 65,536 | retention protection |
| library.rs archive entries | archived `change.md` | 65,536, silently empty | knowledge entry |
| stage.rs `sdd_policy` | `.specsync/sdd.json` | 65,536 | stage gates |
| timing.rs `native_record` | native `approvals.json`, `verification.json`, `review.json`, `finalization.json` | 65,536 | timing report |
| context.rs, records.rs `file_section`, `native` | AGENTS, INTENT, HI export, spec companions | 16 KiB / 128 KiB | display only, kept |

Already at full size or the native bound: spec companions (`read_spec_file`, REQ-cli-034),
library `finalization.json` and `verification.json` (`read_json`), change ledgers and
deltas (`native_file` in stage.rs), and native status JSON (`native_json`).
Chough's own records (session, evidence, bugs, preferences, validation runs, case files,
library manifests) keep their bounds.
