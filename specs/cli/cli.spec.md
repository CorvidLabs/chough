---
module: cli
version: 88
status: active
files:
  - src/main.rs
  - src/bugs.rs
  - src/cases.rs
  - src/checkpoint.rs
  - src/context.rs
  - src/doctor.rs
  - src/change.rs
  - src/gates.rs
  - src/direct_writes.rs
  - src/evidence.rs
  - src/library.rs
  - src/native.rs
  - src/native_check.rs
  - src/policy.rs
  - src/prefs.rs
  - src/records.rs
  - src/skill.rs
  - src/stage.rs
  - src/validate.rs
  - src/vault.rs
  - tests/cli.rs
  - tests/bugs.rs
  - tests/cases.rs
  - tests/optional.rs
  - tests/package.rs
  - tests/stages.rs
  - tests/common/mod.rs
  - tests/native.rs
  - tests/workflow_authority.rs
  - tests/linux_image.rs
  - tests/subdirectory.rs
  - tests/doctor.rs
  - tests/change.rs
  - tests/gates.rs
  - tests/direct_writes.rs
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
  - tests/skill_releases.rs
  - tests/fixtures/skill-releases/manifest.json
  - tests/fixtures/skill-releases/blobs/00b3c3f4ffaaeeaaca6d829ff0277c9eb354ce0911f66f41a55639d27fc7d35d.txt
  - tests/fixtures/skill-releases/blobs/1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d.txt
  - tests/fixtures/skill-releases/blobs/10cd3cbe76a8a56e6e931bed15a42d4ec1a10ccc8a9091fac3f60ca507856069.txt
  - tests/fixtures/skill-releases/blobs/1492e0c5368df8762f09b2bb590a049f6dc88649c6003686951dddf6fe34b2bf.txt
  - tests/fixtures/skill-releases/blobs/18299d9d1adb30fb87a167a1a59ecd8595d1f53ec73522ccee257ea817641701.txt
  - tests/fixtures/skill-releases/blobs/187a887f2999e2d54350a56059a739f8379a7d77305ab53652da3b3eed8caa8d.txt
  - tests/fixtures/skill-releases/blobs/1a0a324a83658a1466ee6fe295ccfe9e980275f7abca809ef2927e71fb912663.txt
  - tests/fixtures/skill-releases/blobs/1c9ef93185390ee98aa4b4f2f9fbf2c442a003ad22097d0e74e4676b42404909.txt
  - tests/fixtures/skill-releases/blobs/23dcd8140f0d5bbbf77ea9a20ce37c43fde91345bf1cddf40b9336797a920a57.txt
  - tests/fixtures/skill-releases/blobs/28eccd903a89ff815e71e0cc3f81250e6c63bb986d93b0ee76b323180c4d92bf.txt
  - tests/fixtures/skill-releases/blobs/2b1aabe936ec051c040820625b7665b5d26706fa5e9396ddb4bfba8425ede78a.txt
  - tests/fixtures/skill-releases/blobs/2e26c47b349c4138d50eb78aa557822909c586802a9dbb12a46a466438d89b3b.txt
  - tests/fixtures/skill-releases/blobs/366259957ea1289ac4c9046a4c65b42cb4a7228ab93006ca9396bc5840d612e3.txt
  - tests/fixtures/skill-releases/blobs/367a9873293e79ca40c9634e2b2a12c3223074aaec161c7444690c2d02e12199.txt
  - tests/fixtures/skill-releases/blobs/370386488c8e5ff6c974ae9d3bd51998de31183868dfe48af44c0935a3019170.txt
  - tests/fixtures/skill-releases/blobs/3da19999ee4830a1fe1e0af63b13efc934f90d05678bbf70c3d79f39103c1150.txt
  - tests/fixtures/skill-releases/blobs/4013dc5ac116f3f32cffeb89ffcef957c3b402236000ea54c130ae41a02a96b5.txt
  - tests/fixtures/skill-releases/blobs/472396403a4f716f0183ec2c3495ac0ecf0dc412c1b61bb850cee767765e1f6a.txt
  - tests/fixtures/skill-releases/blobs/4995658517849e80e567a73eaf0399ec51123549a7e94649a17afa35ec40f20d.txt
  - tests/fixtures/skill-releases/blobs/4d0bf24ddd5e55493658339c76750a639a15997c4290a492cacda3550874f035.txt
  - tests/fixtures/skill-releases/blobs/4e085b88eaa66a59fd6c8ce7d3acb1201b46292603a2cf13ab5333fc5ea85d86.txt
  - tests/fixtures/skill-releases/blobs/51281ff6c9af4b939105cdf6ecf80d9dd11080dc8d682ecbba9d5b5c17d75c42.txt
  - tests/fixtures/skill-releases/blobs/5d7f85aa72b3bba8e5d83b593556bcdee244939b31038ed3e6000783aac5c4ea.txt
  - tests/fixtures/skill-releases/blobs/5f839a81742b9dc3df3b1278e340061a08e95e2a00e203f0f2554eab3aa3fed9.txt
  - tests/fixtures/skill-releases/blobs/658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f.txt
  - tests/fixtures/skill-releases/blobs/733346e29c7a57461926a6c7d297face74fc291a2e5c7f17d4d57dcf3945ec99.txt
  - tests/fixtures/skill-releases/blobs/772b6900528a84ad0c76638ad91536865ab7d62883d05eb49a8f7841896c2740.txt
  - tests/fixtures/skill-releases/blobs/816e40ecd47f39ac62e8f0f5e84a9e6277e3470221df1d9c80fff9d6bccb378e.txt
  - tests/fixtures/skill-releases/blobs/86eb8769f03a556ce4e198b13a028821961c3f686cfbf639afda5651de696102.txt
  - tests/fixtures/skill-releases/blobs/8a50dd8a1ea1afadffaf8b56f86d0031ada23ecc1f72569df3c3ff24642448b4.txt
  - tests/fixtures/skill-releases/blobs/8b9b582018094745034c033327df0093787c23c1b092c8cd706863012f748421.txt
  - tests/fixtures/skill-releases/blobs/8ca2b70f36ee162638cafd63bcc97e8c9ed688b59c2b67b98d61ba3be1d28f37.txt
  - tests/fixtures/skill-releases/blobs/92a9e61abe6011473f526868e731dc003b81835db720f6f4d61d704d92c22d73.txt
  - tests/fixtures/skill-releases/blobs/970d5a96be20ac737f8e1c9650bc327355792b3ee4b2e525acbbd7dc14c7ad30.txt
  - tests/fixtures/skill-releases/blobs/a159d2c4846ca0441337be0716fdd7c7fd96da27e7aea0b0b6e3dfb3b0f9696f.txt
  - tests/fixtures/skill-releases/blobs/a44f6812276d05332b69bac0cff237485c6aefc1973cddd77226c0ff97c66e83.txt
  - tests/fixtures/skill-releases/blobs/ac787118e30b11a8b5eb1429108bf1ac42911767b1ff33c56d68e925c84f11a3.txt
  - tests/fixtures/skill-releases/blobs/b3edaa0e8a58e944ab9ba82455750f8d27018ae6ca9b4b2d738a013e13ef3eed.txt
  - tests/fixtures/skill-releases/blobs/b7371621899a8926447873b1f80db3f64cc51361544251099793a1e1f79760e3.txt
  - tests/fixtures/skill-releases/blobs/ba9e89ee55dd51f7ac477a52c27ce1d641272a1fabc9be3fce150fa6321f47b8.txt
  - tests/fixtures/skill-releases/blobs/bb5d91c18199599e10195d457da0157ebcbfcb9595568b3cad743c8fc1f00908.txt
  - tests/fixtures/skill-releases/blobs/be351d5c3a8c65bd46f4784875b0118f82e5ba4a5b97a7d364d9856636d3b711.txt
  - tests/fixtures/skill-releases/blobs/ce8f63741058cf79afa275e01fb7c6ab0c38dc242c35c87a8628c808bf386239.txt
  - tests/fixtures/skill-releases/blobs/d2a185514ebe6f7428ed2d31c946126e6201bfd1e0feec5e0dccd1250474aa9c.txt
  - tests/fixtures/skill-releases/blobs/df33dfa9aa3db1355b47ef47a653e75ff4ae42a8c1de096bcd1718f6a4287c17.txt
  - tests/fixtures/skill-releases/blobs/df42d472ae25b0ed07ad3beb1257ce77c3fe4e159feefe8e70e7c18ec03a6945.txt
  - tests/fixtures/skill-releases/blobs/e232af339e12254fcc735ffa87b72c36d0e80a8b3efc585cb7be627973d42235.txt
  - tests/fixtures/skill-releases/blobs/e61ad96fe5e2933c538d1ac775e7541a025d81fcd02804dc7c4d0fd215ed10f0.txt
  - tests/fixtures/skill-releases/blobs/e92e5a8fdc2e7b02ac6e01f34e15ee66501f7e3a312970069abb9e409aca167a.txt
  - tests/fixtures/skill-releases/blobs/ef831afa1094da91e5766c2d9597d6c933abe11b4d1a50913893415bd0d511a4.txt
  - tests/fixtures/skill-releases/blobs/fcbc759ab4605a6dd8fffd4af4498783186e308c6d771284bb9a2626f6e0b4e8.txt
  - tests/evidence_inventory_compatibility.rs
  - tests/fixtures/evidence-inventory/OBS-tc-cli-release-corpus-identity-ef1726110e673bd961ec3f56.json
  - tests/fixtures/evidence-inventory/OBS-tc-cli-release-edited-preservation-0e13b37adb18b0e04c90dd23.json
  - tests/fixtures/evidence-inventory/OBS-tc-cli-release-host-current-missing-ee55b3fe9e5380440663ed62.json
  - tests/fixtures/evidence-inventory/OBS-tc-cli-release-legacy-policy-9aaea89ee334873bcfa7b314.json
  - tests/fixtures/evidence-inventory/OBS-tc-cli-release-name-byte-matching-9218194092c97b28451038d3.json
  - tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7.json
  - tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b.json
  - tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619.json
  - tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e.json

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
records observations tied to repository content and environment. Evidence-prune
apply rechecks protections immediately before removal, accepts captured artifact
bytes only when they match the eligible digest, and restores partial removals with
per-file recovery that continues around unsafe restoration destinations without
weakening retention guarantees.
See requirements.md and docs/FRD.md.

