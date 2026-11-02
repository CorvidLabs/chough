# Lesson bundle: fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fresh-agent handoff from existing records: chough next shows recent decisions, the last failure and a lessons pointer, names a new cycle after the native change is archived, and the skill labels lesson certainty and closes out inside the open change
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/main.rs, src/stage.rs, src/evidence.rs, src/bugs.rs, src/records.rs, src/skill.rs, tests/next.rs, tests/common/, tests/package.rs, tests/fixtures/skill-fb6e487/, skills/chough-workflow/, .agents/skills/chough-workflow/, .claude/skills/chough-workflow/, specs/cli/, docs/cli.md, docs/exercises/, docs/ROADMAP.md, CHANGELOG.md
- **Acceptance**: Archived-not-done guidance: when the checkpoint stage is intent, design, plan, build, test or accept, active, with no awaiting-human gate and (at test or accept) no open acceptance-blocking bug, and the recorded native change is archived with authenticated terminal evidence, `chough next` (text and `--json`) and `chough status` no longer say `Continue <stage>` and no longer give the build gate refusal as the action. The next valid action says that native change <id> is archived while the checkpoint is at <stage>, then names the new cycle in done_action's wording: `Start a new cycle: chough stage reset --to intent --reason "<why>" (or --to plan when the approved intent already covers the new work), then specsync change new "<outcome>" --spec <module> --path <path>, then chough checkpoint --native-change <id> --next-action "<next step>".` At accept it first names `chough stage advance` to record done, the documented close after finalize, and its command is that advance; at the other stages its command is the reset with `--reason`. A native next action, when status reports one, is shown as the thing to check first. Paused stages, awaiting-human gates, blocking bugs at test or accept, stage done, release, side stages and an archive whose terminal evidence does not authenticate keep today's behavior. Tests in tests/next.rs drive plan, test and accept fixtures with an archived stub change, check the text, JSON and status agree, and run the suggested reset with its placeholders filled, which succeeds (FLOW-13, LIFE-3, REQ-cli-021).
- **Acceptance**: Decisions block: `chough next` shows the total number of checkpoint decisions and up to the 5 most recent (the last 5 entries of `decisions`, newest first). An entry that begins with `decided:`, `rejected:` or `failed:` is shown with that kind; when its text contains ` because `, the part after the last ` because ` is shown as its reason and the part before it as its text. Any other entry is shown whole with no kind and no reason (the latest-N fallback). The checkpoint schema and the stored strings are unchanged. Each text and reason is cut to at most 512 bytes with the existing visible cut marker. JSON carries `decisions: {total, items: [{kind, text, reason}]}` with null for an absent kind or reason. A test with 7 decisions mixing the three prefixes, an unprefixed entry, a dated entry and a 4 KiB entry shows exactly the last 5 in order, with the stated kinds, reasons and cut (LIFE-3, FLOW-13, REQ-cli-021).
- **Acceptance**: Last failure block: `chough next` names the observation in `.chough/evidence/` whose outcome is failed (the Outcome in src/evidence.rs) with the greatest `observed_at_unix_seconds`, ties broken by the greater observation ID, with its ID, observed time and summary cut to 512 bytes, plus every bug that links it (by its observations, note observations or regression baseline) with that bug's ID and status, at most 3 with their total; with no failed observation it says none. The choice uses recorded times only, never file modification times or directory order. JSON carries `last_failure: {id, observed_at_unix_seconds, summary, bugs: {total, items: [{id, status}]}}` or null. A test seeds failed observations whose recorded times disagree with their file names and modification times: the latest recorded one is shown, its linked bug's status is shown, and an unlinked failure shows no bugs (LIFE-3, FLOW-13, REQ-cli-021).
- **Acceptance**: Lessons pointer: for each module in the checkpoint, at most 10 in checkpoint order with their total, `chough next` names the lesson source SpecSync briefs, `specs/<module>/<module>.3md` when present, otherwise `specs/<module>/context.md` with the line number of its `## Lessons` heading, or says that the file has no Lessons section or is missing. It reads and prints no lesson text. It counts the `.specsync/archive/changes/*/lesson-bundle.md` files and lists up to 5 of their paths, newest first by archive directory name. It records no folded or unfolded state and adds no ledger, index or store. Chough creates no .3md; the .3md branch only follows the existing canonical REQ-cli-002 rule, so the pointer names what SpecSync briefs. JSON carries `lessons: {sources: {total, items: [{module, path, section_line}]}, archived_bundles: {total, items}}`. Tests cover a context.md with a Lessons section, one without, a present .3md and 7 archived bundles (LIFE-1, FLOW-12, REQ-cli-021, REQ-cli-002).
- **Acceptance**: Bounded and deterministic: the new blocks count against the existing 16384-byte cap of each form; when the view would exceed it, decision, lesson source, archived-bundle and linked-bug entries are dropped like other list entries, `truncated` becomes true and every total stays correct. A fixture with 50 decisions of 4 KiB each, 200 observations of which 100 failed, 40 bugs linking the latest failure, 40 archived bundles, 16 modules in the checkpoint, and the existing 500 referenced stale observations, 40 awaiting-human gates and 40 blockers of 4 KiB each keeps both forms within 16384 bytes with correct totals. Two runs over the same records, and a run after every record's file modification time is changed, give byte-identical text and byte-identical JSON. `chough next` still makes at most one native status read, the view keeps schema_version 1 with the new fields added, and on this repository the pull request reports both forms' byte counts (LIFE-3, REQ-cli-021).
- **Acceptance**: Skill guidance: in skills/chough-workflow, references/accept.md (Compounding) says each lesson carries a certainty label, `observation`, `hypothesis` or `verified`, with its source (an observation, bug or commit) and its limits, and that a project decision (a checkpoint decision, a docs/DECISIONS.md entry or an approved contract) always outranks a lesson, which never changes the decision. references/checkpoint.md documents the `decided:`, `rejected:` and `failed:` prefixes with a `because` reason as a convention read best-effort, and what the new next blocks show. references/bugs.md says a failing check that is fixed in the same pull request still gets a bug record or a lesson entry. references/librarian.md describes closeout as one quiet skill pass inside the open native change, before `specsync change check <id> --commit` and finalize: it writes supported lessons from the change into the module lesson source with their labels, checks that new decisions carry a reason and that verified bugs link their observations, adds one `completed` checkpoint entry, and ends with one closeout summary of at most 12 lines; it never edits specs after archive (a lesson found later goes into a new native change), it works with the librarian preference off (REQ-cli-015), and it writes a library brief only when the librarian is on. references/lifecycle.md and references/accept.md replace "the proposed upstream `verification_ignored_paths`" with wording that SpecSync 6.1.2 ships it, that Chough's own repository has not adopted it, and why (entries are directory prefixes, so covering `.chough/session.json` would also hide `.chough/cases/`). No other file gets the housekeeping fix in this change. The agent-assisted case reads each statement at the implementation tip (FLOW-12, LIFE-1, REQ-cli-002, REQ-cli-015).
- **Acceptance**: Installed copies stay current: .agents/skills/chough-workflow and .claude/skills/chough-workflow are byte-identical to the canonical bundle; KNOWN_PREVIOUS in src/skill.rs gains the fb6e487 digest of every bundle file this change edits; tests/fixtures/skill-fb6e487/ holds those fb6e487 bytes and a test in tests/package.rs shows an unedited fb6e487 copy reads known-outdated and is updated while an edited copy stays edited; no bundle file gains U+2013 or U+2014 (REQ-cli-016, REQ-cli-019).
- **Acceptance**: Contract and docs: the delta MODIFIES REQ-cli-021 (the three handoff blocks, the archived-not-done action and determinism) and REQ-cli-002 (the lesson source pointer, certainty labels in the native source, and a project decision outranking a lesson) only. No requirement is added or removed; REQ-cli-012, bug IDs and the bug schema are untouched; no hi/ file is edited; the checkpoint schema stays 2; no new store, ledger, record kind, command, model client or integration is added. docs/cli.md documents the new blocks, the prefix convention and the archived-not-done action; CHANGELOG.md gets an Unreleased entry; docs/ROADMAP.md changes only to list this slice. Cases TC-cli-0126 to TC-cli-0133 are added to .chough/cases/cli.cases.toml after the scoped `specsync change check` materializes the delta, as status active, each with an observation at the implementation tip; TC-cli-0125 and BUG-0026 are not used.
- **Acceptance**: Cold-start proof, protocol fixed before implementation: before any implementation commit, docs/exercises/handoff-cold-start/ holds a seed script that builds a disposable Git repository from static files, and a written pass/fail rubric (rubric.md). The seeded repository has a goal; two human decisions in the checkpoint, one of them `rejected: <approach> because <reason>`; one failed experiment that mixed a real defect with a case binding error, recorded as a failed observation with a linked bug; a verified fix with a `verified` lesson in its specs/<module>/context.md Lessons section; and an unfinished next task. The SHA-256 digests of the seed script and the rubric are recorded as one Chough observation with both files as artifacts before implementation; if either file changes afterwards, every run made with the old pair is void and the new pair is recorded as a new observation before any new run. After that observation and before the baseline, 0xLeif reads the frozen rubric; his reading is recorded as his statement, not as an approval. Mandatory baseline: a fresh agent process on a host other than the author's, Kimi Code CLI 0.41.0 or Codex CLI 0.153.4, started in the seeded repository with the binary and skill from main at fb6e487, given only the repository, the installed skill and the prompt `Resume this project.`. Change run: the same host, model and prompt, with the binary and skill from the implementation tip. Real-repository run: the same host and prompt once on this repository at the implementation tip. Each run's observation records the exact commit SHA of the binary and skill it used; if `src/` or `skills/chough-workflow/` change after the change run or the real-repository run, that run is void and is rerun at the new tip before grading. Each run's transcript and command log are kept as hashed observation artifacts with the host, model and version. A run passes the rubric only if, before its first edit, its transcript states the goal, cites both decisions and the reason for the rejection, names the failure, its bug status and the lesson, and starts the correct next task, and its command log shows it neither repeats the failed experiment nor proposes the rejected approach. These do not count: the same session or a fresh command process of the authoring agent; any hint beyond the fixed prompt; recovery that needed `git log`, `gh` or docs/ROADMAP.md; simulated actors; a grader feeding answers. 0xLeif grades every run against the frozen rubric and his grade is recorded as his statement, not as an approval. This criterion holds when he grades the change run a pass; the baseline and real-repository results are reported beside it whatever they are, and if the baseline also passes the pull request says the runs show no difference from main (LIFE-3, FLOW-13, LIFE-1).
- **Acceptance**: Verification: the full suite and `fledge lanes run verify` (fmt, lint, test, intent, `specsync check --strict --require-coverage 100`, `specsync change audit`, `chough cases check`, exercise, build) pass locally with hi 0.8.0, fledge 1.8.0 and specsync 6.1.2, and CI passes on Ubuntu, macOS, Windows and MSRV; results are reported per platform and nothing unobserved is claimed.

