# Lesson bundle — evolve-chough-into-a-resumable-stage-workflow

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Evolve Chough into a resumable stage workflow
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/, tests/, skills/, .agents/skills/, .claude/skills/, examples/, docs/, hi/, INTENT.md, README.md, AGENTS.md, CLAUDE.md, .chough/, library/, .github/workflows/ci.yml, fledge.toml, Cargo.toml, Cargo.lock, .specsync/sdd.json, .gitignore
- **Acceptance**: A fresh host finds the short router; natural language, a supported skill and the CLI reach the same .chough checkpoint and native records, with no mandatory provider API or runtime, and role reviews also work as sequential passes without subagents (FLOW-1, REQ-cli-016).
- **Acceptance**: An isolated intent exercise preserves original wording and stable HI IDs, derives a scoped FRD, attaches role findings to concrete HI/FRD references and ends with one synthesized intent-stage decision rather than per-criterion approvals (FLOW-3).
- **Acceptance**: A nonvisual sample records a design skip with its reason; a visual sample has one canonical local HTML/CSS prototype with companion mappings, relevant states and viewports, and recorded rendered-review observations, while its simulated human design approval stays labelled pending or simulated (FLOW-4).
- **Acceptance**: This change itself runs through the adopted native SpecSync lifecycle with one human scope approval before implementation; Chough introduces no approval ledger and native status decides eligibility (FLOW-5, REQ-cli-008).
- **Acceptance**: Stage iterate, back, reset, pause, resume, skip-with-reason, side-stage enter and return work and are covered by tests; history is append-only, back/reset name the dependent work to refresh, and a saved stage cannot enter build or close acceptance when native state does not allow it (FLOW-2, REQ-cli-007, REQ-cli-008).
- **Acceptance**: Focused selection returns cases linked to the affected criteria, requirements or paths, always includes their functional cases, adds copy-localization and design-QA cases only when those tracks are relevant, and lists omitted cases with a reason (FLOW-6, REQ-cli-010).
- **Acceptance**: Test definitions in chough-testing/1 TOML are validated separately from observations; case IDs and definition digests survive reordering; an adapter run passes only on exit 0 plus its declared assertion (exactly one cargo test run, or expected output), zero matched tests is not-run, and a real failing case stays failed when others pass (FLOW-7, REQ-cli-009, REQ-cli-011, REQ-cli-017).
- **Acceptance**: A sample defect is reproduced by a failing observation, classified, deduplicated against the local backlog and persisted with evidence; a blocking bug returns work to build and focused retest; an out-of-scope bug stays visible without expanding scope; regression needs an earlier passing observation at an ancestor revision; nothing is posted externally (FLOW-8, REQ-cli-012).
- **Acceptance**: Repeated validation runs only under a run consent or a standing preference, never promotes one into the other, honours stop and resumes with the same limits, persists every failure locally across restarts, and labels results as repeated runs rather than soak, live health or repair authority (FLOW-9, REQ-cli-013).
- **Acceptance**: Native human implementation review stays at acceptance before archival; Chough never records it, and scope approval, agent review, command success or an actor label cannot close acceptance; the delivered PR shows review and archive pending (FLOW-10, REQ-cli-008).
- **Acceptance**: Completing acceptance does not enter release; entering release requires a run consent or standing release preference, release observations distinguish publication, installation, deployment, live health and dry runs, and no real release or live job is performed (FLOW-11, REQ-cli-014).
- **Acceptance**: The optional librarian snapshots originals with source path, revision and SHA-256, regenerates an index that keeps open bugs and canonical owners visible, never moves or rewrites sources, and returns to its calling stage (FLOW-12, REQ-cli-015).
- **Acceptance**: With the librarian off, a real fresh agent session given only repository files discovers the checkpoint, checks native state and evidence freshness, and states the same scoped next action, preferences and pending human gates; the transcript and assessment are recorded, or the criterion stays explicitly pending (FLOW-13, REQ-cli-007).
- **Acceptance**: APS 1.1.0 is documented as qualified-and-deferred with observed reasons; .chough/session.json remains the single canonical checkpoint and core resume works without APS.
- **Acceptance**: Existing init preservation, path, schema, bounds, revision, environment and artifact freshness regressions still pass; schema 1 sessions migrate only through an explicit command that preserves the original bytes; the unapproved metadata-commit evidence reuse is not implemented (LIFE-2, LIFE-7, REQ-cli-001 to REQ-cli-006).
- **Acceptance**: Installation, a realistic feature and bug loop and cold resume are documented; updated Mermaid diagrams are rendered and visually inspected; hosted CI is observed at the final branch tip with native review and archive gates reported as they actually stand (LIFE-1, LIFE-4).

