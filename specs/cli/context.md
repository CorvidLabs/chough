# CLI context

## Decisions

2026-10-31: The human selected Chough, a private CorvidLabs repository, Rust helper
plus portable skills, and current-agent-first v1. Rune coordination is deferred.
HI captures intent; SpecSync owns module contracts and lifecycle; Fledge owns task
execution. The helper records observations and focused session context only.

Bootstrap implementation and initial private push are explicitly authorized. Native
SDD is optional by upstream policy and remains disabled for this baseline. Structural
SpecSync dogfooding is strict. No scope approval, human implementation review,
archive, merge, release or deployment is implied by the bootstrap.

2026-10-31: The human approved native change
evolve-chough-into-a-resumable-stage-workflow (FLOW-1 to FLOW-13, REQ-cli-007 to
REQ-cli-017), adopting the full lifecycle with SpecSync 6.1.1, the TOML
`chough-testing/1` profile, deferred APS, explicit case execution and capture-only
monitoring. See docs/DECISIONS.md D-008 to D-016. The bootstrap paragraph above is
history. It merged as the squash commit 4008346 (PR #2).

Later native changes, each approved by 0xLeif and archived under
`.specsync/archive/changes/` on its own pull request:

- PR #4: parse native status beyond the display bound and apply checkpoint clears
  first; PR #6 recorded the #2 and #4 merges and cleared their gates.
- PR #3: require native lifecycle closure before Chough pull requests merge (CI).
- PR #7: refuse stage moves whose native approval validity, archive integrity or SDD
  policy does not hold.
- PR #8: bound external commands in time and memory; honor validation stop requests
  after waiting.
- PR #9: first release candidate with reference-aware evidence retention (D-017),
  current guidance and an installable package.
- PR #17: keep evidence current across commits and SpecSync bookkeeping; verify bugs
  only by a pass observed after the failure (D-018).
- PR #15: report how long stages, tests, bug fixes and human decisions took
  (`chough timing`, REQ-reporting-001).
- PR #18: run the SpecSync archive gate before every CI job.
- PR #16: bundle hygiene, case binding environment, colour-safe matching and
  actionable requirement and bug errors.

## Working files

- src/main.rs: private binary implementation.
- tests/: isolated integration fixtures.
- skills/chough-workflow/: canonical portable skill bundle embedded in the binary.
- src/records.rs, checkpoint.rs, stage.rs, cases.rs, evidence.rs, bugs.rs, prefs.rs,
  validate.rs, library.rs, skill.rs, context.rs, timing.rs: modules behind the
  commands.
- .chough/session.json: the single checkpoint (schema 2); session.v1.json keeps the
  migrated schema 1 bytes.
- .chough/cases/, evidence/, artifacts/, bugs/, transitions.jsonl, preferences.json:
  case definitions, observations, captured logs and other artifacts, local bugs,
  history and standing preferences.
- .chough/validation/ and validation.stop: repeated validation runs and a pending
  stop request; .chough/pruned.jsonl: the retention removal ledger.
- examples/ and docs/exercises/: disposable samples and recorded exercises.

## Lessons

2026-10-31, Chough CI setup: the Fledge README's `@v1` Action ref was absent, and
the v1.7.2 release tag existed without action.yml. Source: initial CI job
https://github.com/CorvidLabs/chough/actions/runs/41218532137/job/124588047159
and exact-ref repository reads. Scope: this project's Fledge setup Action. Future
use: keep the reviewed wrapper commit pin separate from its binary version pin;
check the actual Action file and binary assets when upgrading. Revise/retire this
lesson when a supported immutable Action release replaces the bootstrap pin.

2026-10-31, SpecSync 6.1.1 tasks: `change check` refuses while `tasks.md` has
unchecked boxes. Source: disposable dry run of evolve-chough-into-a-resumable-stage-
workflow. Scope: native change tasks in this repository. Future use: list human gates
and post-verification steps (push, CI, review) as plain text, not checkboxes.

2026-10-31, SpecSync 6.1.1 deltas: after approval `change check` refuses an edited
delta, while `change status` kept reporting `approval_valid: true`. Source: rehearsal
clone. Scope: approved changes. Future use: treat the delta as frozen at approval and
rely on `change check`, not status, to detect drift.

2026-10-31, new requirement IDs: REQ IDs added by a delta reach `requirements.md` only
when `change check` materializes them, so case definitions and evidence that link them
validate only afterwards. Source: shipping exercise and Chough's own cases. Future
use: run the scoped check before executing cases for new requirements.

2026-10-31, cargo-test adapter: with no `Cargo.lock`, `cargo test --locked` exits 101
before any test runs, and the first adapter recorded that as `failed`. Source: first
shipping exercise run; fixed in 97410b8. Scope: case execution. Future use: a run
without a test result is `not-run`; ship lockfiles with samples.

2026-10-31, host discovery: Claude Code 2.1.316 discovered `.claude/skills` and not
`.agents/skills`. Source: fresh headless sessions in sample repositories. Scope:
Claude Code. Future use: install with `--host claude` for Claude Code projects;
re-check on new host versions.

2026-10-31, cold-start tooling: the local Codex configuration selected a model that
ChatGPT-account sessions reject; `--ignore-user-config` used the default. Source:
first cold-start attempt. Scope: this machine. Future use: confirm the external
agent can start before relying on a cold-start exercise.

2026-10-31, SpecSync visibility: `pub(crate)` items in a binary's modules are reported
as undocumented exports; `pub(super)` items are not. Source: rehearsal. Scope: this
crate. Future use: D-015; revisit if Chough gains a library target.

2026-10-31, exercise identity: `specsync change check --commit` commits inside the
sample repository, and the CI runner has no global Git identity, so the exercise failed
in hosted CI (run 41232974678) while passing locally. Source: that run's log. Scope:
scripts that let native tools commit. Future use: give disposable repositories a
local identity; test with `GIT_CONFIG_GLOBAL=/dev/null`.

2026-11-01, Chough evidence and SpecSync verification: committing Chough records made
every observation stale (HEAD was hashed into the fingerprint), SpecSync's per-change
bookkeeping writes staled them too, and SpecSync's verification digest covers
`.chough/`, so a cases run, commit, check, review and finalize sequence needed three
re-verifications and a stash. Source: the Raven run, CorvidLabs/raven#644 (reported as
CorvidLabs/chough#10 and #12; BUG-0015, BUG-0016). Scope: any project using Chough
evidence with SpecSync 6.1.1 changes. Future use: freshness compares content without
SpecSync bookkeeping, and the accept order runs `change check --commit` after
committing `.chough/`, with no Chough write before finalize. Also: listing `.chough/`
in a change's affected paths makes every checkpoint or evidence edit stale its
verification, so do not list it; and SpecSync applies a MODIFIED requirement only to
an exact `### REQ-cli-NNN` heading. Retire the ordering part when SpecSync ships an
opt-in verification exclusion (proposed upstream as `verification_ignored_paths`).

2026-11-01, cold-start handoff (observation): Kimi Code CLI 0.41.0 resuming the
frozen csvsum repository with main's `chough next` (fb6e487) recovered the next task
and avoided both recorded mistakes, but read neither `.chough/session.json` nor
`specs/csvsum/context.md`, so it never stated the decisions, the failure or the lesson
before its first edit. Source: observations handoff-cold-start-baseline-kimi-1 and
handoff-cold-start-baseline-kimi-1-grade (0xLeif: P2 and P3 not met). Limits: one run,
one host and model. Future use: put what a fresh agent must know into the `chough
next` output itself; do not rely on an agent opening the files it points to.

2026-11-01, driving Kimi for a cold start (observation): `kimi -p` rejects `--auto`
and `--yolo`, so an unattended run uses interactive `kimi --auto` through
CorvidLabs/rune 0.9.0; a prompt sent before the input box is ready is lost, so wait
until the screen shows `context: 0%`. `kimi export` keeps the full tool-call record.
Source: not-run attempts 0 to 2 and the run script kept with
handoff-cold-start-baseline-kimi-1. Limits: Kimi Code CLI 0.41.0 on this machine.
Future use: reuse that script shape for later runs; re-check on new Kimi versions.

2026-11-01, record bounds in criteria (verified): Chough refuses a checkpoint over
64 KiB, so an approved fixture of 50 decisions plus 80 gates and blockers of 4 KiB
each could not be built, and the tested fixture fills the bound instead. Source:
`tests/next.rs` cap fixture in this change; reported as a deviation by 0xLeif's
choice. Limits: checkpoint-backed lists. Future use: check record-size bounds when
writing a criterion's fixture, before approval.

2026-11-02, checkpoint capacity (agent implementation decision): Leif's qualitative direction was "Do your agent's cleanup and raise the progress file and test-result limits." The later direct instruction was to finish these PRs and merge. Under that standing through-PR authority, root selects the existing proposal of131072checkpoint bytes and400observation pointers as bounded engineering values; these are not human-specified numbers. All other record/list/output/history bounds remain unchanged. PR50cleanup fits the old bounds and is separate. Preserve the original capacity archive and qualify final diagnostic/provenance bytes through the fresh followup; this records no new human Pass grade or numeric quote.

2026-11-01, closeout order (verified): SpecSync 6.1.2 `change check` refuses while the
closeout box is open, so closeout runs before the scoped check and is extended, not
reopened, after later runs. Source: this change's first scoped check. Limits: changes
that list closeout as a checkbox. Future use: write closeout as the last checkbox and
do it just before the scoped check.

2026-11-01, cold-start change run (observation): with this change's `chough next`
(9009d9a), Kimi Code CLI 0.41.0 on the same frozen repository was shown the decisions,
the failure with its bug status and the lesson pointer, and it read `context.md` and
`session.json` before its first edit, yet none of its messages before that edit stated
them; 0xLeif graded it Fail (P1 to P3 not met), with P4 to P6 met. Source:
observations handoff-cold-start-change-kimi-1 and handoff-cold-start-change-kimi-1-grade.
Limits: one run, one host and model; the skill has no step asking an agent to state
the handoff before acting. Future use (hypothesis, untested): add a skill step that
states the goal, binding decisions with reasons, the last failure with its bug status
and the applicable lesson before the first edit, in its own native change, and rerun
the frozen cold start.

2026-11-01, external host quota (observation): the real-repository run stopped after
8 tool calls with Kimi's 429 "5-hour usage limit", after three earlier runs that day.
Source: observation handoff-cold-start-real-repo-kimi-1-attempt-1. Limits: this
account and plan. Future use: check the host's remaining quota before a run and keep
an interrupted run as not-run, never as a graded result.

2026-11-02, resume handoff step shipped unproven (observation): the skill step that
asks an agent to state the handoff before its first edit (change
agents-state-the-recorded-handoff-before-their-first-edit-the-skill-s-resume-steps-ask-a-fresh-agent-to-say-the-goal)
shipped without a cold-start run. The Kimi baseline script was prepared, but the
agent cannot start an auto-approving host and the operator's `!` command arrived as
plain text, so 0xLeif chose to proceed without Kimi. Source: the checkpoint decision
of 2026-11-02 and the package tests for the step. Limits: the step's presence and
wording are tested; its effect on a fresh agent is not. The earlier hypothesis stays
a hypothesis. Future use: run the frozen cold start (baseline at 118a895, change run,
real repository) when a human can start the host, and record the result here; a
pass shows the step is followed, not that more was understood, because the step
mirrors rubric P1 to P3.

2026-11-02, starting external hosts (observation): an agent session cannot start an
auto-approving host such as `kimi --auto`, so every external cold-start run needs a
human to run the prepared script with `!` as the first character of the prompt.
Source: this change's baseline attempt. Limits: this Claude Code permission setup.
Future use: before planning a graded run, confirm who will start it and when.

2026-11-02, case recording order (observation): on Linux with SpecSync 6.1.2, an
unchecked "Record Chough cases" task made `specsync change check` refuse, while
`chough cases check` refused the case until that check materialized its new
requirement, so neither could go first; without the box, the check materialized the
requirement and `cases check` passed. Source: OBS-product-test-11dfd04637cfc41549d8bec2
and BUG-0017 (#11). Limits: a disposable shipping sample, one platform. Future use:
keep running and recording cases out of `tasks.md` and run the scoped check before
cases that link a new requirement, as the plan, testing and accept references now
say. Revisit if SpecSync stops refusing unchecked boxes or Chough resolves links
against approved deltas.

Add further lessons only from observed work, with source, applicable scope, concrete
future use and revision or retirement guidance. A future native cli.3md would become
the lesson source in place of this section.

verified: When another approved branch edits a bundled skill file during a learning change, register that integrated main predecessor as well as the original baseline. The exact identity-main librarian bytes were otherwise classified as edited and skipped. Sources: BUG-identity-main-librarian-reference-is-mistaken-for-a-user-edit-5c40a0c138d1a902783af838, OBS-product-test-bfc159865a39075249fb6e40 and OBS-product-test-b0ea9516761a4e7adcf61406; the named package regression checks both safe update and edited-copy preservation. Limits: observed on macOS with Rust 1.99.1; this is bundle-upgrade evidence, not a fresh-agent cold-start grade.

2026-11-02, exact diagnostic parity (verified): the first new report trace-parity
regression joined error lines and removed a final newline before comparison. A
split-inclusive trace-line filter preserves the exact remaining diagnostic bytes;
all five focused report-read regressions pass. Sources:
OBS-product-test-72b596ad74a5bf0901e6aa92 and
.chough/artifacts/report-reads-focused-after.log. Limits: this is test-harness parity
on local macOS arm64, not human acceptance or native authentication. Future use:
remove diagnostic trace lines without normalizing the underlying output, and keep
capped native timing separate from completed native qualification.

2026-11-02, request-local error reuse (verified): storing a failed structural case load as a formatted string changed its original multiline cause diagnostics. The strengthened loader/report parity regression failed before typed shared error ownership restored identical diagnostics, and all five focused read-context regressions then passed. Sources: BUG-report-case-cache-flattens-existing-loader-error-chains-ba1e469eeb3970b370b6456b, OBS-product-test-553ec9b0ffe80fd46a96145d and OBS-product-test-4e2d19e426e48d18085f0b50. Limits: local macOS arm64; final platform and product qualification is separate. Future use: preserve typed source chains when sharing fallible reads and compare the cached path against its original loader, rather than only comparing trace on/off within the new path.

2026-11-02, shipped workflow predecessors (observation): the new skill-upgrade
regression classified exact unchanged main 825a380 SKILL.md and lifecycle.md copies
as edited until their exact predecessor hashes were registered; after registration,
real updates and edited-copy preservation passed in three disposable projects.
Sources: .chough/artifacts/native-workflow-upgrade-before.log,
.chough/artifacts/native-workflow-upgrade-before-status.json and
.chough/artifacts/native-workflow-upgrade-after.log. The pre-fix content identity was
ec6b39de3f8acd0cba74b5290168f0d5965efdc4c52e20942ffb69a5ff460d06;
its binary hash was not separately retained, so this is a development observation,
not historical native qualification. Future use: register every changed shipped
predecessor and verify unchanged upgrades and edited-copy preservation before
publishing a bundle. This does not establish fresh-agent behavior or a cold-start
result. The earlier exact-predecessor lesson remains applicable.

2026-11-02, user-doc authority consistency (observation): extending the native
writer check beyond the skill reproduced four human-actor examples across the
CLI guide, walkthrough and next-change proposal. Source:
OBS-product-test-4df13417a21256aaa3c5b151 and
.chough/artifacts/native-workflow-user-docs-before/. Future use: check current
writer examples in user documentation as well as installed bundles; preserve
historical proposal status and separate the acting account from the actual human
source. Limits: structural copy consistency, not fresh-agent compliance or human
implementation acceptance. Existing approval history remains unchanged.
2026-11-02, immutable-history cleanup ordering (observation): a development retention fixture caught newer-first deletion hiding the ancestry needed to revalidate older candidates. Source: .chough/artifacts/history-retention-ordering-development.log and tests/history.rs. Limits: the intermediate failed source identity was not retained; the log is not historical native qualification. Future use: remove eligible selected ancestors oldest first, retain immutable receipts, and keep independent/orphan tips out of current timing. Revisit this ordering if the history format changes.

2026-11-02, legacy history request bounds (observation): an independent read-only
bounds review reproduced 5,000 legacy transitions being processed by one timing
request with no partial warning, despite the approved 4,096-event request limit.
Source: OBS-product-test-557ba9d0d6f01009fa59fa6c and
BUG-legacy-timing-bypasses-the-4096-event-request-bound-55a3830ea9566f55e4a6d84a,
at 6fa45db on local macOS arm64. Limits: legacy checkpoints without a history cursor;
the immutable-event traversal was already bounded. Future use: apply shared request
limits to every compatible storage path and test the exact limit and one record
beyond it, including malformed input counted against the read budget.

2026-11-02, frozen reference aliases (verified): structured checkpoint paths were
normalized but frozen artifact text was not. A retained Windows-style cycles
directory reference made three protected fixture cycles eligible for deletion.
Sources: OBS-product-test-063bada1f0797d7f9fc3d83e and
BUG-frozen-artifact-directory-aliases-fail-to-protect-history-cycles-6dfc916446fda2d03ddb49a3,
with exact binary hash and commands under .chough/artifacts/history-frozen-reference-before/.
The failure-first regression now covers four aliases under two quote styles and
asserts byte-identical preview/apply fixtures. Future use: apply reference path
normalization to every protection source, including raw frozen text, and verify
equivalent paths preserve identical protected sets. Limits: synthetic local fixtures;
no live project cleanup or new cold-start proof is implied.

2026-11-02, retention review boundaries (observation): independent peer review
reproduced two gaps at 489ecf50. An idle cycle was aged from its last activity,
so a reset that closed it moments ago did not receive fourteen-day protection.
Separately, a branch added a reference after another branch pruned the target;
Git merged cleanly but history inspection incorrectly claimed completeness.
Sources: OBS-product-test-4d5f09a757e2e50d651abef4 and
OBS-product-test-f59a8fa906c43f699940f33c, with exact source, binary, commands and
fixture inventories under .chough/artifacts/history-idle-closure-before/ and
.chough/artifacts/history-post-prune-reference-before/.
Future use: measure closure from its causal successor event, and inspect merged
references against removal receipts. Do not infer that a clean Git merge restores
proof or that old event activity establishes an old closure. Limits: disposable
local fixtures; no live proof was removed and no automatic restoration is implied.

2026-11-02, directory reference classification (observation): incremental review
reproduced bare and single-quoted cycle-directory references being protected but
omitted from missing-reference diagnostics after a clean Git integration. Source:
OBS-tc-cli-history-post-prune-reference-23f4f23b82735a0a21fc1aab and the exact
clean-tip inputs under .chough/artifacts/history-cycle-directory-references-before/.
Future use: reuse normalized token boundaries and one whole-cycle classification
for both protection and availability; retain a separate exact-file restoration
case alongside all four directory quote forms. Limits: disposable local fixtures;
no live proof was removed and this does not define every external path grammar.

2026-11-02, reference occurrence classification (observation): punctuation and mixed
file/directory references exposed another availability gap after the earlier quote
repair. Source: OBS-tc-cli-history-post-prune-reference-8e61ca8d5022c39e4631183c,
with the original capture and failure-first regressions retained under
.chough/artifacts/history-protected-reference-punctuation-proof/ and
.chough/artifacts/history-reference-classification-followup/. Future use: classify
each normalized cycle occurrence once; only occurrences inside known exact file
paths are file-specific. Check both pre-prune protection and post-prune
availability, including partial restoration. Limits: disposable local fixtures;
no live proof was removed and external path grammars remain outside this proof.

2026-11-03, checkpoint refusal publication order (observation): writer-level
regressions reproduced four orphan gate events after a rejected capacity update,
while the saved checkpoint stayed unchanged. Source:
OBS-product-test-2a7d7facc915bcf96b293641 and
BUG-checkpoint-capacity-refusal-publishes-orphan-gate-history-b53b7f8277c4ff818e09bde1,
with stable tests and unchanged production source frozen in
.chough/artifacts/pr51-writer-atomicity/before/. Future use: validate the complete
serialized candidate before publishing side records, including the 26-byte growth
from a null history head to its first fixed-width event ID. Test every managed
record path and byte, exact-bound success and multiple clear/open gates. Limits:
disposable macOS fixtures; this finding does not establish transactionality for
later I/O errors, ID collisions or cycle-storage capacity failures.

2026-11-03, refusal remedies drift from the actor rule (observation): after
lifecycle.md, plan.md and docs/cli.md moved to the acting-agent actor with a human
authority note, two approved-delta guard refusals in src/stage.rs still told agents
to ask the human and approve under their name, and tests quoted that text. Source:
OBS-tc-cli-delta-guard-refusals-name-the-agent-actor-a56263c10f646c87c5ed5ea0 (failed
at main 8300b58) and its bug. Limits: one search of src/, skills/ and docs/ for this
wording; frozen old-skill fixtures were not changed. Future use: when guidance wording
changes, search runtime refusal and next-action strings in src/ as well as the
skill and docs, and assert the remedy text in a test.

2026-11-03, spec size bounded by Chough readers (verified): `chough evidence add`
read `specs/<module>/requirements.md` through the 65,536-byte structured-record
reader, so a growing cli companion blocked #55 and forced the #59 split, although
SpecSync sets no spec-file cap. The #61 reviews then found `chough next` skipping a
module `context.md` over 4 MiB and `chough library snapshot` refusing files over
4 MiB; REQ-cli-034 named both as known exceptions. #66 lifts `next`'s; `library
snapshot`'s stays named for a later native change. Source: the failing-first runs of
`spec_requirements_companions_are_read_at_any_size` ("structured record exceeds
65536 bytes") and `next_searches_a_context_md_larger_than_4_mib` ("larger than
4194304 bytes, not searched"), and Leif's 2026-11-03 10:05 AM MT decision that spec files have no size
limit. Limits: Chough reads spec files in `evidence add` (`requirements.md`, whole),
`next` (the `context.md` Lessons heading, streamed at any size),
`library snapshot` (4 MiB until the follow-up), `context` (display-budgeted
sections that mark their cut) and evidence snapshots (streamed digests). Future use:
when removing a bound for a kind of file, search every reader of that path, not only
the one that failed; read files under `specs/` with `records::read_spec_file` or a
streamed read, keep `read_record` for Chough's own records, and treat size as a
composability hint, not a gate.

2026-11-04, Chough bounds leaked onto HI and SpecSync files (verified): REQ-cli-034
exempted `specs/` but not HI or SpecSync's own files. `validate_criteria` parsed the
16 KiB display section of `hi export`, so in CorvidLabs/jackdaw PR #41 an agent cut
`hi/board.md` intent from about 1,300 words to 64 and `hi/cost.md` from about 1,250 to
70 to get under it (issue #100). Archived `finalization.json` and `change.md`,
`.specsync/sdd.json` and timing's native records also used the 64 KiB record bound.
Source: the failing-first runs in `.chough/artifacts/issue-100/` ("native HI family
export is invalid or exceeds 16384 bytes", "structured record exceeds 65536 bytes")
and Leif's 2026-11-04 rule, "Chough should only reduce and do all that for its own
files in .chough and not the specs, not the human intent" (LIFE-15, REQ-cli-042,
D-020). Limits: `library snapshot`'s 4 MiB copy limit stays; the earlier spec splits
are not undone. Future use: decide from `records::native_json` or
`records::read_native_text`, never from `native` or `read_record`, for any file Chough
does not own; and when a bound refuses a native source, the fix is in Chough, never in
the source.

2026-11-04, archived status cost dominates the Archive check (verified): one `specsync
change status <archived-id>` read costs 26 to 35 s inside SpecSync 6.1.2, with no Git
subprocesses, and the hosted SpecSync Archive job made one per archive on every PR. It
took about 7 minutes with 13 PR commits, 8 with 26 and 9m51s with 42, and was cancelled
at its 10-minute limit with 91. Source: a profile of
.github/scripts/check-specsync-archive.py on #97's branch (78 archives), the hosted runs
on #101, #92, #77 and #93, and CorvidLabs/spec-sync#831. Future use: never put a
per-archive native status read on a path that runs per PR or per command; #103's four
shards only buy headroom. Limits: Linux aarch64 in Docker and ubuntu-latest; measure
again on SpecSync 6.2.0.

2026-11-04, squash merges and chained PRs (verified): with squash-only merges, a PR that
merged an earlier PR's branch keeps that branch's commits after the earlier PR lands as
one squash commit, so its range against main includes them. The Archive check walks
every commit in that range: 91 on #93, against its own 26. Merging main normally then
conflicts wherever both sides added the same lines. Source: #91, #93, #95 and #97, and
the stacked #90, #94, #96 and #99. Future use: to land PRs in sequence, merge the
earlier PR's final head into the next one; after the earlier one lands, record its
squash commit with `git merge -s ours` only after checking that the squash tree equals
that head's tree. Otherwise merge main after each landing. Limits: this repository's
squash-only, linear-history settings.

2026-11-04, a merge seam drops a shared closing brace (observation): when two branches
appended tests to the end of the same Rust file, Git kept the closing `);` and `}` that
both functions end with outside the conflict. Keeping both sides then left one function
unclosed ("this file contains an unclosed delimiter"). Source: tests/sessions.rs while
merging #94 into #96 and #96 into #99. Future use: after resolving a conflict where both
sides appended code, read the seam and build before committing. Limits: seen with Git's
default merge on Rust test files.

2026-11-04, merging formerly stacked PRs (observation): for a PR that had been stacked
on another and was retargeted to main, GitHub refused both the synchronous merge
endpoint and `gh pr merge`, saying it "must be merged using the asynchronous merge REST
API". The synchronous endpoint also returned HTTP 500 for a squash whose default message
listed many commits. Source: the #91 and #99 merges. Future use: give an explicit squash
title and message, and use `PUT .../pulls/<n>/merge-async` with
`merge_action=direct_merge` for a formerly stacked PR. Limits: GitHub's behaviour on
2026-11-04.

2026-11-04, tests that replay every archive (verified):
`the_guard_accepts_every_archived_approval_in_this_repository` assumed every archived
change has a cli delta. #96, the first history-only change, passed before it was
archived and failed hosted CI after. Source: CI on #96. Future use: derive each
archive's expectation from its own approved scope; such a test fails only once a new
kind of archive exists, so run the full suite after every finalize. Limits: none known.

2026-11-04, a lifted bound survived in a test (verified): tests/spec_split.rs still
capped every requirements companion at 65,536 bytes after Chough stopped bounding native
files (REQ-cli-034, REQ-cli-042). #92 and #91 together made specs/cli/requirements.md
66,330 bytes and hosted CI failed. Leif chose "Remove the size test" and said the size
cap is only for the files inside the .chough folder. Source: CI on #91. Future use: when
a bound is lifted from the product, search the tests and CI scripts for the same number.
Limits: none known.

2026-11-04, closeout lessons skipped (observation): of the changes archived on
2026-11-04, only #101 recorded a lesson, although the closeout step in
references/librarian.md asks for one inside the open native change. The agent's own
closeout helper went straight from cases to review and finalize. Source: this section
before this change. Future use: write the closeout lessons before `specsync change check
<id> --commit`, in the order the librarian reference gives. Limits: one agent's runs on
one day.

2026-11-04, parallel reads and count-timed test stubs (verified): once validity reads
ran in parallel, two safety tests in tests/evidence.rs fired their stub hooks at the
wrong time or not at all. Their stub counted calls in a file without a lock. That was
also the one intermittent verify failure on #108. Source: #108, review threads at
src/evidence.rs:2121. Future use: a stub shared by concurrent processes counts under a
lock (`mkdir` is atomic). Limits: shell stubs on Linux and macOS.

2026-11-04, a re-check that forbids any new file refuses safe work (verified): the
per-file re-check before removal required a set to list no file beyond its plan. When
two eligible sets shared an artifact, removing the first left that artifact to the
second, and the whole first trim on this repository was refused and restored (21 files).
Source: #108's first closeout. Future use: a safety re-check asks whether every planned
file still belongs to the set alone, not whether the set gained nothing; only planned
files are ever removed. Limits: the retention apply.

2026-11-04, never read "cannot read" as "missing" (verified): `observed_at` turned a
read or parse error into None, and the closed-cycle drop then removed a current
reference whose record existed but was unreadable. Source: corvid-agent review
5780146055 on #108. Future use: separate absence from error in any helper that feeds a
removal, and keep the reference while refusing the step when a record cannot be read.
Limits: none known.

2026-11-04, hand-parsing another tool's syntax drifts from it (observation): #107 took
five review rounds on the vault's reading of HI and requirements text: sub-criterion
IDs; `Trace:` mentioned inline before and after the list; quoted and block YAML lists,
YAML comments and CRLF; and unmatched backticks. Source: corvid-agent and Leif's review
agent on #107. Future use: reuse the owning tool's grammar (`records::hi_criterion_id`),
parse with `lines()`, and test every form the source tool accepts. Limits: the vault
readers.

2026-11-04, generated files need an upgrade path (verified): a knowledge README that an
earlier Chough generated never gained the new front matter, because it was create-only.
A CRLF checkout of it then failed Windows CI. Source: #107 review 5779587370 and hosted
Windows. Future use: keep the digests of earlier generated bytes, taken over
LF-normalized text, and replace only an exact earlier copy, as the skill's
KNOWN_PREVIOUS does. Limits: generated files Chough owns.

2026-11-04, a staged conflict marker (observation): merging #108 into this change's
branch staged `tests/reduce.rs` with unresolved markers, because the commit command ran
after a resolver step that had stopped. Source: commit 63f0605, fixed in 05a59f5. Future
use: before committing a merge, check that `git diff --name-only --diff-filter=U` is
empty, and chain the steps with `&&`. Limits: an agent's shell habits.

2026-11-04, the first wide trim (verified): on this repository #108's closeout trimmed
367 observation sets (4.6 MB) in 327 s. `.chough/` went from 32 to 26 MB and evidence
records from 868 to 501. Dropping 385 closed-cycle checkpoint references (KNOW-16) is
what made them eligible. Source: #108's closeout commit. Future use: expect the first
trim of an old repository to take minutes, and later ones only seconds. Limits: one
repository.

2026-11-05, generated paths must check out on Windows (verified): #109's rerun trim
wrote vault attachments up to 246 characters long under full archive IDs, and Product /
windows-latest failed to check the branch out ("Filename too long"). Source: hosted job
127268393117, fixed in 5998879. Future use: keep every path Chough writes into a tracked
tree short. Name folders with a readable prefix and a digest, and refuse to write past
220 characters. Limits: Windows checkouts without `core.longpaths`.

2026-11-05, a move is create-only and keeps the mode (verified): the trim's vault copies
used `fs::write`, which truncated an attachment that was already there and dropped the
executable bit. Its restore used create-new without the mode, so two kimi scripts came
back as 0644. Source: corvid-agent's review of #109 at 6e0a835 and a review thread, fixed
in 5998879 and 05d8244. Future use: copy with create-new, set the source's permissions on
the copy and on a restore, and treat as yours, and roll back, only what you created.
Limits: permissions, not owners or times.

2026-11-05, a folder reference names what is inside it (verified): a bug fixture or a
checkpoint reference that named a folder protected nothing, because protections matched
exact file keys only. An open bug lost its reproduction log to the trim. Source: a #109
review thread, fixed in 05d8244. Future use: match a reference against each ancestor of
a path, at `/` boundaries, on case-folded canonical keys, so `a/b` covers `a/b/c` but
not `a/bc`. Limits: references that name paths.

2026-11-05, judge a group's members one by one (verified): one cold-listed file made a
whole loose folder cold, including a file first committed after the newest archive. A
mid-apply re-check of the whole group refused the closeout trim, because an earlier
removal had released a file into that folder; it restored 172 files. Source: a #109
review thread and the refused closeout at 76fb917. Future use: decide ownership per
file, keep a mixed folder whole, and re-check only the files planned for removal.
Limits: loose artifact groups.

2026-11-05, CI runners have no Git identity (verified): a test helper ran `git commit`
with only dates set. It passed locally, where it borrowed the developer's identity, and
failed on the MSRV and Fledge jobs with "empty ident name". Source: hosted jobs
127268393092 and 127268393085, fixed in b4943f3. Future use: pass `-c user.name`,
`-c user.email` and `-c commit.gpgsign=false` on every test Git call, and run the suite
once with `GIT_CONFIG_GLOBAL=/dev/null`. Limits: test fixtures.

2026-11-05, most merges conflicted (verified): replaying the last 89 merges with `git
merge-tree` found 83 conflicted, most often on the one checkpoint (69) and the one
transition log (60). Source: the replay for #111. Future use: give each parallel writer
its own file (sessions, events, a cases file per change), union-merge append-only files,
rebuild generated files and check them in CI, and give structured records a merge
driver. Limits: SpecSync's own files (CorvidLabs/spec-sync#862).

2026-11-05, Windows checks text out with CRLF (verified): a test that read
`specs/cli/requirements.md`, replaced `\n` with spaces and split sentences on ". " passed on
Linux and macOS, then failed on `Product / windows-latest`. There, sentences ending ".\r"
never split. Source: hosted run 42153579452 on #131, fixed by the CRLF repair change.
Future use: normalize `\r\n` to `\n` before parsing any repository text in a test, and
check a CRLF copy too. Limits: tests that read checked-out text.

2026-11-05, GitHub cannot use the union merge drivers (verified): a branch that merged
cleanly locally, through `merge=union` on CHANGELOG.md and the generated vault notes,
showed as conflicting on GitHub. corvid-agent requested changes, and CI did not run on the
merge ref. Source: #130 at 3f8b85fd. Future use: whenever main moves, merge main into the
branch locally before asking for review or a merge, then rebuild the vault. Limits: PRs.
