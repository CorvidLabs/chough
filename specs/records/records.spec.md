---
module: records
version: 7
status: active
files:
  - src/markdown.rs
  - src/leaks.rs
  - src/identity.rs
  - src/actor.rs
  - src/humans.rs
  - src/session_md.rs
  - src/journal.rs
  - src/migrate.rs
  - src/evidence_migrate.rs
  - src/local.rs
  - tests/ids.rs
  - tests/actor.rs
  - tests/humans.rs

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
| `require_explicit_observation_id` | Applies new-form explicit observation validation and the old-style omit-ID remedy. |
| `with_random_results` | Test-only random-source seam; no production override. |
| `random_draws` | Test-only draw-count seam used to prove there is no retry. |
| `NAME_LIMIT` | The longest record name, 80 bytes, a clash suffix included. |
| `utc_date` | Formats a Unix time as its UTC `YYYY-MM-DD` date. |
| `dated_name` | Names a new record `<date>-<slug>`, cut back at a hyphen to leave room for a clash suffix. |
| `create_named` | Creates a record by create-new, with one clash suffix on a clash and a refusal on a second. |
| `suffixed` | Adds one clash suffix to a name from one fresh draw of 24 random bits. |
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
| `remove_item` | Removes the line of one item of a `## ` section, matched by its exact text, such as a superseded case run's link once its session has finished. |
| `insert_first_item` | Inserts one item before the first item of a `## ` section, or appends it as `append_to_section` does when the section has none, keeping every other byte. |
| `insert_section_before` | Inserts a `## ` section before the first of the given sections the body has, or appends it at the end of the body, keeping every other byte. |
| `RECEIPTS` | The heading of the section an archived story ends with, holding its receipts. |
| `receipts_section` | Writes an archived story's `## Receipts` section: `receipts` as one fenced YAML block, one map per receipt, by the front matter's rules. |
| `with_receipts_section` | An archived story's text with its last `## Receipts` section replaced by one holding the given receipts, or added when it has none. |
| `story_receipts` | An archived story's receipts, from its `## Receipts` section or, for a story written before that section, from its front matter; a block that does not read is an error naming the file. |
| `replace_item` | Replaces one item of a `## ` section by its exact text, for a line Chough wrote whose target no longer exists, such as a session entry's link once the session has finished. |
| `mark_item` | Marks one item of a `## ` section on its own line, the one in-place edit Chough makes to a section it owns. |
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
| `current` | This machine's local facts, read once per process; for `Actor`, the acting agent this process runs as, or none. |
| `check` | Refuses a value that leaks as `refused:leak`, naming the field and kinds, never the text. |
| `redact` | Replaces every leak in a value with `<redacted:<kind>>` and counts them. |
| `find` | Every non-overlapping leak match in a value. |
| `find_in_case_definition` | The leak matches in a case definition's value, where a path under `/tmp/` or `/private/tmp/` is a deliberate test setting and not a match, while a home path, user or host name, email or credential still is. |
| `Actor` | The acting agent of a record (REQ-records-007): its agent, model and hc terminal, each only when known; an unauthenticated claim. |
| `AGENT` | `CHOUGH_ACTOR`, the variable that names the agent. |
| `MODEL` | `CHOUGH_ACTOR_MODEL`, the variable that names the agent's model. |
| `TERMINAL` | `CHOUGH_HC_TERMINAL`, the variable that names the hc terminal. |
| `CLAUDE_CODE` | `CLAUDECODE`, set by Claude Code; it names the agent `claude-code` when no override does. |
| `HC_TERMINAL` | `HC_TERMINAL_ID`, set by hc; it names the terminal when no override does. |
| `from_lookup` | The actor from a variable lookup, leaving out with a warning each value that leaks, is too long, or is a terminal other than a plain hc name. |
| `account` | The agent account `<agent>-<terminal>` a native step records when an agent acts on authority given in advance. |
| `display` | The actor as one line for people and as a Timeline entry's `by` field: `<agent> (<model>, hc <terminal>)`, leaving out what is unknown. |
| `from_by` | The actor a `by` field gives, as `display` writes it. |
| `front_value` | The actor as the one flat map a record's front matter `actor` holds. |
| `from_front` | The actor a front matter `actor` value holds, when it is one well-formed map. |
| `set_redacting` | Records whether this command was given `--redact`. |
| `redacting` | Whether this command was given `--redact`. |
| `guard` | Checks one value for leaks, or redacts it in place under `--redact`, returning how many matches were redacted. |
| `Drives` | The native step an open gate drives when a human closes it: approve, review or none (REQ-cli-030). |
| `default_for` | The step a gate drives by default: approve for one held at plan, review for one held at accept, none otherwise. |
| `Human` | One configured human: display name, aliases and SpecSync actor (REQ-records-008). |
| `LOCAL_FILE` | `.chough/local/identity.toml`, the project's identity file, which replaces the user file when present. |
| `USER_FILE` | `chough/identity.toml` under `XDG_CONFIG_HOME` or `~/.config`, the user identity file. |
| `Step` | A native step run as a human: an approval with its note, or a review with its verdict. |
| `arguments` | The SpecSync arguments of a step, with the human's configured actor. |
| `summary` | A step as one line for the decision and the output, without the note's text. |
| `approval_note` | The note an approval records: the human's words, where and when, and the relaying agent. |
| `Ran` | What a native step left: SpecSync's output and exit code. |
| `run` | Runs a step as a human in the project root, keeping the direct-write baseline current. |
| `Written` | One 0.6.0 file the migration writes create-new, with its bytes (REQ-records-009). |
| `Carried` | What one mapper carries into 0.6.0: the 0.5.x sources it read, the files it writes, the aliases it keeps, the files it removes with why, the leaks it kept and the gaps it found. |
| `SessionHome` | Where a 0.5.x session went, by its 0.5.x key: its open session folder or its archived story. |
| `Receipt` | The receipt map of one carried observation, as a story's or a bug's `receipts:` list holds it (REQ-history-006). |
| `ObservationHomes` | Where each 0.5.x observation went: an open session's evidence entry or a receipt; one in neither was removed. |
| `names` | Each of a list of 0.5.x observation IDs by the name of the entry or receipt it became, once each and in order; a removed one keeps its ID. |
| `legacy_paths` | Every 0.5.x record path in the project, sorted; any one makes the tree unmigrated (REQ-records-010). |
| `receipt_path` | The migration receipt's path under `.chough/archive/migrations/`, fixed when the plan is built. |
| `LOGS` | `.chough/local/logs`, where an executed case's raw log stays machine-local (REQ-records-005). |
| `EXTERNAL` | `.chough/local/external-artifacts.toml`, the map of external artifacts' paths and fingerprints by neutral ID. |
| `log_path` | The local raw log of an executed case's evidence entry. |
| `write_log` | Writes a captured log by create-new, refusing an existing file, and returns its path. |
| `External` | One external artifact as the local map holds it: its path and fingerprint. |
| `draw_external_id` | A fresh neutral ID `ext-<6 lowercase hex>` from one random draw; a clash is refused without another draw. |
| `save_external` | Records an external artifact's path and fingerprint under its new neutral ID, once its entry is written. |
| `external` | The external artifact a neutral ID names on this machine, or none. |
| `toolchain_limitation` | The limitation an evidence entry records when the local tools differ from the CI pins (BUG-0022), or none. |
| `Binary` | One binary 0.6.0 file the migration writes create-new, such as a stripped key screenshot (REQ-records-013); never redacted as text. |
| `ensure_migrated` | Refuses a command on a tree holding any 0.5.x record as `refused:unmigrated`, naming up to 5 paths and the count of the rest (REQ-records-010). |
| `exempt_from_guard` | Marks this process as running `migrate`, `doctor` or `skill`, which REQ-records-010 lets read a 0.5.x tree. |
| `ensure_migrated_under_lock` | The same guard once a writer holds the checkpoint lock, unless this process is exempt, so no writer saves into a mixed tree. |

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
| 2 | 2026-11-12 | chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an: Chough 0.6.0 human-first records: one Markdown file with YAML front matter per record, date+slug session folders with an append-only timeline that fold into one archived story at finish, a knowledge vault that links and never copies, no local facts or credentials in committed records, a human's gate approval drives the SpecSync approval as that human, and chough migrate converts 0.5.x projects with a digest receipt |
| 3 | 2026-11-12 | prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood: Prepare the Chough 0.6.0 release: Cargo 0.6.0 and the dated CHANGELOG with the dogfood results, and the fixes the dogfood found |
| 4 | 2026-11-13 | 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak: 0.6.1: the leak check does not read an ordinary word equal to a short host name as a host-name leak |
| 5 | 2026-11-13 | fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply: Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply |
| 6 | 2026-11-14 | the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries: The leak check now flags an scp-style host path, `host:/path`, as the host name and the path it carries |
| 7 | 2026-11-14 | the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275: The leak check flags an scp-style host path as the host name and the path it carries (#275) |