## Evidence

- Verification commit: `de9fe45c9ccebfe688829c16a775a099d135892e`
- Base commit: `c582a10e0761e5bb89a10b9f4b8002a2547b21d2`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## What led here

The bootstrap (HEAD d38aa19) delivered `init`, `context`, `status`, `next` and
`evidence add` with native structural checks while SpecSync SDD was disabled. It made
no claim of scope approval, implementation review or archive. A human brief of
2026-10-31 redirected the next work from the observation-reuse proposal in
`docs/NEXT_CHANGE.md`, which was never approved, to a complete resumable development
workflow: stage routing, conditional design, one scope approval, focused three-track
testing, a local bug backlog, optional validation and release, an optional
librarian, and mandatory checkpoints with cold resume. This change is the first use
of the full native lifecycle in this repository.

## What a session picking this up needs to know

- Native lifecycle was adopted in commit c582a10 on `feat/resumable-stage-workflow`;
  SpecSync writers and CI use 6.1.1. Use a `PATH` that selects hi 0.8.0, fledge 1.7.2
  and specsync 6.1.1; the default `PATH` on the authoring machine picks hi 0.5.0 and an
  unreleased fledge 1.8.0.
- The delta is frozen once approved. `change status` may still show
  `approval_valid: true` after a delta edit; `change check` refuses it.
- `.chough/session.json` still points to `docs/NEXT_CHANGE.md`. That pointer is stale
  planning, not authority, and is replaced during increment 7.
- APS and 3md were evaluated and not selected; see research.md. Do not reintroduce
  them without a new decision.
- Human gates pending: scope approval of this definition, then implementation review
  at acceptance. No agent records either.

## Ruled out

Reusing observations after metadata-only commits; APS as checkpoint backend or mirror;
3md as the test-definition source; Magpie generic mode as an oracle; a dashboard UI.

## From the change's design.md

# Design

## Shape

The current agent does the work. Behavior lives mostly in the skill: a short
`SKILL.md` router, one stage reference that every stage shares, progressive per-stage
references and Markdown templates. The Rust helper keeps only deterministic jobs:
the checkpoint, stage transitions checked against native state, test-definition
validation and explicit case execution, bug records, preferences and consent,
librarian snapshots and index, and host skill installation. No model client, runtime,
hosted service, slash-command engine or second lifecycle ledger is added.

## Ownership

| Authority | Owns | Chough's relation |
|---|---|---|
| HI | Human wording, criterion IDs and history | Reads `hi export`; proposes FLOW IDs only after human approval |
| FRD (`docs/FRD.md`) | Derived functional requirements and traceability | Links to native `REQ-*`, never forks them |
| SpecSync | Requirements, contracts, changes, approval, verification, review, archive, lessons | Checkpoint stores the change ID; gates read `change status --json` |
| Fledge and project frameworks | Tasks, lanes and real checks | Case bindings call `cargo test` or `fledge run` |
| Canonical prototype and companion | Approved visual and interaction design | Checkpoint references their paths |
| Chough checkpoint | Stage, scope references, decisions, consents, observations, local bugs, next action | Single writable record |
| Optional librarian | Index, source-linked snapshots, briefs | Never a second current source |
| Environment and evidence | What ran, where, on which inputs | Observation records |

## Records under `.chough/`

| Path | Content | Change |
|---|---|---|
| `session.json` | Canonical checkpoint, schema 2 | Extended; schema 1 read-only until migrated |
| `session.v1.json` | Original schema 1 bytes kept by `migrate` | New, write-once |
| `transitions.jsonl` | Append-only stage transitions | New |
| `preferences.json` | Standing project preferences with source | New |
| `cases/<module>.cases.toml` | Test definitions, profile `chough-testing/1` | New |
| `evidence/<id>.json` | Observations, schema 1 or 2 | Extended |
| `artifacts/` | Captured adapter output | Convention made explicit |
| `bugs/<BUG-id>.json` | Local bug backlog | New |
| `validation/<run>.json` | Stored limits for a repeated run; `validation.stop` request file | New |

