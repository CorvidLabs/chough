---
module: cli
version: 14
status: active
files:
  - src/main.rs
  - src/bugs.rs
  - src/cases.rs
  - src/checkpoint.rs
  - src/context.rs
  - src/evidence.rs
  - src/identity.rs
  - src/library.rs
  - src/prefs.rs
  - src/records.rs
  - src/skill.rs
  - src/stage.rs
  - src/timing.rs
  - src/validate.rs
  - tests/cli.rs
  - tests/bugs.rs
  - tests/cases.rs
  - tests/optional.rs
  - tests/package.rs
  - tests/stages.rs
  - tests/timing.rs
  - tests/common/mod.rs

db_tables: []
depends_on: []
---

# Chough CLI

## Purpose

Provide the optional local helper for Chough's current-agent workflow. It installs
the workflow skill for supported hosts without replacing user files, keeps one
canonical checkpoint of the current stage, scope, decisions, preferences and human
gates, routes stage transitions within native lifecycle gates, assembles bounded
focused context, validates test definitions and runs explicitly invoked automated
cases, keeps a local bug backlog, supports an optional project-local librarian, and
records observations tied to repository content and environment. See
requirements.md and docs/FRD.md.

## Public API

The binary exposes CLI commands; it does not export a Rust library API. The
observable command contract is `chough --help`, docs/cli.md and README.md. The
command groups are init, context, status, next, migrate, checkpoint, stage, cases,
evidence, bug, pref, consent, validate, library, skill and timing.

Internal modules under src/ make their entry points visible to the crate root only.
They are implementation details of the binary, not exports or library promises, and
command behavior is specified under Behavioral Examples and in docs/cli.md rather
than presented as Rust API symbols to the structural checker. The following table
covers the crate-internal identity boundary without creating a public Rust API.

| Internal symbol | Structural behavior |
|---|---|
| `OBSERVATION_SLUG_LIMIT` | Limits an observation slug so the complete OBS ID fits 100 bytes. |
| `VALIDATION_SLUG_LIMIT` | Limits a validation slug so the complete VAL ID fits 100 bytes. |
| `SNAPSHOT_SLUG_LIMIT` | Limits a snapshot slug so the complete SNAP ID fits 100 bytes. |
| `slug` | Normalizes bounded nonempty lowercase ASCII slugs with caller-selected fallbacks. |
| `valid_case_id` | Accepts curated slug and exact legacy numeric case IDs with the module-aware bound. |
| `valid_observation_id` | Accepts the exact new OBS form with a complete 24-digit lowercase hex suffix. |
| `valid_validation_id` | Accepts the exact new VAL form with a complete 24-digit lowercase hex suffix. |
| `valid_snapshot_id` | Accepts the exact new SNAP form with a complete 24-digit lowercase hex suffix. |
| `observation` | Allocates one OBS candidate from one random 96-bit draw. |
| `validation` | Allocates one VAL candidate from one random 96-bit draw. |
| `snapshot` | Allocates one SNAP candidate from one random 96-bit draw. |
| `require_explicit_observation_id` | Applies new-form explicit observation validation and the old-style omit-ID remedy. |
| `with_random_results` | Test-only random-source seam; no production override. |
| `random_draws` | Test-only draw-count seam used to prove there is no retry. |

## Invariants

1. Initialization and skill installation never overwrite an existing user file;
   updates replace only files byte-identical to a known earlier bundle.
2. The helper never creates, records or authenticates human scope approval,
   implementation review, release or deployment.
3. HI owns criterion IDs; SpecSync owns its lifecycle, approvals, reviews and
   archives. The checkpoint references native records and never duplicates their state.
4. Observations record revision, working content, environment and, for cases, the
   definition digest. Freshness compares the working content, which leaves out
   Chough records, build output and SpecSync lifecycle bookkeeping; the revision is
   recorded but not compared, except for records without a content identity. Current
   evidence is not equivalent to proven criterion achievement.
