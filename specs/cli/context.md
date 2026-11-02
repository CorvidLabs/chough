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

Add further lessons only from observed work, with source, applicable scope, concrete
future use and revision or retirement guidance. A future native cli.3md would become
the lesson source in place of this section.
