# Lesson bundle: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Prepare the Chough 0.6.0 release: Cargo 0.6.0 and the dated CHANGELOG with the dogfood results, and the fixes the dogfood found
- **Kind**: Operations
- **Specs**: cli, history, records, reporting
- **Paths**: Cargo.toml, Cargo.lock, CHANGELOG.md, README.md, src/, tests/, .chough/cases/, docs/, skills/, .agents/skills/, knowledge/
- **Acceptance**: Cargo.toml and Cargo.lock name version 0.6.0, and chough --version prints chough 0.6.0
- **Acceptance**: CHANGELOG.md has a dated [0.6.0] section whose upgrade notes give the dogfood results, and tests/changelog.rs passes
- **Acceptance**: After an in-place migration, the local pointer that names a 0.5.x key resolves to its session through the alias, so next and status read it (D1, red then green)
- **Acceptance**: No test helper is named session_id, so CodeQL rust/cleartext-logging has nothing to flag in the tests
- **Acceptance**: Every live KNOW criterion has an active case linking it with a passing receipt recorded through Chough, so the KNOW gap list of #216 is empty
- **Acceptance**: The #148 bug record is verified through Chough: a red run against the code before #157, then a later green run
- **Acceptance**: Each of the four flaky-test bugs is settled through Chough: fixed and verified, or left open with its diagnostics and named to Leif
- **Acceptance**: The closeout leaves chough doctor clean and every earlier archived story byte-identical
- **Acceptance**: For a second native change on one branch, the gate summary counts only that change's own work since its base_commit (D2, REQ-cli-061, red then green), and #190's merged-main case still counts from the merge base
- **Acceptance**: chough migrate's preview says up front that --apply will refuse while native changes are open, naming them, and still writes nothing (red then green)
- **Acceptance**: A journal post has a short human title and a real summary: chough checkpoint --title and --summary set them before the move into done, a title and body derived from the objective and the story are the fallback, and chough journal add --record writes an archived story's post (REQ-records-011, red then green)
- **Acceptance**: Every archived story has a journal post: the 68 earlier stories through journal add --record, each title and summary written from its story, and 0.6.0's own post written again under the new rules; library index --check passes
- **Acceptance**: knowledge/lessons/ holds a few real cross-project lessons from this release, each sourced to recorded evidence or a story
- **Acceptance**: A path under /tmp/ or /private/tmp/ in a case definition gives no doctor leak finding, while a home path there still does (REQ-records-006, red then green)
- **Acceptance**: Every fresh-eyes bug from t38j's report is fixed with a case run red then green and verified: the cases check crash, the context walk, next at plan naming the scope gate (REQ-cli-021), the scope-gate summary showing the delta and plan (REQ-cli-061), labelled Timeline fields (REQ-history-002), a bug's trigger and Fix (REQ-cli-012), receipts in time order (REQ-history-006), none in empty migration-receipt sections (REQ-records-009), gate record's output and note, the authority line, the plain check's entry, and status and the stale next action
- **Acceptance**: A new project's path is in README.md, docs/cli.md and the workflow skill, with specsync change adopt and the delta format, and the CHANGELOG and docs/cli.md describe 0.6.0 as it ships (drift checks red then green)
- **Acceptance**: Every session.md chough migrate --apply writes ends with exactly one newline, so git diff --check passes on the migration commit (jackdaw's finding, red then green)
- **Acceptance**: A stopped command leaves no child it forked as the stop arrived: the stop stress that failed 2 of 24 runs before the fix passes after it, rerun by t382 (red then green)
- **Acceptance**: Every finding of the second fresh-eyes round is fixed with a failing test first, red then green, each bug verified: a gate that drives nothing, or a close with --no-native, refuses a --decided-by that names no configured human (F1, REQ-records-008); next never loops on itself, names the accept preparation and keeps commands whole (REQ-cli-021); the gate list names the ship for a verifying gate (REQ-cli-061); the story lists the latest run per case, its bugs and no stale next (REQ-history-006, REQ-cli-012); direct writes in order and change check --no-commit (REQ-cli-060, REQ-cli-057); a start entry (REQ-history-002); merge_driver_outdated (REQ-history-012); evidence add with delta requirements and optional links (REQ-cli-005); the migrate preview on a migrated tree (REQ-records-009); the output polish and the docs gaps (F2, F7, Kimi 1)
- **Acceptance**: Every finding of the third fresh-eyes round is fixed with a failing test first, red then green, each bug verified: next at accept never passes on SpecSync review command and at test leaves superseded runs out (REQ-cli-021); session close records the gates it withdraws, and the story puts its receipts last and names its bugs (REQ-history-006); gate summaries name the intent and the delta (REQ-cli-061); a failed run body says why (REQ-cli-017); stage moves print a short summary with the full JSON behind --json (REQ-cli-008); the migrate preview is text with --json (REQ-records-009); the dated lesson form documented, the messages, next at plan, context, the migrated verified line, timing and the init hint fixed; and a full local run passes
- **Acceptance**: The fourth round four real findings are fixed with a failing test first, red then green, each bug verified: a stage move replaces a gate close next action, and at accept next names the stale cases rerun before the review gate (REQ-cli-021); the scope summary counts uncommitted change files (REQ-cli-061); the migration follow-ups name chough skill update for an outdated skill copy (REQ-records-009); accept.md writes lessons before the case rerun; F5 to F11 and the host-name rule are recorded as deferred bugs quoting Leif; and a full local run passes

## Evidence

- Verification commit: `0c4cf1845ce013d0e86dd099f272023188a11d05`
- Base commit: `166d1ea1d37971e4c95ee8ba02249a1a46b2a0d2`
- Verified by: `specsync check --spec cli --spec history --spec records --spec reporting`

## From the change's context.md

# Context

Chough 0.6.0 (human-first records) was accepted by Leif and archived on SpecSync 6.3.0
(2026-11-12), and this repository was then migrated to 0.6.0 records with `chough
migrate` (receipt `.chough/archive/migrations/2026-11-12-from-0-5.md`). The archived
change's dogfood plan makes the release preparation the dogfood cycle: one small
native change driven intent to done with the 0.6.0 build only, with any fix the
dogfood finds. Hosted CI is green on the migrated tree (977bd246).

The dogfood so far found:

- D1: an in-place migration keeps the machine-local active-session pointer, which
  0.5.x wrote as the session's key. That key is now the session's alias, but the
  pointer is read as an ID only, so `next` and `status` fail until `session use`
  or `doctor --fix`. Clones never showed it, since a clone drops `.chough/local/`.
- CodeQL's rust/cleartext-logging rule flags a test helper named `session_id` (it
  returns a migrated session's dated folder name, not a secret) where its value
  reaches an assertion message in `tests/common/mod.rs`.
- The KNOW gap list (#216), computed from `hi/knowledge.md` and the recorded
  evidence: KNOW-3, KNOW-6 and KNOW-24 have no case linking them, and KNOW-4 and
  KNOW-15 have cases but no passing receipt in the records.
- #148's local bug record: GitHub #148 is closed and #157 merged on 2026-11-06, but
  the record has no linked failure, so Chough will not mark it verified.

Re-scoped (2026-11-12T15:14Z): asked whether deferred bugs block the 0.6.0 tag, Leif
chose "Block the tag" ("Fix both inside 0.6.0 before tagging. Skip 0.6.1."), so two
more dogfood findings join this change:

- D2: for a second native change on one branch, the gate summary's What changed
  counts from the merge base with main, so it includes the earlier change's work.
  REQ-cli-061 specifies that, so the fix is a contract change (t38c's wording).
- The jackdaw dogfood (t38f): `chough migrate`'s preview exits 0 with a digest while
  native changes are open, and only `--apply` says it will refuse.

Four flaky-test bugs stay open from 0.6.0 (the macOS writer lock timeout, the timing
native-budget regression on MSRV, the timing trace flake, the verify lane's test
step under load). The release gate allows no tag while any Chough bug is open, and a
flake that will not reproduce goes to Leif, never closed to clear the gate.

Re-scoped again (2026-11-12, 15:35Z to 15:41Z, relayed by t370). Leif saw this
repository's first journal post: its title is the raw three-line objective, cut
mid-word in the index, and its body is a stale first bullet ("Plan step 3 … 561
passed"). He chose "yes put the journal fixes in 0.6.0": short human titles, real
summaries, a backfill so each of the 68 earlier archived stories has a post, and 0.6.0's
own post written again under the new rules. Since Chough cannot write a good title or
summary by itself, he chose "Yes, add both (Recommended)": `chough checkpoint
--title/--summary`, which the agent sets before the move into done, and `chough
journal add --record <story>` for the backfill, with a derived title and body as the
fallback. He chose "yes add lessons in 0.6.0": a few real cross-project lessons from
this release in `knowledge/lessons/`. For the jackdaw finding (t38f: `chough doctor`
reports the deliberate `/tmp` `XDG_DATA_HOME` settings in case definitions as leaks and
offers `--redact`, which would break the cases) he chose "Yes (Recommended)": a
temporary-root path in a case definition is not a leak.

Re-scoped a third time (2026-11-12T16:26Z, relayed by t370). A fresh-eyes tester
(t38j, a cold full cycle on a scratch project plus a 0.5.0 migration) rated 0.6.0 at 7
of 10 and found twelve issues. Leif wants 0.6.0 at 90% or better with no open bug at the
tag, and chose "Yes, fix all (Recommended)" for every one: bugs and gaps (the cases
check crash on a module's first change, no new-project path in the docs, `chough
context` reading AGENTS.md files above the project), wrong or misleading messages
(`gate record`'s output and approval note, the stage-move authority line, the empty
entry a plain `specsync change check` leaves, `chough status` cutting its next action
and the next action going stale after a gate closes), and small requirement changes
(`next` at plan naming the scope gate instead of SpecSync's approve, the scope-gate
summary showing the delta and plan, and record polish: labelled Timeline fields, a bug's
Fix line, a labelled trigger, receipts in time order and `none` in empty receipt
sections). The CHANGELOG and docs corrections the report also found were already in
scope.

On 2026-11-12T16:38Z Leif also chose "Yes, fix it (Recommended)" for a finding from
jackdaw's real migration (t38f): `chough migrate --apply` can write a `session.md` that
ends with a blank line, which `git diff --check` rejects.

On 2026-11-12T17:42Z Leif chose "Yes, fix it (Recommended)" for a fifth flaky test that
t38b traced to Chough itself: on macOS, one SIGKILL to a stopped command's process
group can miss a child the group gains while it is signalled, so the child survives.

On 2026-11-12T19:43Z Leif chose "Yes, fix all + re-rate (Recommended)" for the 16
findings of the second fresh-eyes round: t38k's F1 to F13 (rated 7/10 at f69889a9) and
t38h's (1), (2) and (6). Most important: a gate that drives no native step, or a close
with `--no-native`, accepted any `--decided-by`, so intent and design approvals could
be recorded in any name (F1). The rest: `next` looping on itself after a gate, a
confusing accept, a cut ship command, a gate list footer that names a refused reply
(F3 to F6); the story's verified list, its bug links, its stale `next` and its
direct-write order (F8, F9, Kimi 6); the migrate preview on a migrated tree, a start
entry for timing, output polish and an outdated `chough` on `PATH` for the merge
drivers (F10 to F13); `evidence add` and requirements a delta adds (Kimi 2); and the
docs gaps (F2, F7, Kimi 1). Each fix gets a failing test first, and a third fresh
tester re-rates the result against the 90% bar.

On 2026-11-12T21:47Z Leif chose "Yes, fix all + re-rate (Recommended)" for the 12
findings of the third fresh-eyes round (t38m, 8/10 at 425fa948): `next` at accept
passing on SpecSync's own review command (F1) and `session close` dropping an open
human gate unrecorded (F2), the two gate bugs; the dated lesson form undocumented (F3);
gate summaries that name neither the intent nor the delta (F4, F5); two messages (F6);
`next` at test and at plan (F7, F8); about 150 lines of JSON per stage move, now a
short summary with the full JSON behind `--json` so agents lose nothing (F9); the
story's readability: receipts before the story, its bug unnamed, a failed run's body
without its reason, `context`'s JSON block, a migrated bare ID (F10); timing's first
segment (F11); and the init hint for Claude Code and a text migrate preview (F12).
Each fix gets a failing test first, and a fourth fresh tester re-rates the result.

On 2026-11-12T23:32Z, after a fourth fresh-eyes round (t38n, 7.5/10 at a7d6406a), Leif
chose "Fix 4 real ones, then release (Recommended)": fix the 4 real problems with failing
tests first, run the full suite, then his review and the tag; "Polish and the Mac
leak-check go to 0.6.1. Real use on jackdaw will tell us more than another tester." The
four: a gate close's next action outliving the move it named (F1); the scope summary
ignoring uncommitted change files (F2); no pointer to `chough skill update` after a
migration leaves the skill copy outdated (F3); and accept steps that stale the evidence
twice, with `next` routing to the review gate past stale cases (F4). F5 to F11 and the
host-name leak rule are recorded as deferred bugs, quoting him. No fifth tester.

On 2026-11-13T00:58Z Leif chose "Yes (Recommended)" to add one reporting sentence and let
his 00:56Z approval of 0.6.0 ("Approve (Recommended)") stand: SpecSync's scoped review
refused to finalize because `src/timing.rs`, which the F11 timing fixes touched, is owned
by the reporting module this change had not declared. The change now declares reporting,
with a delta that writes into REQ-reporting-001 what timing already does: the `start`
entry begins Cycle 1, and the wall-clock label is said once in the text header. No code
changed.

## From the change's design.md

# Design

D1: the active-session pointer resolves through aliases. `sessions::pointed` returns
the pointer's session ID: the pointer itself when a session folder or story has that
ID, otherwise the one open session whose alias or bare key it is. `active()` and
doctor's `stale_pointer` check both use it, so a pointer an older layout or a
`session use <alias>` left keeps naming its session, and a pointer naming nothing
is still `stale_pointer`. The pointer file is machine-local and is not rewritten.

D2: the gate summary's base is the change's own `base_commit`, unless the merge base
of HEAD with the default branch is not an ancestor of it (default-branch work merged or
rebased in after the change began, #190), or the change records none; a merge base that
is HEAD itself is never used, and with no usable base the summary says so. One more Git
call, no native read.

The migrate preview: when a native change is open, the plan says up front that
`--apply` will refuse, naming each open change. The preview still writes nothing,
and the apply's refusal is unchanged.

The journal: the session record gains `title` (one line, at most 72 bytes) and
`summary` (a list of lines, at most 900 bytes), set by `chough checkpoint --title` and
`--summary` and kept in the story. The post at the move into done uses them; without
them it derives a title from the objective's first clause and a body from the story's
Goal, first decision, newest verified entry and first open thread. `chough journal add
--record <story>` writes an archived story's post with the agent's title and body,
dated by the story's `closed` and timed by its latest Timeline entry on that date, and
refuses a story that already has a post. The backfill runs it once per story, with a
title and summary the agent writes from reading that story. 0.6.0's own post is
removed with Git (Chough never removes a post) and written again with `--record`.

Lessons are notes under `knowledge/lessons/` as REQ-cli-044 already defines them
(`id`, `title`, `type: lesson`, `tags`, `date`, `applies_to`, `source`), each from
recorded evidence; no new command.

Leaks: doctor's scan skips a system-path match under `/tmp/` or `/private/tmp/` in a
`.chough/cases/*.cases.toml` file. Every other kind still counts there, and every other
file is scanned as before.

The fresh-eyes fixes. `cases check` reads a module's missing `requirements.md` as
empty, so a requirement only the approved delta adds is accepted as REQ-cli already
says. `chough context` stops its AGENTS.md walk at the Git root. `next` at plan routes
to the scope gate as it routes to the review gate at accept, and shows no `specsync
change approve`. The gate summary at a scope gate counts and names the change's own
Markdown files first. Timeline fields with parts become labelled lines, read back
alongside the JSON values older entries hold. A bug's Steps start with `Trigger:`, `bug
update --fix` sets its Fix, and verifying a bug with no Fix names the passing run. A
story's receipts sort by observation time; a migration receipt says `none` under an
empty section. `gate record` reports the native step it ran and its note names `gate
record`; the stage-move authority line says what gates do since #236; a plain `specsync
change check` leaves an entry that says what ran; `status` gives its next action whole,
and closing the last gate holding a stage clears a next action that described it.

Everything else is configuration, evidence or text: the Cargo version, the CHANGELOG
section, a test helper's name, and case links under `.chough/cases/`.

## From the change's testing.md

# Testing

Each fix is red before it and green after, recorded through Chough with `chough
cases run`: TC-records-a-0-5-pointer-still-names-its-session (D1) and
TC-cli-empty-change-dir-not-open (#148), TC-cli-gate-summary-counts-from-the-changes-own-base (D2, with #190's TC-cli-190-gate-summary-counts-the-change still green) and TC-records-the-migrate-preview-warns-of-open-changes. The journal: TC-records-journal-post-uses-the-title-and-summary,
TC-records-journal-post-derives-a-title-and-body and TC-records-journal-add-record-backfills-a-story,
each red then green. Temporary paths: TC-records-doctor-case-temp-paths-are-not-leaks,
red then green. After the backfill, every archived story has a post whose `record`
names it, and `chough library index --check` passes. The fresh-eyes fixes each get a case run red then green, linked to its bug; the
CHANGELOG and docs corrections get drift checks (TC-cli-changelog-0-6-0-names-only-what-ships,
TC-cli-docs-name-what-cases-run-returns, TC-records-migration-docs-name-every-key). The KNOW gap list is recomputed from
`hi/knowledge.md` and the records and must come out empty. `tests/changelog.rs`
checks the dated [0.6.0] section and the Cargo version. Full suites run on hosted CI;
local runs are targeted. `chough doctor` is clean after the closeout.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
- `specs/records/context.md`
- `specs/reporting/context.md`
