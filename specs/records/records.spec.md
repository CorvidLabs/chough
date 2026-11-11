---
module: records
version: 1
status: active
files:
  - src/markdown.rs
  - src/leaks.rs
  - src/identity.rs
  - tests/ids.rs

db_tables: []
depends_on:
  - cli
---

# Chough records

## Purpose

Own the format of every record Chough writes, so that a person can read and edit it
and an agent can parse it: one Markdown file per record with front matter in a strict
subset of YAML, date and slug names with clash suffixes and aliases, and the leak
guard that keeps local paths, user and host names, email addresses and credentials
out of committed records. Added by the Chough 0.6.0 change (human-first records); the
session, evidence, bug, journal and migration requirements of this module are wired
into the commands by that change's later steps. See requirements.md.

## Public API

The binary exposes CLI commands; it does not export a Rust library API. This module
covers the crate-internal record layer the commands use: reading and writing record
files and their sections, naming new records, resolving IDs and aliases, and checking
or redacting every value before it is written.

| Internal symbol | Structural behavior |
|---|---|
| `OBSERVATION_SLUG_LIMIT` | Limits an observation slug so the complete OBS ID fits 100 bytes. |
| `VALIDATION_SLUG_LIMIT` | Limits a validation slug so the complete VAL ID fits 100 bytes. |
| `SNAPSHOT_SLUG_LIMIT` | Limits a snapshot slug so the complete SNAP ID fits 100 bytes. |
| `slug` | Normalizes bounded nonempty lowercase ASCII slugs with caller-selected fallbacks. |
| `valid_case_id` | Accepts curated slug and exact legacy numeric case IDs with the module-aware bound. |
| `valid_observation_id` | Accepts the exact new OBS form with a complete 24-digit lowercase hex suffix. |
| `valid_validation_id` | Accepts the exact new VAL form with a complete 24-digit lowercase hex suffix. |
| `valid_snapshot_id` | Accepts the exact new SNAP form with a complete 24-digit lowercase hex suffix. |
| `observation` | Allocates one OBS candidate from one random 96-bit draw. |
| `validation` | Allocates one VAL candidate from one random 96-bit draw. |
| `snapshot` | Allocates one SNAP candidate from one random 96-bit draw. |
| `require_explicit_observation_id` | Applies new-form explicit observation validation and the old-style omit-ID remedy. |
| `with_random_results` | Test-only random-source seam; no production override. |
| `random_draws` | Test-only draw-count seam used to prove there is no retry. |
| `NAME_LIMIT` | The longest record name, 80 bytes, a clash suffix included. |
| `utc_date` | Formats a Unix time as its UTC `YYYY-MM-DD` date. |
| `dated_name` | Names a new record `<date>-<slug>`, cut back at a hyphen to leave room for a clash suffix. |
| `create_named` | Creates a record by create-new, with one clash suffix on a clash and a refusal on a second. |
| `RECORD_BOUND` | The 65,536-byte bound of every record kind not named otherwise. |
| `SESSION_BOUND` | The 1,048,576-byte bound of a session record and of an archived session story. |
| `RECEIPT_BOUND` | The 4,194,304-byte bound of a migration receipt. |
| `Kind` | A record kind with its `name` and its `bound`; also the kind of a leak match, with its `name`. |
| `name` | The display name of a record kind or of a leak kind. |
| `bound` | The byte bound of a record kind. |
| `Value` | A front matter value: a scalar, a list of scalars or a list of flat maps. |
| `Record` | One parsed record: its front matter entries, kept in order with their exact text, and its body. |
| `new` | A new record with a body and no front matter; for `Local`, local facts given explicitly. |
| `get` | A front matter value by key. |
| `scalar` | A scalar value by key. |
| `list` | A list of scalars by key, empty when missing. |
| `maps` | A list of flat maps by key, empty when missing. |
| `keys` | The front matter keys in order. |
| `set` | Sets a key in place, keeping its exact text when the value is unchanged. |
| `set_scalar` | Sets a scalar value. |
| `remove` | Removes a key. |
| `body` | The Markdown body. |
| `section` | The text of a `## ` section. |
| `append_to_section` | Appends lines to the end of a `## ` section, or creates it at the end of the body. |
| `render` | The whole record file. |
| `render_within` | The record file, refused when over its kind's bound. |
| `fields` | Every value a record would write, with its field name, for the leak guard. |
| `check_leaks` | Refuses a record whose values leak, naming the field and kind. |
| `redact_leaks` | Redacts every leaking value in a record and counts the matches. |
| `resolve` | Finds the record whose `id` or alias is an ID, refusing an ID two records hold. |
| `write_new` | Writes a new record by create-new within its bound. |
| `replace` | Replaces a record atomically within its bound. |
| `parse` | Reads a record, refusing front matter outside the subset with the file, line and rule. |
| `Match` | One leak match: its kind and byte range. |
| `Local` | The local user and host names a value is checked against. |
| `current` | This machine's local facts, read once per process. |
| `check` | Refuses a value that leaks as `refused:leak`, naming the field and kinds, never the text. |
| `redact` | Replaces every leak in a value with `<redacted:<kind>>` and counts them. |
| `find` | Every non-overlapping leak match in a value. |

## Invariants

1. A record file starts with front matter between two `---` lines; front matter
   outside the subset, a duplicate key or a missing closing line is refused with the
   file, the line and the rule, and nothing is written.
2. A key Chough does not know, a blank line and a comment line are kept byte for byte
   when Chough rewrites the front matter; every body byte outside the sections Chough
   appends to is kept.
3. A record is never larger than its bound: 65,536 bytes, or 1,048,576 for a session
   and an archived session story, or 4,194,304 for a migration receipt.
4. A new record's name is a UTC date and a slug of at most 80 bytes with any clash
   suffix, never a count; a clash draws one suffix, and a second clash is refused
   without another draw.
5. A refusal for a leak names the field and the kind of match, never the text.

## Behavioral Examples

```gherkin
Given a record whose front matter holds a key Chough does not know
When Chough sets another key and writes the record
Then the unknown key's line is unchanged and in place

Given a bug titled "  Fix   LOGIN... now!  " on 2026-11-10
When Chough names it
Then its name is 2026-11-10-fix-login-now

Given an evidence summary holding a path under /Users/
When Chough writes it without --redact
Then the write is refused as refused:leak naming the field and home-path, and no file changes
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Front matter outside the subset | A nested map, an anchor, a tag, a block scalar, a single-quoted value, a duplicate key or no closing line | Refuse with the file, the line and the rule; write nothing |
| Record over its bound | The rendered record exceeds its kind's bound | Refuse naming the bound; every file stays byte-identical |
| Second name clash | The suffixed name also exists | Refuse with nothing written and no further draw |
| Shared alias | Two records hold the same ID or alias | Refuse naming both records |
| Leak | A value holds a local path, user or host name, email or credential | Refuse as `refused:leak` naming the field and kind, or redact with `--redact` |

## Dependencies

Depends on the cli module for bounded reads, the project-relative path checks, the
atomic and create-new writers and the random source of REQ-cli-023. The other modules
use this one for every record they write.

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-11-11 | Created by the Chough 0.6.0 change (human-first records): the record layer, names and the leak guard; `src/identity.rs` and `tests/ids.rs` move here from cli |