The existing fingerprint rule is unchanged: `.chough/` and `target/` stay outside the
worktree fingerprint so recording does not stale observations. Case edits stale their
own observations through the definition digest instead.

### Checkpoint schema 2

```json
{
  "schema_version": 2,
  "objective": "…",
  "intent_scope": "LIFE",
  "modules": ["cli"],
  "stage": {"current": "build", "status": "active", "callers": [], "iteration": 1},
  "native_change": "evolve-chough-into-a-resumable-stage-workflow",
  "references": {"criteria": [], "requirements": [], "cases": [], "design": [], "files": []},
  "decisions": [], "open_questions": [], "completed": [],
  "observations": [], "bugs": [], "blockers": [], "awaiting_human": [],
  "run_consents": [{"key": "repeat-validation", "source": "…", "objective": "…"}],
  "next_action": "…"
}
```

Validation keeps `deny_unknown_fields`, the 64 KiB cap, module-name rules and the
16-module limit, and adds closed stage names and status, at most four nested callers,
valid ID shapes and project-relative reference paths. `chough checkpoint` edits
fields and writes atomically (temporary file plus rename in the same directory after
the existing symlink checks); hand edits stay allowed and are validated on read.
`decisions` remain sentences, so a decision carries its rationale inline. Hidden
reasoning is never stored. `chough migrate [--stage S]` writes `session.v1.json` with
`create_new`, then the schema 2 file; it refuses when the backup exists. Context and
status keep accepting schema 1 and report that migration is needed.

## Stage model

One representation is shared by the skill and the helper. `references/stages.md`
holds a table per stage with purpose, inputs, outputs and owners, roles and tools,
completion evidence, approval and preference policy, consequential questions,
checkpoint content and valid transitions. The helper embeds only stage names, order,
kind and gate rules, and a test asserts both lists match.

| Stage | Kind | Gate checked by the helper |
|---|---|---|
| intent | main | none; the intent decision is human and skill-led |
| design | conditional main | skip requires a reason |
| plan | main | none |
| build | main | entering requires native state past definition approval |
| test | main | none |
| accept | main | leaving requires the native change archived |
| done | terminal | reached only from accept |
| release | optional | entering requires a release run consent or standing preference |
| bugs, validate, librarian | side | caller recorded; validate needs consent or preference |

Transitions: `stage advance`, `stage iterate --note`, `stage back --to S --reason`,
`stage reset [--to S] --reason`, `stage pause --reason`, `stage resume`, `stage skip
--reason` (skips the current stage and advances), `stage enter <side|release>
--reason`, `stage return --note`. Each appends a JSON line with from, to, kind,
reason, revision and time. Back and reset print the later stages and the checkpoint
references that need refresh, and for targets at or before plan they print the native
`specsync change status` command as the authority for reapproval or recovery.
Nothing is deleted.

Native gate reading: the helper runs `specsync change status <id> --json` and reads
`summary.state`. A disposable dry run of this exact definition observed `draft`,
`approved`, `verifying` and, after finalize, `archived` from that field. States
`approved`, `implementing` and `verifying` satisfy the build gate, and only
`archived` satisfies leaving accept. Missing, failed or undecodable status
refuses. When no SpecSync configuration exists the gate reports not-configured and
allows the transition, labelled ungated. `chough next` combines the stage, the native
next action, freshness, open blocking bugs and awaiting-human gates into one report;
the saved `next_action` stays a suggestion beside native authority.

## Testing profile `chough-testing/1`

TOML at `.chough/cases/<module>.cases.toml`, outside `specs/` where SpecSync treats
`.3md` as lessons and outside Magpie's globs. Example:

