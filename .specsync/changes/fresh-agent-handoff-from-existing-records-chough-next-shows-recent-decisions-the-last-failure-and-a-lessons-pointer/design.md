---
change: fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer
artifact: design
---

# Design

The slice links and shows records that already exist. It adds no store, no record
kind, no command and no authority. Everything new in `chough next` is read from the
checkpoint, `.chough/evidence/`, `.chough/bugs/`, `specs/<module>/` and
`.specsync/archive/changes/`, all read-only and bounded by the existing record limits.

## Assumption (stated, not verified)

Decisions stay `Vec<String>` in checkpoint schema 2. Agents write them with a prefix
convention, read best-effort:

```text
decided: <what> because <reason>
rejected: <approach> because <reason>
failed: <experiment> because <reason>
```

The prefix must start the string, lowercase, followed by a colon. The reason is the
text after the last ` because `. Any other entry (older free text, dated entries such
as `2026-10-31: ...`) is shown whole with no kind, so the latest-N rule is the
fallback. The assumption is that agents following the skill write prefixes often
enough for the block to help; if they do not, the block still shows the 5 newest
decisions, which is today's information moved into the focused view. No migration,
no new field, no validation of the convention (an unknown prefix such as `note:` is
just text).

## Archived-not-done (REQ-cli-021)

The Action chain in `src/stage.rs` (rules in order, first wins) becomes:

1. schema 1: migrate. 2. paused: resume. 3. awaiting human: name the gates.
4. blocking bugs at test or accept: back to build. 5. stage done: `done_action`
(unchanged).
6. **New:** stage is intent, design, plan, build, test or accept, and the native
   state read once for this view is archived with terminal evidence that the done
   gate accepts (`gate_for_state(root, session, "done", state)` allowed, the same test
   `done_action` uses). Action:
   - text: `Native change <id> is archived while the checkpoint is at <stage>, so this
     cycle's work is closed. <steps>` where `<steps>` is exactly `done_action`'s
     `Start a new cycle: chough stage reset --to intent --reason "<why>" (or --to plan
     when the approved intent already covers the new work), then specsync change new
     "<outcome>" --spec <module> --path <path>, then chough checkpoint --native-change
     <id> --next-action "<next step>".` At accept the text begins `Record done first:
     chough stage advance (accept to done is allowed). Then: ` because accept.md's
     documented close is advance after finalize.
   - reason: `The archive's terminal evidence is <validity>; continuing <stage> would
     work on a closed change.` plus `Native next action, check it first: <text>.` when
     status names one.
   - command: `chough stage advance` at accept, otherwise `NEW_CYCLE_RESET`.
   The step strings are factored out of `done_action` into constants shared by both
   branches, so the wording cannot drift.
7. gate not open: label and detail (unchanged; now unreachable for an authenticated
   archive at intent to accept, which previously gave `refused:native-archived` at
   plan).
8. otherwise `Continue <stage>: <next action>` (unchanged).

Release and side stages are excluded: release legitimately runs after archive, and a
side stage returns to its caller first. An archive whose terminal evidence does not
authenticate falls through to rule 7 or 8 as today. `chough status` gets the same action
because both views already share the function.

## Decisions block

`next` takes `checkpoint.decisions`, iterates from the end, takes 5, and parses each
with the convention above into `{kind, text, reason}`; text and reason each go through
the existing `cut` (512 bytes, visible marker). JSON: `"decisions": {"total": N,
"items": [...]}` with `null` for an absent kind or reason. Text form:

```text
Decisions (5 of 23, newest first):
  rejected: <approach> (because <reason>)
  decided: lessons live in specs/<module>/context.md (because <reason>)
  2026-10-31: <an unprefixed entry, shown whole>
```

A schema 1 session has no decisions in this view (`total` 0).

## Last failure block

Read the observations in `.chough/evidence/*.json` through the existing bounded record
reader (today `next` only counts unreferenced ones; this adds one parse per record), keep those with `outcome == Failed`, and pick the
one with the greatest `(observed_at_unix_seconds, id)`. Then find bugs whose
`observation_references` list contains its ID, sorted by bug ID, at most 3 with their
total. JSON: `"last_failure": {"id", "observed_at_unix_seconds", "summary", "bugs":
{"total", "items": [{"id", "status"}]}}` or `null`. Text:

```text
Last failure: <observation id> at <unix seconds>: <summary>
  Linked bugs: <BUG id> <status> (1 of 1)
```

No freshness label is computed for it (that would need the snapshot work only
referenced observations get today); the agent runs `chough status` or the case for
freshness. File modification times and directory order are never consulted.

## Lessons pointer

