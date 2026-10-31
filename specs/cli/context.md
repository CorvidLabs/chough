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
history.

## Working files

- src/main.rs: private binary implementation.
- tests/: isolated integration fixtures.
- skills/chough-workflow/: canonical portable skill bundle embedded in the binary.
- src/records.rs, checkpoint.rs, stage.rs, cases.rs, evidence.rs, bugs.rs, prefs.rs,
  validate.rs, library.rs, skill.rs, context.rs: modules behind the commands.
- .chough/session.json: the single checkpoint (schema 2); session.v1.json keeps the
  migrated schema 1 bytes.
- .chough/cases/, evidence/, bugs/, transitions.jsonl, preferences.json: case
  definitions, observations, local bugs, history and standing preferences.
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

Add further lessons only from observed work, with source, applicable scope, concrete
future use and revision or retirement guidance. A future native cli.3md would become
the lesson source in place of this section.