```toml
profile = "chough-testing/1"
module = "cli"

[[case]]
id = "TC-cli-0001"
title = "Init preserves user files"
status = "active"
track = "functional"
method = "automated"
criteria = ["LIFE-4"]
requirements = ["REQ-cli-001"]
paths = ["src/"]
environments = ["local-macos", "ci-linux"]
artifacts = ["test-log"]
setup = ["Isolated Git fixture"]
steps = ["Run the bound test"]
expected = ["Edited session and skill bytes are unchanged after a second init"]

[case.bind]
adapter = "cargo-test"
test_target = "cli"
test = "init_preserves_user_files_and_installs_a_complete_skill"
```

The helper validates the profile, module stem, `TC-<module>-NNNN` IDs unique across
files, status active or retired, track (functional, copy-localization, design-qa),
method (automated, manual, agent-assisted), active HI criteria, native requirement
IDs, non-empty environments, closed artifact types (test-log, report-json,
screenshot, recording, transcript, reviewer-note), non-empty expected outcomes,
setup and steps for manual and agent-assisted cases, and a binding for automated
cases. `deny_unknown_fields` applies at every level. The digest is SHA-256 of the
canonical JSON of the parsed case, excluding title and notes, so position never
changes identity. Retired cases stay in the file.

Adapters, which only read `bind`:

| Adapter | Executes | Passed only when |
|---|---|---|
| cargo-test | `cargo test --locked --test <target> -- --exact <test>` | exit 0 and the summary reports exactly one test passed; zero is not-run |
| fledge-task | `fledge run <task>` | exit 0 and every `expect_output` fragment appears; at least one fragment is required |
| command | argv as written, no shell | same as fledge-task |

Manual and agent-assisted cases are recorded with `cases record <ID> --verdict
<index>=pass|fail` for every expected outcome plus method, environment, summary and
artifacts; the outcome is passed only when all verdicts pass. Title, setup, steps,
inputs, expected text and notes are never interpreted as an oracle. `cases select
--criterion/--requirement/--path [--track copy-localization] [--track design-qa]`
returns selected and omitted cases with reasons and the project-required lane from
preferences. Magpie stays an unqualified future adapter because its generic mode
counts skipped skills as passes and is not installed here.

Command execution uses `std::process::Command` with argv, a per-case timeout
(default 600 s) enforced by polling and killing the child, and stdout and stderr
captured to `.chough/artifacts/<observation>.log` up to a bounded size with a visible
truncation marker. Running repository-bound commands has the same trust as running
the project's tests; it happens only on explicit invocation.

## Observations schema 2

Adds optional `cases` (ID and digest), `method`, `track`, `argv`, `exit_code`,
`tests_run`, `limitations`, outcome `not-run`, and kinds publication-observation,
installation-observation, deployment-observation, live-health-observation and
dry-run. Schema 1 records are read unchanged. Freshness adds `stale-case-definition`.
`evidence add` gains `--case`, `--method`, `--track` and `--limitation`.

## Bugs

`chough bug add|update|list|search|show|report`. IDs are `BUG-NNNN`, allocated by
scanning existing files. Status is open, investigating, fix-pending-verification,
verified, deferred, duplicate or wont-fix; scope is blocking, in-scope or
out-of-scope; classification is unclassified, product-defect, regression,
environment, requirement-gap or evidence-gap. A record is `unverified` until a
failed observation is linked; regression requires a passed observation whose revision
is an ancestor (`git merge-base --is-ancestor`). Updates append notes and evidence.
`search` scores token overlap of symptom and trigger plus shared criterion,
requirement and case IDs, and the agent decides on duplicates. `report` prints the
existing bug-report Markdown shape. Nothing is published.

## Preferences, consent, validation and release

`pref set <key> <value> --source <text>` writes `.chough/preferences.json`; `consent
grant <key> --source <text>` writes only the checkpoint's `run_consents`, tied to the
current objective. Keys and defaults: repeat-validation ask, monitoring ask, release
on-request, monitor-initiation capture-only, librarian off, library-dir `library`,
required-lane unset. `validate <case> --times N [--interval S] --environment E` runs
repeated case executions when consent or a standing `always` preference exists,
stores limits in `validation/<run>.json`, checks `validation.stop` before each
attempt, and records a summary with attempts, failures and elapsed time. `validate
stop` writes the stop request and `validate resume <run>` continues with the stored
limits. Each failure creates or appends to a local bug. With monitor-initiation
propose-change, the helper prints a suggested `specsync change new` command; it never
runs it. Results are named repeated runs, never soak or live health.