For each module in the checkpoint (at most 16, as `chough context` already enforces):
if `specs/<module>/<module>.3md` is a file, the source is that path with no section
line; otherwise `specs/<module>/context.md`, with `section_line` the 1-based line of
the first line that is exactly `## Lessons`, or `null` with the note `no Lessons
section`; a missing file says `missing`. Only the heading is searched; no lesson text
is copied. Archived bundles: list `.specsync/archive/changes/*/lesson-bundle.md` that
exist, sort by archive directory name descending (names are date-prefixed by SpecSync),
total plus the first 5 paths. No fold state, no ledger. Text:

```text
Lessons: specs/cli/context.md (## Lessons at line 59); 14 archived lesson bundles, newest:
  .specsync/archive/changes/2026-11-01-run-the-specsync-archive-gate-before-every-ci-job/lesson-bundle.md
  ...
```

## Size and determinism

The new lists join `LISTS` in `src/main.rs`, so `fit` can drop their entries when the
view exceeds 16384 bytes and set `truncated`; totals are computed before cutting. All
orderings come from recorded fields or names: decisions by checkpoint order,
observations by recorded time then ID, bugs by ID, bundles by directory name. The
directory reader sorts names, so output does not depend on file system order. The view
keeps `schema_version` 1; the fields are additive and existing consumers
(`examples/*.sh` read `.next_valid_action.action`) are unaffected.

## Skill wording

`skills/chough-workflow/`:

- `references/accept.md` Compounding: lessons carry `observation`, `hypothesis` or
  `verified`, with source (observation, bug or commit) and limits; a project decision
  (checkpoint decision, `docs/DECISIONS.md` entry, approved contract) always outranks a
  lesson and a lesson never rewrites it; a `hypothesis` is a prompt to check, not a
  rule. The "proposed upstream" sentence becomes: SpecSync 6.1.2 ships
  `verification_ignored_paths`; Chough's own repository has not adopted it, because
  entries are directory prefixes and covering `.chough/session.json` would also hide
  `.chough/cases/`, so a Chough write in that window still forces another check unless
  a project lists only safe directories.
- `references/lifecycle.md`: the same replacement.
- `references/checkpoint.md`: the decision prefix convention and the three new blocks
  in "Resume cold".
- `references/bugs.md`: a failing check fixed in the same pull request still gets a
  bug record (with its failed and passed observations) or a lesson entry.
- `references/librarian.md`: a "Closeout" section. One quiet skill pass inside the
  open native change, before `specsync change check <id> --commit` and finalize:
  write supported lessons from this change into the module lesson source with label,
  source and limits; check that new decisions carry a reason and verified bugs link
  their observations; add one `completed` checkpoint entry; print one closeout summary
  of at most 12 lines. It never edits specs after archive; a lesson found later goes
  into a new native change. It works with the librarian off; a library brief is
  written only when it is on. It is a skill step, not a command or gate.
- `SKILL.md`: one line in Working rules pointing to closeout and the labels, only if
  the stage table does not already lead there.

Edited bundle files get their fb6e487 digests in `KNOWN_PREVIOUS` (`src/skill.rs`), a
`tests/fixtures/skill-fb6e487/` copy and a known-outdated test in `tests/package.rs`,
following the ee6e3d2 and 2756dc2 precedents. Host copies are refreshed with `chough
skill update`.

## Cold-start protocol

`docs/exercises/handoff-cold-start/`: `seed.sh` builds a disposable Git repository
(local identity, `GIT_CONFIG_GLOBAL=/dev/null`) from static files only, so the
fixture is the same whichever binary later reads it; the binary under test installs
the skill with `chough init --host openai` (Kimi reads `.agents/skills`) or the host's
mapping for Codex. `rubric.md` lists the pass items and the disqualifiers from the
acceptance criteria. Both are hashed into one observation before implementation; the
baseline uses the main binary and skill at fb6e487. Transcripts and command logs are
observation artifacts; 0xLeif's grade is his statement recorded as a checkpoint
decision with his words, not an approval record. A single run is a sample of one; the
write-up says so.

## Deferred

Not in this slice: cross-project lesson reuse; representing independent workstreams
or multiple checkpoint writers; a readiness gate at accept (blocking on current
passing evidence); a snapshot dependency manifest for `library snapshot`; adopting
`verification_ignored_paths` and relocating the checkpoint into a subdirectory;
correction observations for the 7 artifact hash mismatches in the Kimi observations;
fix-commit and corrected-case fields on bug records (after PR #26 settles REQ-cli-012);
BUG-0017; BUG-0020; the ROADMAP Kimi row, moving this repository's old cycle records
and other housekeeping (a separate pull request).