5. Managed and selected paths stay inside the project and symlinks are rejected.
   Context, records and captured output are bounded and deterministic for the same
   inputs and native tool responses.
6. Missing tools, missing sources, unsupported schemas, unknown native state and
   stale evidence remain visible; they never silently become successful states.
7. The helper executes a product check only when `cases run` or `validate` is
   explicitly invoked, runs the bound argument vector without a shell, and records
   passed only when the binding's assertion holds.
8. `.chough/session.json` is the single writable checkpoint. Transition history is
   append-only, and schema migration is explicit and preserves the original bytes.
   Bounded three-way merges accept schema 2 only, replace only ours on success and
   introduce no second writable checkpoint store.
9. The helper publishes nothing to an external service.
10. Bug records remain schema 1. Manual and automatic creation share one generator
    for `BUG-<lowercase-title-slug>-<24-lowercase-hex>` IDs, whose creation-time
    digest, ID and filename never change during updates. Exact legacy `BUG-NNNN`
    records remain usable without migration or renumbering, and duplicate and related
    links accept either ID form. Bug generation and known answers do not change when
    other record identities do.
11. New cases use curated `TC-<module>-<title-slug>` IDs with lowercase ASCII slug
    syntax, a module-aware 100-byte bound and global duplicate refusal. Exact legacy
    `TC-<module>-NNNN` cases retain their IDs and definition digests without
    renumbering or migration.
12. Automatic observations, validation runs and snapshots use
    `OBS-<slug>-<24-lowercase-hex>`, `VAL-<slug>-<24-lowercase-hex>` and
    `SNAP-<slug>-<24-lowercase-hex>`. Each suffix encodes one random 96-bit draw.
    Creation makes one candidate and one atomic no-replace attempt, preserving
    existing bytes on collision or failure without fallback or retry. Timestamps
    remain the chronology source, legacy IDs remain readable and resumable where
    supported, and automated artifacts inherit the exact observation ID.

## Behavioral Examples