## Evidence

- Verification commit: `9a230918dd7c2a4a523a58345eae6bd47219e97d`
- Base commit: `fb6e4871a802c55ba273cd9f5a6207f6e42be884`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

0xLeif, 2026-11-01, directed a new cycle: project-local learning and reliable
handoffs. This change is the first slice. Asked the open design questions, he settled
them (answers to design questions, not scope approval of this definition):

- Cold-start proof: freeze a seeded fixture repository and a written pass/fail rubric
  before implementation and hash them as an observation; run a fresh agent on a
  different host (Kimi Code CLI 0.41.0 or Codex CLI 0.153.4) on main as a mandatory
  baseline, then on the change; 0xLeif grades; plus one run on this real repository.
- Lessons live in the existing `specs/<module>/context.md` Lessons section, which
  SpecSync already briefs. Chough only points at them. No `.3md`, no `library/` store.
- Assumption to state in the design: decisions stay `Vec<String>` in the checkpoint; a
  prefix convention is read best-effort, with a latest-N fallback; no checkpoint
  schema change.

What is true at fb6e487 (origin/main), checked in this worktree:

- The checkpoint was still at stage `test` for the archived 0.1.0 change, and
  `chough next` said `Continue test: specsync change check --commit, push, CI after
  archive, then 0xLeif's implementation review` while native status read archived with
  authenticated terminal evidence and named `merge the PR on GitHub if it is still
  open`. The Action chain in `src/stage.rs` (around lines 783 to 797) reaches
  `done_action` only at stage done; at test or accept the archived change opens the
  gate, so the final `Continue <stage>` branch wins. The reset that opened this cycle
  (`chough stage reset --to plan`, transition at fb6e487) moved this repository off
  that state; the defect remains for any checkpoint left behind an archive.