Release: `stage advance` from accept goes to done. `stage enter release` requires a
release consent or a standing release preference. Release work uses the project's
real commands, recorded with the distinct delivery kinds above.

## Librarian

Optional and off by default. `library init` creates `<dir>/INDEX.md` and
`<dir>/README.md`. `library snapshot <path>… --reason` copies bytes into
`<dir>/snapshots/<YYYY-MM-DD>-<n>/` with `manifest.json` entries for source,
revision, SHA-256 and reason, after the same path and symlink checks. `library
index` regenerates `INDEX.md` deterministically from HI files, `docs/FRD.md`, spec
modules, prototype directories with a `COMPANION.md`, native active and archived
changes, open bugs, observation counts, briefs and snapshots. Briefs are
agent-written from a template. Canonical sources are never moved or edited.

## Skill bundle and hosts

`skills/chough-workflow/` becomes: `SKILL.md` (short router), `references/stages.md`,
`intent.md`, `roles.md`, `design.md`, `plan.md`, `build.md`, `testing.md`, `bugs.md`
(existing, updated), `validate.md`, `accept.md`, `release.md`, `librarian.md`,
`checkpoint.md`, `lifecycle.md` (existing, updated), `templates/` (frd, review
findings, prototype index.html, styles.css and COMPANION.md, cases.toml, bug report,
brief), and `agents/openai.yaml`. Every file is embedded with `include_str!`.

`init --host <openai|claude|codex|cursor|gemini|grok>` is repeatable and defaults to
openai (`.agents/skills`, the current behavior). `skill status` and `skill update`
compare files with the embedded bundle and with SHA-256 digests of the 0.1.0 bundle
files. Roles (product and user advocate, QA, design, accessibility, engineering,
architecture) are described once in `roles.md`; a host may run them as parallel
subagents with explicit file ownership or as sequential passes. Only claude, and
any other host actually exercised, is reported as verified.

## Rust layout

`src/main.rs` keeps CLI parsing and dispatch. New modules: `records.rs` (paths,
bounded reads, atomic writes, IDs), `checkpoint.rs`, `stage.rs`, `cases.rs`,
`bugs.rs`, `prefs.rs`, `library.rs`, `skill.rs`. Existing context, status and
evidence code moves into modules only where needed. Module entry points use
`pub(super)`: the binary has no library target, so no item is reachable outside
the crate, and the contract describes commands rather than Rust symbols. This choice
is stated openly because SpecSync's checker would otherwise require documenting
`pub(crate)` items as API. One new dependency, `toml`, parses definitions; the
lockfile is updated and the offline source install is rechecked.

## Policy and CI

`.specsync/sdd.json`: set `require_change_for_meaningful_files` to true and add
`skills/` to `meaningful_paths`, so later product edits need a native change. CI
gains a `specsync change audit` step with `fetch-depth: 0` in the native gates job.
The Fledge pins stay. New test files keep the Windows-portable cases portable and gate
Unix-only behavior as the existing tests do.

## Compatibility and migration

Schema 1 sessions and observations remain readable. Chough's own `.chough/session.json`
is migrated explicitly in this change, its stale pointer to `docs/NEXT_CHANGE.md` is
replaced by this change ID, and `NEXT_CHANGE.md` is marked unapproved and deferred (not rejected) without
deleting its content. Installed 0.1.0 skill copies are updated only where unmodified.
APS is not a dependency; the checkpoint has one writer per project by convention
and no lock beyond atomic replace, which is documented with the concurrent-agent rule
of one owner per record.

## Open choices resolved here, reversible on request

1. Test definitions use TOML `chough-testing/1`, not 3md.
2. APS is qualified and deferred; `session.json` stays canonical.
3. Monitor initiation defaults to capture-only; propose-change only prints a command.
4. `cases run` and `validate` may execute bound commands on explicit invocation,
   which replaces the invariant that the helper never runs product tests.
5. Policy enables `require_change_for_meaningful_files` and treats `skills/` as meaningful.
6. No release, deployment, live monitoring or external publication is configured.

## From the change's testing.md

# Testing