```gherkin
Given an edited session and installed skill
When init is repeated
Then existing files remain byte-for-byte unchanged

Given product-test observations on one revision/environment
When relevant working files change or a different environment is selected
Then status reports those observations stale or mismatched

Given a selected module and scoped HI family
When a new agent requests context
Then it receives instructions, intent, module companions, decisions and next action
without loading the whole repository or conversation history

Given a checkpoint at plan whose native change is still a draft
When the agent asks to advance to build
Then the transition is refused and native status is named as the authority

Given two cases where one bound assertion fails
When both cases are run
Then the failing case is recorded failed and the passing case does not hide it

Given a run consent for repeated validation
When the objective changes
Then the consent is cleared and project preferences are unchanged

Given independent roots at the same legacy numeric high-water mark
When distinct bug reports are created manually and by automatic failure capture
Then each distinct report receives a different new-form ID
And legacy schema 1 records and cross-form duplicate and related links remain usable

Given equivalent independent roots and identical repeated operations in one root
When observations, validation runs and snapshots are allocated
Then each new record receives its required slug and random 96-bit suffix form
And identical repeated operations remain distinct history

Given a new observation write with an old-style explicit ID
When the command is invoked
Then it refuses before writing an observation or artifact and says to omit the ID

Given mixed legacy and new case, observation, validation and snapshot records
When links, status, timing, retention, resume and indexes read them
Then both forms work without migration or schema rewrite and timestamps determine chronology

Given schema 2 base, ours and theirs checkpoints with compatible collection edits and an unchanged current-cycle group
When checkpoint merge runs directly or through the configured Git driver
Then exact-string additions, removals and ordering are reconciled and only ours is replaced

Given base, ours and theirs checkpoints whose current-cycle groups changed differently on the two sides
When checkpoint merge runs
Then it refuses with a current-cycle conflict and leaves all three inputs byte-identical without creating a Chough record
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Invalid path/schema | A record escapes the root or uses an unsupported version | Fail with an actionable error |
| Missing native tool/source | Context or lifecycle information is unavailable | Report unavailable/unknown without inventing state |
| Existing observation ID | An observation record already exists | Refuse replacement and retain the original |
| Old-style explicit observation ID | A new write supplies an ID outside the new OBS form | Refuse before observation or artifact creation and give the omit-ID remedy |
| Identity collision or allocation failure | A same-ID different-content create-new occurs, the random source fails, or a later write fails | Preserve pre-existing records and artifacts; do not overwrite, retry or allocate a timestamp, numeric or digest fallback |
| Existing bug ID | A new bug candidate ID already exists, including a same-ID different-content result | Refuse creation and leave the original record byte-identical, with no overwrite, digest retry or numeric fallback |
| Invalid traceability | A criterion, requirement or case cannot be established | Refuse to record the observation |
| Native gate not satisfied | A stage transition needs a native state that is absent or unknown | Refuse the transition and name the native status command |
| Unauthorized optional track | Repeated validation or release lacks consent or a standing preference | Refuse and explain how a human can authorize it |
| Vacuous check | A bound command exits zero without its declared assertion | Record not-run or failed, never passed |
| Schema 1 session | A stage command meets a schema 1 checkpoint | Refuse and name the explicit migration command |
| Checkpoint merge refusal | Inputs use schema 1, mixed or unsupported versions, malformed content or unknown fields; bounds overflow; or current-cycle groups conflict | Fail closed with all three inputs byte-identical and no checkpoint, transition, gate, consent, approval, review or other record |

## Dependencies

Rust dependencies are declared in Cargo.toml, including a TOML parser for test
definitions and direct use of the existing getrandom 0.4.3 package for random record
suffixes. Git supplies revision and working content identity; HI supplies scoped
intent; SpecSync supplies native lifecycle output. Cargo, Fledge or another bound
command runs only when a case is explicitly executed. The helper invokes no model
provider, runtime or hosted service.

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-10-31 | Authorized first-slice bootstrap contract; no native approval claimed |
| 2 | 2026-10-31 | evolve-chough-into-a-resumable-stage-workflow: Evolve Chough into a resumable stage workflow |
| 3 | 2026-10-31 | refuse-stage-moves-whose-native-approval-validity-archive-integrity-or-sdd-policy-does-not-hold: Refuse stage moves whose native approval validity, archive integrity or SDD policy does not hold |
| 4 | 2026-10-31 | bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting: Bound external commands in time and memory and honor validation stop requests after waiting |
| 5 | 2026-10-31 | prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package: Prepare a first release candidate with reference-aware evidence retention, current guidance and an installable package |
| 6 | 2026-11-01 | keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after: Keep evidence current across commits and SpecSync lifecycle bookkeeping, and verify bugs only by a pass observed after the failure |
| 7 | 2026-11-01 | report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records: Report how long stages, tests, bug fixes and human decisions took, from existing records |
| 8 | 2026-11-01 | bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors: Bundle hygiene, case binding environment and colour-safe matching, and actionable requirement and bug errors |
| 9 | 2026-11-01 | focused-chough-next-a-verification-guard-before-build-and-done-stage-resume: Focused chough next, a verification guard before build, and done-stage resume |
| 10 | 2026-11-01 | prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill: Prepare the 0.1.0 release: pinned toolchain with one install command, a checked minimum Rust version, accurate skill guidance and contract wording, and a change log |
| 11 | 2026-11-01 | use-collision-resistant-slug-identities-for-new-bugs: Use collision-resistant slug identities for new bugs |
| 12 | 2026-11-01 | make-checkpoint-merges-safe-across-parallel-changes: Make checkpoint merges safe across parallel changes |
| 13 | 2026-11-02 | use-slug-identities-for-all-new-chough-records: Use slug identities for all new Chough records |
| 14 | 2026-11-02 | fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer: Fresh-agent handoff from existing records: chough next shows recent decisions, the last failure and a lessons pointer, names a new cycle after the native change is archived, and the skill labels lesson certainty and closes out inside the open change |