- `chough next` (REQ-cli-021, `src/main.rs` `next`) shows the objective, stage, native
  summary, next action, gate, human gates, blockers, blocking bugs and stale
  observations, within 16384 bytes. It shows no decisions, no failure summary and no
  lesson pointer; those are only in `chough context` (69,721 bytes here) and `chough
  status` (515,382 bytes here).
- Decisions are free text (`SessionV1.decisions: Vec<String>`, `deny_unknown_fields`,
  `src/checkpoint.rs`); REQ-cli-007 requires an explicit migration for any schema
  change. Observations carry `outcome` (`Outcome` in `src/evidence.rs`) and
  `observed_at_unix_seconds`; bugs link observations by their observations, note
  observations and regression baseline (`bugs::observation_references`).
- SpecSync 6.1.2 briefs lessons from `specs/<module>/context.md` (or a parsed
  `<module>.3md`) on `change new`, `show` and `status`. It writes a `lesson-bundle.md`
  into each archive but never surfaces those bundles and keeps no fold marker; this
  repository has 14 of them. `chough context` already prints `<module>.3md` when present
  and `context.md` otherwise (`src/context.rs`).
- Skill guidance: `references/accept.md` Compounding already allows lessons at any
  stage in the native source, with no certainty label and no rule that a decision
  outranks a lesson. The librarian (`references/librarian.md`) is off by default
  (`src/prefs.rs`), and REQ-cli-015 requires checkpoints and resume to work without
  it. A CRLF test failure fixed inside PR #22 left no bug or lesson in `.chough/`.
  `references/lifecycle.md` and `accept.md` still call `verification_ignored_paths`
  "proposed upstream", though SpecSync 6.1.2 ships it.