Product checks run through `fledge run test` (Rust integration tests in isolated
temporary Git repositories with stubbed `hi` and `specsync` where native state must
be controlled). Structural checks run through `fledge run spec`, HI structure through
`fledge run intent`, and the full set through `fledge lanes run verify`. A structural
pass never stands in for the product checks below. Each requirement gets one or more
stable cases in `.chough/cases/cli.cases.toml`; case IDs are allocated during
implementation and recorded here.

## Track selection for this change

- Functional: always; every requirement below.
- Copy and terminology: relevant, because CLI output, skill references and docs are
  user-visible. Checked by review against a terminology list (observation, not proof;
  run consent versus standing preference; repeated run, not soak) and by assertions on
  key CLI strings.
- Design QA: Chough has no UI. Applied to the visual sample prototype and to rendered
  diagrams; recorded as rendered observations with method and limits.

## Requirement verification

| Requirement | Verification responsibility |
|---|---|
| REQ-cli-007 | Cross-process checkpoint round trip; migrate preserves schema 1 bytes and refuses an existing backup; stage commands refuse schema 1; context and status read schema 1 |
| REQ-cli-008 | Each transition kind; skip needs a reason; back and reset keep history and list refresh targets; build gate refused for draft and unavailable status, allowed for approved; leaving accept refused for verifying, allowed for archived; side-stage caller restored |
| REQ-cli-009 | Reordered file gives identical IDs and digests; edited expected outcome changes digest; duplicate ID, unknown criterion, unknown requirement, unknown field and missing binding rejected with location |
| REQ-cli-010 | Affected requirement selects linked functional cases; unrelated cases omitted with reason; design-QA and copy cases only with their track |
| REQ-cli-011 | Failing assertion recorded failed beside a passing case; zero-match cargo filter recorded not-run; missing expected output recorded failed; changed definition yields stale-case-definition; stop request halts a repeated run and resume keeps stored limits |
| REQ-cli-012 | Failure becomes reproduced classified bug; search finds duplicate and evidence is appended; regression without ancestor pass refused; out-of-scope bug listed without checkpoint scope change; report prints Markdown only |
| REQ-cli-013 | Consent leaves preferences byte-identical; preference requires source; validation refused without authorization; new objective clears consent; validation failure persists as a bug after a new process starts |
| REQ-cli-014 | Accept advances to done; release entry refused without authorization and allowed with consent; dry-run observation reported as dry-run |
| REQ-cli-015 | Snapshot keeps source bytes and digest; escape and symlink rejected; index lists open bugs; checkpoint works with the librarian off |
| REQ-cli-016 | Claude host install creates the complete bundle; repeat preserves edits; unmodified 0.1.0 file updated; edited file reported unchanged |
| REQ-cli-017 | Schema 1 observations keep freshness results; new kinds accepted; existing kinds valid |

REQ-cli-001 to REQ-cli-006 remain covered by the existing tests, which must keep
passing, including the Windows-portable record-limit test.

## Exercises recorded as observations

| Exercise | Method | Evidence |
|---|---|---|
| Feature and bug loop in an isolated sample repository: case fails on a deliberate assertion, bug recorded and deduplicated, fix, focused retest passes | Automated plus agent-assisted | Case observations, bug record, logs |
| Nonvisual sample skips design with a reason | Automated | Transition log entry |
| Visual sample prototype reviewed at two viewports and several states in a real browser | Agent-assisted rendered review | Screenshots and reviewer notes; human design approval left pending or explicitly simulated |
| Preference and consent behavior with synthetic configuration | Automated | Test results |
| Intent exercise: wording preserved, FRD derived, role findings synthesized into one decision | Agent-assisted | Sample files and findings |
| Cold start: a separate agent CLI, given only the repository, resumes one scoped task | Agent-assisted, independent session | Transcript of visible output, assessment against expected next action and gates |
| Diagrams rendered and inspected | Agent-assisted rendered review | SVG, PNG previews, verification record |
| Hosted CI at the final branch tip | Observed | Run URL and job results, reported per layer |

Missing capability is recorded as pending, not passed: for example, if no
independent agent session can run, FLOW-13 stays pending.

## Where these lessons go

- `specs/cli/context.md`
