---
change: evolve-chough-into-a-resumable-stage-workflow
artifact: design
---

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
replaced by this change ID, and `NEXT_CHANGE.md` gains a superseded banner without
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