- The canonical REQ-cli-002 heading carries a title after an em dash (`Focus context
  for resumption`); SpecSync applies a MODIFIED item only to an exact `### REQ-cli-002`
  heading (the REQ-cli-004 precedent in PR #17).

Coordination: draft PR #26 (Kimi, collision-resistant bug IDs) owns REQ-cli-012,
TC-cli-0125 and BUG-0026 and edits `.chough/session.json`. This change does not touch
any of those, starts cases at TC-cli-0126, and keeps checkpoint commits apart from code
commits so the conflict surface stays at `.chough/session.json`.

Ruled out for this slice (from the recon critique): a structured decision schema
(needs a migration and collides with #26); folding or tracking unfolded bundles
(needs a ledger SpecSync does not have, which would be a second lesson authority);
editing `context.md` after archive (a contract-companion edit with no native change
behind it); a new requirement (the blocks fit REQ-cli-021 and the lesson rule fits
REQ-cli-002); repository housekeeping (moving the checkpoint to done, the ROADMAP Kimi
row), which goes to a separate pull request.

## From the change's design.md

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

In the checkpoint this assumption is recorded unprefixed and labelled as the agent's,
so the decisions block shows it whole with no kind and it never reads as a human
decision. The first draft recorded it with a `decided:` prefix and ended 0xLeif's two
settled answers with reasons that were the agent's, not his. On 2026-11-01 the three
entries were removed from `.chough/session.json` by hand (the CLI has no flag to
remove a decision; this edit leaves no transition record) and re-added with `chough
checkpoint --decision`: his two answers in his words with no reason, the assumption
unprefixed. A `completed` checkpoint entry records the correction. The agent's
rationale stays here: a fresh command process or the author grading cannot show that
a cold handoff works; a second lesson store would compete with SpecSync; a structured
decision schema would need a checkpoint migration (REQ-cli-007) and could conflict
with PR #26.

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

For each module in the checkpoint (the checkpoint allows at most 16; the view lists at most 10 in checkpoint order, with their total, like every other list):
if `specs/<module>/<module>.3md` is a file, the source is that path with no section
line; otherwise `specs/<module>/context.md`, with `section_line` the 1-based line of
the first line that is exactly `## Lessons`, or `null` with the note `no Lessons
section`; a missing file says `missing`. Only the heading is searched; no lesson text
is copied. Archived bundles: list `.specsync/archive/changes/*/lesson-bundle.md` that
exist, sort by archive directory name descending (names are date-prefixed by SpecSync),
total plus the first 5 paths. No fold state, no ledger.

Chough creates no `.3md`, following 0xLeif's settled answer (lessons live in the
module `context.md` Lessons section; no `.3md`, no `library/` store). The `.3md`
branch only follows the existing canonical REQ-cli-002 rule and SpecSync's briefing,
which reads `<module>.3md` in place of `context.md` when one exists, so the pointer
always names what SpecSync briefs. This repository has none. Reading "No .3md" as
"Chough creates none" rather than "never point at one" is the drafter's
interpretation, for 0xLeif to confirm or correct at scope approval. Text:

```text
Lessons: specs/cli/context.md (## Lessons at line 59); 14 archived lesson bundles, newest:
  .specsync/archive/changes/2026-11-01-run-the-specsync-archive-gate-before-every-ci-job/lesson-bundle.md
  ...
```

## Size and determinism

The new lists (decision items, lesson sources, archived bundle paths and the last
failure's linked bugs) join `LISTS` in `src/main.rs`, so `fit` can drop their entries when the
view exceeds 16384 bytes and set `truncated`; totals are computed before cutting. All
orderings come from recorded fields or names: decisions and modules by checkpoint order,
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
acceptance criteria. Both are hashed into one observation before implementation; 0xLeif reads
the frozen rubric before the baseline (his statement, not an approval); the
baseline uses the main binary and skill at fb6e487. Transcripts and command logs are
observation artifacts; 0xLeif's grade is his statement recorded as a checkpoint
decision with his words, not an approval record. A single run is a sample of one; the
write-up says so. Each run's observation records the exact commit SHA of the binary
and skill it used; a change to `src/` or `skills/chough-workflow/` after the change
run or the real-repository run voids that run, which is rerun at the new tip before
grading.

## Deferred

Not in this slice: cross-project lesson reuse; representing independent workstreams
or multiple checkpoint writers; a readiness gate at accept (blocking on current
passing evidence); a snapshot dependency manifest for `library snapshot`; adopting
`verification_ignored_paths` and relocating the checkpoint into a subdirectory;
correction observations for the 7 artifact hash mismatches in the Kimi observations;
fix-commit and corrected-case fields on bug records (after PR #26 settles REQ-cli-012);
BUG-0017; BUG-0020; the ROADMAP Kimi row, moving this repository's old cycle records
and other housekeeping (a separate pull request).

## From the change's testing.md

# Testing

## Cases

Case IDs TC-cli-0126 to TC-cli-0133 are reserved for this change (TC-cli-0125 is
claimed by PR #26). The case profile allows only `active` or `retired`, so they are not
added to `.chough/cases/cli.cases.toml` as proposed; they are added as `active` after
the scoped `change check` materializes the delta, each with an observation at the
implementation tip. Test names are the planned names.

| Case | Requirement | Criteria | Method | What it shows |
|---|---|---|---|---|
| TC-cli-0126 | REQ-cli-021 | FLOW-13, LIFE-3 | automated, `tests/next.rs::an_archived_change_before_done_names_a_new_cycle_not_continue` | At plan, test and accept with an archived stub change whose terminal evidence authenticates, `next` text, `next --json` and `status` give the new-cycle action (accept names `chough stage advance` first), never `Continue <stage>`; the suggested reset with placeholders filled succeeds; paused, awaiting-human, blocking-bug and unauthenticated-archive fixtures keep today's action |
| TC-cli-0127 | REQ-cli-021 | LIFE-3, FLOW-13 | automated, `tests/next.rs::next_shows_the_five_newest_decisions_with_kind_and_reason` | 7 decisions (`decided:`, `rejected:`, `failed:`, unprefixed, dated, 4 KiB, last with two ` because `) give exactly the last 5 newest first, the stated kinds, the reason after the last ` because `, null kind and reason for unprefixed entries, the 512-byte cut marker, and `total` 7 |
| TC-cli-0128 | REQ-cli-021 | LIFE-3, FLOW-13 | automated, `tests/next.rs::next_names_the_latest_failed_observation_by_recorded_time_with_its_bugs` | Failed observations whose `observed_at_unix_seconds` disagree with their file names and modification times: the latest recorded wins, a tie goes to the greater ID, passed and not-run records are ignored, each linking bug shows ID and status (at most 3 with total), an unlinked failure shows no bugs, no failures shows none |
| TC-cli-0129 | REQ-cli-021, REQ-cli-002 | LIFE-1, FLOW-12 | automated, `tests/next.rs::next_points_to_the_lesson_source_and_counts_archived_bundles` | context.md with `## Lessons` gives its path and line; without the section says so; a missing file says missing; a present `<module>.3md` is preferred; 7 archived bundles give total 7 and the 5 newest paths by directory name; no lesson text appears in either form |
| TC-cli-0130 | REQ-cli-021 | LIFE-3 | automated, `tests/next.rs::handoff_blocks_stay_within_the_cap_and_are_byte_identical` | 50 decisions of 4 KiB, 200 observations (100 failed), 40 bugs linking the latest failure, 40 bundles, 16 modules (10 sources listed, total 16), plus the existing 500 stale observations, 40 gates and 40 blockers of 4 KiB: both forms at most 16384 bytes with correct totals and `truncated` true; two runs and a run after changing every record's modification time are byte-identical; a counting stub shows one native read |
| TC-cli-0131 | REQ-cli-016, REQ-cli-019 | LIFE-1 | automated, `tests/package.rs::a_skill_copy_from_main_before_the_learning_handoff_is_known_outdated_and_updated` | fb6e487 bytes of each edited bundle file read known-outdated and update to current; an edited copy stays edited; the existing no-dash test passes |
| TC-cli-0132 | REQ-cli-002, REQ-cli-015 | FLOW-12, LIFE-1 | agent-assisted, reviewer-note | At the implementation tip each skill statement in the acceptance criteria is present: certainty labels with source and limits; decision outranks lesson; same-PR failure gets a bug or lesson; closeout inside the open change before `check --commit`, at most 12 summary lines, never edits specs after archive, works with the librarian off; the `verification_ignored_paths` wording in lifecycle.md and accept.md; `chough skill status` reports both host copies current |
| TC-cli-0133 | REQ-cli-021 | FLOW-13, LIFE-3, LIFE-1 | manual, transcript | The cold-start protocol below, graded by 0xLeif |

TC-cli-0131 and the REQ-cli-015 part of TC-cli-0132 are regression cases on
requirements this delta does not change (REQ-cli-016, REQ-cli-019, REQ-cli-015). They
are added with the delta cases, after the scoped check, for traceability only.

## Cold-start protocol (TC-cli-0133)

Frozen before implementation: `docs/exercises/handoff-cold-start/seed.sh` and
`rubric.md`, bound by SHA-256 in one observation. A change to either voids every
earlier run and needs a new frozen pair and observation. After the observation and
before the baseline, 0xLeif reads the frozen rubric; his reading is recorded as his
statement, not as an approval.

Setup the seed builds: a goal; two human decisions in the checkpoint, one
`rejected: <approach> because <reason>`; one failed experiment mixing a real defect
with a case binding error (failed observation, linked bug); a verified fix with a
`verified` lesson in `specs/<module>/context.md` Lessons; an unfinished next task.

Runs, each a fresh agent process on a host other than the author's (Kimi Code CLI
0.41.0, else Codex CLI 0.153.4), given only the repository, the installed skill and
the prompt `Resume this project.`:

1. Baseline (mandatory, before implementation): binary and skill from main at
   fb6e487.
2. Change: the same host, model and prompt; binary and skill from the implementation
   tip.
3. Real repository: the same host and prompt once on this repository at the
   implementation tip.

Each run's transcript and command log are observation artifacts with host, model and
version, and the observation records the exact commit SHA of the binary and skill the
run used. If `src/` or `skills/chough-workflow/` change after the change run or the
real-repository run, that run is void and is rerun at the new tip before grading. Pass, per the rubric: before its first edit the transcript states the goal,
cites both decisions and the reason for the rejection, names the failure, its bug
status and the lesson, and starts the correct next task; the command log shows no
repeat of the failed experiment and no proposal of the rejected approach.

Does not count: the same session or a fresh command process of the authoring agent;
any hint beyond the fixed prompt; recovery that needed `git log`, `gh` or
docs/ROADMAP.md; simulated actors; a grader feeding answers.

0xLeif grades every run against the frozen rubric; his grade is recorded as his
statement, not as an approval. The criterion holds when he grades the change run a
pass; the baseline and real-repository results are reported beside it whatever they
are, and a baseline pass is reported as no demonstrated difference from main. Each run
is a sample of one.

## Regression

Existing `tests/next.rs` (TC-cli-0120, TC-cli-0121, TC-cli-0122), `tests/stages.rs`
and `tests/package.rs` cases keep passing; the done-stage resume wording is unchanged.
Full `cargo test --locked`, `fledge lanes run verify` with hi 0.8.0, fledge 1.8.0 and
specsync 6.1.2 locally, and CI on Ubuntu, macOS, Windows and MSRV. The pull request
reports `chough next` and `chough next --json` byte counts on this repository.

## Where these lessons go

- `specs/cli/context.md`
