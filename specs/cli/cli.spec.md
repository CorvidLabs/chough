---
module: cli
version: 67
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
  - src/native_check.rs
  - src/policy.rs
  - src/prefs.rs
  - src/records.rs
  - src/skill.rs
  - src/stage.rs
  - src/validate.rs
  - src/vault.rs
  - src/attachments.rs
  - tests/cli.rs
  - tests/bugs.rs
  - tests/cases.rs
  - tests/optional.rs
  - tests/package.rs
  - tests/stages.rs
  - tests/common/mod.rs
  - tests/workflow_authority.rs
  - tests/fixtures/workflow-at-825a380-SKILL.md
  - tests/fixtures/workflow-at-825a380-lifecycle.md
  - tests/fixtures/workflow-at-edbc3c1-lifecycle.md
  - tests/fixtures/workflow-at-edbc3c1-checkpoint.md
  - tests/next.rs
  - tests/librarian_trim.rs
  - tests/native_check.rs
  - tests/verification.rs
  - tests/knowledge_default.rs
  - tests/vault.rs
  - tests/reduce.rs

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
command groups are init, context, status, next, migrate, checkpoint, stage, history, cases,
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
2. The helper never creates native scope approval, implementation review, release or
   deployment records, and never authenticates anyone. A relayed decision it keeps
   (REQ-cli-050) is a recorded claim of who decided, and never satisfies a native gate.
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
| 14 | 2026-11-02 | make-chough-timing-read-each-native-change-at-most-once-skip-archived-history-the-requested-report-does-not-show-bound: Make chough timing read each native change at most once, skip archived history the requested report does not show, bound total native wait per report with explicit unknown states, and add an opt-in phase trace |
| 15 | 2026-11-02 | fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer: Fresh-agent handoff from existing records: chough next shows recent decisions, the last failure and a lessons pointer, names a new cycle after the native change is archived, and the skill labels lesson certainty and closes out inside the open change |
| 16 | 2026-11-02 | agents-state-the-recorded-handoff-before-their-first-edit-the-skill-s-resume-steps-ask-a-fresh-agent-to-say-the-goal: Agents state the recorded handoff before their first edit: the skill's resume steps ask a fresh agent to say the goal, binding decisions with reasons, the last failure with its bug status, the applicable lessons and the next task in its own words |
| 17 | 2026-11-02 | case-recording-order-the-shipped-skill-keeps-running-and-recording-chough-cases-out-of-tasks-md-checkboxes-and-runs-the: Case recording order: the shipped skill keeps running and recording Chough cases out of tasks.md checkboxes and runs the scoped specsync change check first, so a case can link a requirement an approved change adds (BUG-0017, issue 11) |
| 18 | 2026-11-02 | unlinked-failure-gate-leaving-test-refuses-while-a-checkpoint-case-s-latest-observation-failed-and-no-local-bug-links: Unlinked failure gate: leaving test refuses while a checkpoint case's latest observation failed and no local bug links it (issue 39) |
| 19 | 2026-11-02 | unlinked-failure-ignores-not-run-leaving-test-keeps-refusing-when-a-referenced-case-s-latest-executed-observation: Unlinked failure ignores not-run: leaving test keeps refusing when a referenced case's latest executed observation failed and a later observation is only not-run (PR 43 review) |
| 20 | 2026-11-02 | human-gate-hold-stage-advance-and-skip-refuse-while-the-checkpoint-has-an-open-awaiting-human-gate-issue-38: Human-gate hold: stage advance and skip refuse while the checkpoint has an open awaiting-human gate (issue 38) |
| 21 | 2026-11-02 | new-project-setup-path-and-sdd-off-hint-document-hi-specsync-init-scaffold-and-specsync-change-adopt-for-a-new-project: New-project setup path and SDD-off hint: document HI, specsync init/scaffold and specsync change adopt for a new project, and have chough next say to run specsync change adopt when SpecSync is configured but SDD is off (issue 40) |
| 22 | 2026-11-02 | bound-transition-history-and-preserve-referenced-proof: Bound transition history and preserve referenced proof |
| 23 | 2026-11-02 | Integrate merged bounded history (main version22) with the agent-operated native workflow (approved on parallel version17, previously integrated as parallel version22), preserving both accepted meanings, ordering and gates. |
| 24 | 2026-11-02 | read-report-case-definitions-once-per-request-and-expose-local-phase-costs: Read report case definitions once per request and expose local phase costs |
| 25 | 2026-11-03 | identity-acceptance-evidence-for-issue-30-a-pinned-clock-same-second-run-in-two-git-roots-and-a-real-git-integration: Identity acceptance evidence for issue 30: a pinned-clock same-second run in two Git roots and a real Git integration fixture for generated records |
| 26 | 2026-11-03 | raise-the-checkpoint-session-byte-limit-to-131072-and-the-observation-pointer-limit-to-400: Raise the checkpoint session byte limit to 131072 and the observation pointer limit to 400 |
| 27 | 2026-11-03 | approved-delta-guard-refusals-name-the-acting-agent-account-and-the-human-authority-note-instead-of-asking-the-human-to: Approved-delta guard refusals name the acting agent account and the human authority note instead of asking the human to approve under their name |
| 28 | 2026-11-03 | claude-host-installs-the-shared-agents-skill-copy-legacy-claude-copies-are-reported-and-removed-only-on-an-explicit: Claude host installs the shared .agents skill copy, legacy .claude copies are reported and removed only on an explicit command, and this repository drops its .claude copy |
| 29 | 2026-11-03 | split-the-cli-spec-move-the-retention-and-transition-history-requirements-req-cli-018-and-req-cli-025-to-req-cli-027-to: Split the cli spec: move the retention and transition-history requirements REQ-cli-018 and REQ-cli-025 to REQ-cli-027 to a new history module as REQ-history-001 to REQ-history-004 with wording unchanged and a documented ID mapping, so the cli requirements companion has headroom under the 65536-byte record bound |
| 30 | 2026-11-03 | context-knowledge-directory-knowledge-by-default-with-the-librarian-on-and-an-opt-out-one-regenerated-summary-per: Context knowledge directory: knowledge/ by default with the librarian on and an opt-out, one regenerated summary per archived change with a lessons index, and raw proof outside the newest 3 archived changes leaves the working tree only through a previewed, authorized, validity-checked step |
| 31 | 2026-11-03 | read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies: Read native SpecSync spec files under specs at any size: Chough's 65536-byte structured-record bound no longer applies to spec companions such as requirements.md, every Chough record keeps its bound, and the workflow skill gains advisory composability guidance |
| 32 | 2026-11-03 | search-a-module-context-md-for-its-lessons-heading-at-any-size-in-chough-next-as-req-cli-034-requires-instead-of: Search a module context.md for its Lessons heading at any size in chough next, and remove next's named 4 MiB exception from REQ-cli-034 while library snapshot's exception stays |
| 33 | 2026-11-04 | record-a-stage-gate-that-already-happened-with-its-original-time-quoted-evidence-and-a-backfilled-flag-that-timing: Record a stage gate that already happened with its original time, quoted evidence and a backfilled flag that timing reports separately (DRIVE-5) |
| 34 | 2026-11-04 | stop-what-chough-started-when-chough-is-stopped-issue-75: Stop what Chough started when Chough is stopped (issue 75) |
| 35 | 2026-11-04 | let-chough-cases-link-a-requirement-that-the-checkpoint-s-approved-native-change-adds-before-it-is-materialized-and: Let Chough cases link a requirement that the checkpoint's approved native change adds before it is materialized, and document how the test stage links failing-first evidence (DRIVE-6, Raven BUG-0004) |
| 36 | 2026-11-04 | refuse-a-native-change-id-that-specsync-does-not-know-issue-63: Refuse a native change ID that SpecSync does not know (issue 63) |
| 37 | 2026-11-04 | refuse-a-native-change-status-reply-that-names-no-change-id-issue-63-review-thread-on-71: Refuse a native change status reply that names no change ID (issue 63, review thread on #71) |
| 38 | 2026-11-04 | split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new: Split the cli spec: move the timing and reporting requirements REQ-cli-020, REQ-cli-024 and REQ-cli-028 to a new reporting module as REQ-reporting-001 to REQ-reporting-003 with wording unchanged and a documented ID mapping, so the cli requirements companion stays under the 65536-byte record bound |
| 39 | 2026-11-04 | order-next-lesson-bundles-by-finalization-and-guide-a-fresh-cycle-to-a-new-native-change-issues-74-and-76: Order next lesson bundles by finalization and guide a fresh cycle to a new native change (issues 74 and 76) |
| 40 | 2026-11-04 | read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an: Read HI exports and native SpecSync files at full size for every gate and validation so Chough bounds never push an agent to edit human intent or specs (issue 100) |
| 41 | 2026-11-04 | warn-when-the-native-check-runs-none-of-the-project-commands-judged-from-the-effective-commands-specsync-reports-and: Warn when the native check runs none of the project commands, judged from the effective commands SpecSync reports, and point the test stage to recorded test evidence (issue 85) |
| 42 | 2026-11-04 | show-the-latest-failed-native-check-in-next-and-say-when-it-has-not-passed-for-this-tree-issue-86: Show the latest failed native check in next and say when it has not passed for this tree (issue 86) |
| 43 | 2026-11-04 | show-a-failed-native-command-as-exit-n-or-killed-by-signal-n-instead-of-rust-debug-text-issue-87: Show a failed native command as exit N or killed by signal N instead of Rust debug text (issue 87) |
| 44 | 2026-11-04 | stop-descendants-that-left-the-process-group-on-linux-and-report-git-failures-and-grace-kills-of-chough-own-git: Stop descendants that left the process group on Linux and report Git failures and grace kills of Chough own Git commands (issue 81) |
| 45 | 2026-11-04 | trim-old-proof-automatically-when-a-change-reaches-done-with-the-librarian-on-and-let-retention-manual-opt-out-issue-88: Trim old proof automatically when a change reaches done with the librarian on, and let retention manual opt out (issue 88) |
| 46 | 2026-11-04 | keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer: Keep one session per objective or native change under .chough/sessions with append-only transitions, an active pointer, automatic archiving and an append-only subagent inbox (DRIVE-3) |
| 47 | 2026-11-04 | show-open-questions-in-next-and-status-report-an-archived-session-as-archived-and-warn-when-an-inbox-observation-has-no: Show open questions in next and status, report an archived session as archived, and warn when an inbox observation has no record (issue 84) |
| 48 | 2026-11-04 | keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy: Keep the checkpoint writer lock in the project so every temp directory shares it, name the file being written when busy, and specify serialized writers and the local state directory (issue 83) |
| 49 | 2026-11-05 | build-the-knowledge-directory-by-default-chough-init-creates-it-and-every-move-into-done-refreshes-it-after-the-trim: Build the knowledge directory by default: chough init creates it and every move into done refreshes it after the trim; fold the 2026-11-04 lessons into the module lesson sources |
| 50 | 2026-11-05 | make-the-knowledge-directory-an-obsidian-style-vault-yaml-front-matter-on-every-note-iso-dates-standard-links-and: Make the knowledge directory an Obsidian-style vault: YAML front matter on every note, ISO dates, standard links, and module, intent and rules notes that connect each archived change to what it touched |
| 51 | 2026-11-05 | reduce-old-observation-proof-drop-closed-cycle-checkpoint-references-at-done-trim-superseded-runs-of-cold-changes: Reduce old observation proof: drop closed-cycle checkpoint references at done, trim superseded runs of cold changes, attribute unlisted proof to its change by cycle time, and read archive validity in parallel |
| 52 | 2026-11-05 | move-old-proof-into-the-knowledge-vault-trim-loose-artifacts-of-cold-changes-and-move-key-items-into-knowledge: Move old proof into the knowledge vault: trim loose artifacts of cold changes and move key items into knowledge attachments (KNOW-18, KNOW-19) |
| 53 | 2026-11-05 | keep-vault-attachment-paths-within-windows-limits-and-keep-file-modes-when-the-trim-moves-or-restores-files: Keep vault attachment paths within Windows limits and keep file modes when the trim moves or restores files |
| 54 | 2026-11-05 | never-overwrite-an-attachment-let-folder-references-protect-their-files-and-keep-a-loose-folder-with-any-current-file: Never overwrite an attachment, let folder references protect their files, and keep a loose folder with any current file (109 review P1s) |
| 55 | 2026-11-05 | no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12: No conflicting files: per-change case files, a knowledge rebuild check and a bug merge driver (DRIVE-12) |
| 56 | 2026-11-05 | done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21: Done drops every file and design reference, and archived sessions hold no proof by them (KNOW-21) |
| 57 | 2026-11-06 | closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22: Closed bugs release proof: key items move into the vault under the bug and the rest is trimmed when cold (KNOW-22) |
| 57 | 2026-11-06 | rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their: Rotate the retention ledger: keep the newest 100 entries and move older summarized ones into the vault with their archive (KNOW-23) |
| 58 | 2026-11-06 | one-librarian-default-in-the-requirements-req-cli-013-defers-to-req-cli-015-on-124-d8: One librarian default in the requirements: REQ-cli-013 defers to REQ-cli-015, on (124, D8) |
| 59 | 2026-11-06 | size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each: Size the checkpoint from measured use: a 1 MiB cap with at-cap budgets, session shelve, and sessions that never break each other (DRIVE-14, 125) |
| 60 | 2026-11-06 | keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29: Keep the five most recent archived sessions and roll older ones into the knowledge vault (KNOW-29) |
| 61 | 2026-11-06 | native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10: Native check AC2: only a verifying change with handoff readiness safe shows no note (126, D10) |
| 62 | 2026-11-06 | record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7: Record relayed human decisions at gates as claims, not authentication (123, D7) |
| 63 | 2026-11-06 | advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into: Advance through several stages on one native read with stage advance --to, and read native status once on the move into done (DRIVE-9, DRIVE-2) |
| 64 | 2026-11-06 | assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11: Assert the eight-reader ceiling for archive validity reads on any host (CH-M11) |
| 65 | 2026-11-06 | a-session-archives-itself-at-its-own-move-into-done-155: A session archives itself at its own move into done (155) |
| 66 | 2026-11-06 | assert-positive-track-selection-and-every-delivery-observation-kind-and-that-a-dry-run-never-stands-in-for-one-152-ma: Assert positive track selection and every delivery-observation kind, and that a dry run never stands in for one (152, MA-01, MA-02) |
| 67 | 2026-11-06 | bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153: Bug classification, deduplication and report mappings, and their recovery branches (153) |
