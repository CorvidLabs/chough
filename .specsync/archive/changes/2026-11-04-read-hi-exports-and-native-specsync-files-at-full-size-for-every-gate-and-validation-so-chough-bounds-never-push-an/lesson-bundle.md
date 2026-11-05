# Lesson bundle: read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Read HI exports and native SpecSync files at full size for every gate and validation so Chough bounds never push an agent to edit human intent or specs (issue 100)
- **Kind**: BugFix
- **Specs**: cli, reporting
- **Paths**: src/records.rs, src/evidence.rs, src/context.rs, src/library.rs, src/stage.rs, src/timing.rs, src/skill.rs, tests/cli.rs, tests/timing.rs, skills/chough-workflow/SKILL.md, skills/chough-workflow/references/build.md, Cargo.toml, Cargo.lock, CHANGELOG.md, docs/cli.md, docs/DECISIONS.md, docs/FRD.md, docs/architecture.md, hi/life.md, INTENT.md
- **Acceptance**: chough evidence add and chough cases check validate a criterion against a HI family export larger than 16384 bytes and still refuse an unknown or retired criterion,chough evidence prune reads an archived finalization.json larger than 65536 bytes and the stage gates read a SpecSync policy larger than 65536 bytes,library index reads an archived change.md larger than 65536 bytes and timing reads approval ledgers larger than 65536 bytes,display truncation markers tell the reader to read the source in full and never to shorten it,the workflow skill states that a Chough size bound never justifies editing HI intent or criteria or specs or SpecSync files and that an oversized native source is a Chough bug to report,Chough records keep their bounds and each new test fails on main 91a1fdd and fledge lanes run verify passes

## Evidence

- Verification commit: `2b930ff96c2edbf8d978cec828d28e0aea80965d`
- Base commit: `91a1fdd82a4e314e5037c6256f17199ef0d604f0`
- Verified by: `specsync check --spec cli --spec reporting`

## From the change's context.md

# Context

Issue https://github.com/CorvidLabs/chough/issues/100, filed by Leif (0xLeif), the
Chough maintainer and the jackdaw product owner. In CorvidLabs/jackdaw PR #41 an
agent following the Chough workflow rewrote the human's HI intent to fit a Chough
size bound: `hi/board.md` intent went from about 1,300 words to 64 and `hi/cost.md`
from about 1,250 to 70. `evidence::validate_criteria` parsed `hi export -- <FAMILY>`
from `records::native`, whose stdout is the 16,384-byte display section, so any family
export over 16 KiB failed with "native HI family export is invalid or exceeds 16384
bytes". The same class of bound applied to native SpecSync files read through
`read_record` (65,536 bytes): archived `finalization.json` (`latest_archive` and
`retention_protection` in src/evidence.rs), archived `change.md` (src/library.rs,
silently dropped when large), `.specsync/sdd.json` (`sdd_policy` in src/stage.rs) and
the timing report's native records (`native_record` in src/timing.rs). REQ-cli-034
(2026-11-03) exempted spec files under `specs/` but not HI or these SpecSync files.

The owner's rule, verbatim (2026-11-04, relayed by the parent agent): "Chough should
only reduce and do all that for its own files in .chough and not the specs, not the
human intent." Captured as LIFE-15 in his words.

Authority: that instruction and issue #100 (its "Proposed fix" items 1 to 4) cover
this bounded fix; the agent account `claude` acts under it. Limits: no merge, tag,
publish, release or human acceptance grade; Chough's own record bounds unchanged.

Out of scope: `chough library snapshot` keeps its 4 MiB copy limit (REQ-cli-034 names
it as a known exception; it decides no gate). Earlier spec splits (history #59,
reporting) cited the 65,536-byte record bound for `specs/cli/requirements.md`; this
change does not undo them.

## From the change's design.md

# Design

- `records::read_native_text` reads a native file within `NATIVE_RECORD_LIMIT`
  (4 MiB, SpecSync's own change-artifact bound) and names that bound when exceeded.
  The four SpecSync readers above use it.
- `context::hi_export_json` runs `hi export` through `records::native_json`, which parses
  the complete stdout up to the 4 MiB parse bound and keeps the display section bounded;
  `validate_criteria` uses the parsed value. `chough context` keeps `hi_export` for display.
- The test-only `with_native_stdout` stand-in now also answers `native_json`, so unit
  tests that validate criteria keep running without a host `hi`.
- Display markers (`records::DISPLAY_TRUNCATED` and the context aggregate markers) say
  "for display only", send the reader to the complete source and say never to shorten it.
  Section state labels are unchanged.
- Skill: one Working rules bullet in SKILL.md (kept under its 6,144-byte entry bound by
  tightening the Host use wording without dropping a fact) and one build.md bullet; the
  previous digests join `KNOWN_PREVIOUS` so installed copies update.

## From the change's testing.md

# Testing

- `tests/cli.rs` `criteria_validate_against_a_hi_family_export_over_16_kib`: a HI export
  over 64 KiB; `evidence add` and `cases check` accept `LIFE-40`, found only past the
  first 16 KiB, and refuse retired `LIFE-2`; the refusal never names 16384.
- `tests/cli.rs` `an_archived_finalization_over_the_record_bound_is_read`: `evidence prune`
  reports native state observed with a 70 KB `finalization.json`.
- `tests/cli.rs` `an_archived_change_md_over_the_record_bound_is_read`: `library index`
  writes the title and criterion of a 70 KB archived `change.md`.
- `tests/cli.rs` `an_sdd_policy_over_the_record_bound_is_read`: a padded disabled policy
  gives `ungated:sdd-disabled`.
- `tests/cli.rs` `display_truncation_points_to_the_complete_source` and the extended
  `aggregate_context_budget_is_explicit`: markers say display only and never shorten.
- `tests/cli.rs` `the_skill_says_chough_bounds_never_justify_editing_native_sources`.
- `tests/timing.rs` `native_approval_ledgers_over_the_record_bound_still_give_their_times`.
- `tests/cli.rs` `records_other_than_the_checkpoint_keep_the_65536_byte_bound` still
  passes: Chough records keep their bounds.
- Every new test failed with `src/` at main 91a1fdd
  (`.chough/artifacts/issue-100/failure-first-on-main-91a1fdd.log`).
- `cargo test --locked` and `fledge lanes run verify` pass.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-042 | `tests/cli.rs`: the six tests above and `aggregate_context_budget_is_explicit` |
| REQ-reporting-004 | `tests/timing.rs`: `native_approval_ledgers_over_the_record_bound_still_give_their_times` |

## Where these lessons go

- `specs/cli/context.md`
- `specs/reporting/context.md`