## Public API

The binary exposes CLI commands; it does not export a Rust library API. The
observable command contract is `chough --help`, docs/cli.md and README.md. The
command groups are init, context, status, next, migrate, checkpoint, stage, history, cases,
evidence, bug, pref, consent, validate, library, skill and timing.

Internal modules under src/ make their entry points visible to the crate root only.
They are implementation details of the binary, not exports or library promises, and
command behavior is specified under Behavioral Examples and in docs/cli.md rather
than presented as Rust API symbols to the structural checker. The record identity
boundary moved to the records module with `src/identity.rs` in Chough 0.6.0.

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
   Chough records, build output, the generated knowledge directory and SpecSync
   lifecycle bookkeeping; the revision is
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
| 68 | 2026-11-06 | a-session-whose-native-change-is-archived-is-finished-and-abandoned-work-closes-with-a-reason-163-drive-16: A session whose native change is archived is finished, and abandoned work closes with a reason (163, DRIVE-16) |
| 69 | 2026-11-06 | chough-doctor-finds-and-fixes-the-chough-debt-that-stops-a-fresh-worktree-from-entering-chough-drive-17: Chough doctor finds and fixes the Chough debt that stops a fresh worktree from entering Chough (DRIVE-17) |
| 70 | 2026-11-06 | a-project-in-a-subdirectory-of-its-git-repository-records-evidence-against-its-own-files-170-drive-18: A project in a subdirectory of its Git repository records evidence against its own files (170, DRIVE-18) |
| 71 | 2026-11-06 | run-specsync-s-change-new-answer-check-and-status-through-chough-171-drive-20: Run SpecSync's change new, answer, check and status through Chough (171, DRIVE-20) |
| 72 | 2026-11-06 | rebuild-the-linux-test-environment-from-the-repository-with-one-command-using-the-project-s-pinned-tool-versions-177: Rebuild the Linux test environment from the repository with one command, using the project's pinned tool versions (177, DRIVE-22) |
| 73 | 2026-11-06 | make-archived-next-and-status-fast-within-explicit-warm-budgets: Make archived next and status fast within explicit warm budgets (32, DRIVE-15) |
| 74 | 2026-11-06 | chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20: Chough change ship: ship after the review, close the review gate with the relay, and move to done (179, DRIVE-20) |
| 75 | 2026-11-07 | flag-direct-specsync-lifecycle-writes-in-history-183-drive-21: Flag direct SpecSync lifecycle writes in history (183, DRIVE-21) |
| 76 | 2026-11-07 | chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20: Chough gate list: one-screen gate summaries with digests, and one reply for listed gates only (184, FLOW-19, FLOW-20) |
| 77 | 2026-11-07 | after-the-move-into-done-chough-library-index-check-passes-with-no-command-in-between-session-notes-link-archive-notes: After the move into done, chough library index --check passes with no command in between: session notes link archive notes from their sources, and the knowledge directory is regenerated after the session archives itself (#191, KNOW-4) |
| 78 | 2026-11-07 | chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21: chough timing skips direct-native-write events, and chough change new passes --no-spec-change (189, DRIVE-21, DRIVE-20) |
| 79 | 2026-11-07 | make-evidence-prune-apply-recheck-protections-and-restore-partial-removals: Make evidence prune --apply recheck protections and restore partial removals (65) |
| 80 | 2026-11-07 | correct-prune-capture-authority-and-unsafe-rollback-for-pr-197: Correct prune capture authority and unsafe rollback for PR 197 |
| 81 | 2026-11-09 | recognize-pristine-skills-from-pinned-released-bundles-without-replacing-edits: Recognize pristine skills from pinned released bundles without replacing edits |
| 82 | 2026-11-10 | read-bounded-diagnostic-input-inventories-without-changing-freshness-authority: Read bounded diagnostic input inventories without changing freshness authority |
| 83 | 2026-11-10 | keep-observations-current-when-the-generated-knowledge-index-is-rebuilt-136: Keep observations current when the generated knowledge index is rebuilt (#136) |
| 84 | 2026-11-10 | return-the-observation-id-from-cases-record-and-evidence-add-issue-60: Return the observation ID from cases record and evidence add (issue 60) |
| 85 | 2026-11-12 | chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an: Chough 0.6.0 human-first records: one Markdown file with YAML front matter per record, date+slug session folders with an append-only timeline that fold into one archived story at finish, a knowledge vault that links and never copies, no local facts or credentials in committed records, a human's gate approval drives the SpecSync approval as that human, and chough migrate converts 0.5.x projects with a digest receipt |
| 86 | 2026-11-12 | prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood: Prepare the Chough 0.6.0 release: Cargo 0.6.0 and the dated CHANGELOG with the dogfood results, and the fixes the dogfood found |
| 87 | 2026-11-13 | 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json: 0.6.1: chough change ship prints a short text summary after SpecSync's output, with its full report under --json |
| 88 | 2026-11-13 | fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve: Fix #249: chough passes a relayed decision's words, place and recording agent to SpecSync's review and approve |
