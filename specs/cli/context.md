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
  (`chough timing`, REQ-cli-020).
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
