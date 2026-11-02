# Lesson bundle: make-checkpoint-merges-safe-across-parallel-changes

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Make checkpoint merges safe across parallel changes
- **Kind**: Feature
- **Specs**: cli
- **Paths**: .gitattributes, src/checkpoint.rs, src/main.rs, src/skill.rs, tests/stages.rs, tests/cli.rs, tests/optional.rs, tests/package.rs, specs/cli/requirements.md, specs/cli/testing.md, specs/cli/tasks.md, specs/cli/cli.spec.md, docs/FRD.md, docs/cli.md, docs/architecture.md, README.md, skills/chough-workflow/references/checkpoint.md, .agents/skills/chough-workflow/references/checkpoint.md, .claude/skills/chough-workflow/references/checkpoint.md
- **Acceptance**: `chough checkpoint --merge <base> <ours> <theirs>` performs a bounded three-way merge of schema 2 checkpoints. Exact-string history and reference collections preserve base order plus ours-only and theirs-only additions, identical additions appear once, and one-side removals are not resurrected.
- **Acceptance**: Objective, intent scope, modules, stage, native change, next action, run consents, blockers and awaiting-human gates merge as one current-cycle group: an unchanged or identical side is selected, while conflicting changes refuse with a named reason. Every refusal leaves all inputs byte-identical and creates no transition, gate, approval, review or consent record.
- **Acceptance**: Schema 1, malformed input, unknown fields, mixed versions and output beyond existing bounds fail closed. The canonical checkpoint remains .chough/session.json with no second writable store, runtime, service or duplicate lifecycle state.
- **Acceptance**: A .gitattributes declaration and documented per-clone Git driver invoke the same command without changing Git configuration during init. A real Git fixture proves compatible merges and unresolved current-cycle conflicts. The explicit command remains the fallback when no driver is available.
- **Acceptance**: Failure-first evidence is retained for the current conflicting checkpoint behavior and linked to a local bug before implementation, followed by a later pass. Existing schema 1 read and migration behavior, one-owner live writes, native SpecSync authority and all non-checkpoint ID schemes remain unchanged.
- **Acceptance**: Focused and full Rust tests, real Git merge tests, formatting, Clippy with warnings denied, fledge verify and pre-push pass with the current CI tool versions. No new case number is allocated; evidence uses explicit readable IDs. No release or deployment is introduced.

## Evidence

- Verification commit: `c93de0eb5ce7789559cf71df8a3273be4681824e`
- Base commit: `b35acaf0804f84a2c56fcee4b3ca1504cf74c95e`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## Observed problem

Parallel Chough branches repeatedly conflicted on `.chough/session.json`. Merge commit `b2e2f19d904aec2a5acd73569c8439f5583c01f9` names that file among its conflicts and its resolution manually combined checkpoint history while selecting current-cycle values. Merge commit `e31fa7c768625b8cb2c2b1b12ce77c195deac99d` records another hand resolution and explicitly says that it kept the branch checkpoint. Merge commit `14b4e7f508b9701a7fbf19fb752c8176a66293b4` again names `.chough/session.json` as conflicted. These are observed integration costs, not a hypothetical formatting issue.

The file mixes two kinds of data. Decisions, completed work, questions, observations, bugs and references behave like exact-string collections that can often combine by membership. Objective, stage, native change, consents, blockers, awaiting-human gates and related scope fields describe one current cycle. Selecting different fields from different branches can create a cycle that neither branch represented, so those fields need atomic conflict handling.

## Constraints and prior decisions

REQ-cli-007 keeps one canonical schema 2 checkpoint at `.chough/session.json`. Schema 1 remains readable for context and status and migrates only through the explicit backup-preserving command. This change must not add a journal, shard directory, second writable store, runtime, service or duplicate lifecycle state. Native SpecSync remains the lifecycle authority, and checkpoint merges must not infer transitions, approvals, reviews, consents or human gates.

The selected design is a bounded `chough checkpoint --merge <base> <ours> <theirs>` command plus a `.gitattributes` entry naming `chough-session`. Each clone must configure its own Git driver. Initialization must not mutate Git configuration, and the explicit command remains available when no driver is configured.

Implementation must wait for fresh failure-first evidence from a real Git merge at the unfixed revision, linked to a local bug. A later pass must use the same kind of fixture. Evidence uses readable IDs without allocating a case number. No existing checkpoint, transition, case, bug, observation, HI, requirement or SpecSync ID scheme changes.

The main accepted limitation is that independent current-cycle edits can remain unresolved. The command must refuse those edits rather than guess. Driver setup is intentionally per clone, so a clone without setup retains ordinary Git behavior until the user configures it or invokes the fallback.

## From the change's design.md

# Design

## Command and Git integration

Add `chough checkpoint --merge <base> <ours> <theirs>` with exactly three path operands. The merge option is mutually exclusive with every existing checkpoint update option. It reads only the three supplied files and never reads or writes the canonical session merely because `--root` names a project.

The command follows Git's merge-driver convention: base is `%O`, ours is `%A`, and theirs is `%B`. On a compatible merge it replaces only ours and exits zero. On a refusal it exits nonzero with a named diagnostic and does not insert conflict markers. The repository declares:

```gitattributes
.chough/session.json merge=chough-session
```

The documentation gives this per-clone setup command:

```sh
git config merge.chough-session.driver 'chough checkpoint --merge %O %A %B'
```

Initialization does not run this configuration command or otherwise mutate Git config. Without the driver, the user can run the same explicit command on three files. The command performs no shell evaluation and introduces no background process, runtime or service.

## Input and path safety

Each operand must be an existing regular file whose final component is not a symbolic link. Read each input as bounded UTF-8 with the existing 65536-byte structured-record ceiling. The three resolved paths must be distinct so the read-only base and theirs operands cannot also be the output path.

Git can place `%O`, `%A` and `%B` outside the project and can use absolute temporary paths. Therefore these command operands must not go through the project-relative `local_path` rule. This exception applies only to the three explicitly named merge files. Existing validation of `references.design`, `references.files` and every managed project path remains unchanged.

Only ours is writable. After all parsing, merging, serialization and output validation succeed, write the complete result to a create-new sibling temporary file, sync it, and rename it over ours. Do not create parent directories. Remove the temporary file on every pre-rename failure. Base and theirs are never opened for writing. No other persistent file or record is created. The transient sibling is renamed over ours on success and removed on every pre-rename failure.

This is a Git merge operation, not a lock or a new policy for concurrent live checkpoint writers. Existing one-owner live writes remain the rule.

## Exact-string collection algorithm

Apply the same ordered three-way membership merge independently to `references.criteria`, `references.requirements`, `references.cases`, `references.design`, `references.files`, `decisions`, `open_questions`, `completed`, `observations` and `bugs`.

Strings compare exactly and case-sensitively, with no trimming or normalization. Interpret each input collection by membership and emit each retained string at most once. For a string present in base, retain it only when it is present in both ours and theirs. Thus a removal on either side is preserved when the other side is unchanged. For a string absent from base, retain it when either side contains it. Thus one-side additions survive, and identical additions from both sides appear once.

Construct the output order as follows:

1. Walk base order and emit each base string retained by the membership rule.
2. Walk ours order and emit each retained string that was absent from base and has not already been emitted.
3. Walk theirs order and emit each retained string that was absent from base and has not already been emitted.

For example, base `[a, b, c]`, ours `[a, c, o, shared]`, and theirs `[b, c, t, shared]` produce `[c, o, shared, t]`. The one-side removals of `a` and `b` survive, base order leads, ours-only additions precede theirs-only additions, and the identical addition `shared` appears once.

This rule does not reorder retained base entries to match either branch. Reordering alone has no membership effect. If a later requirement needs order editing as distinct behavior, it needs a separate contract rather than an implicit extension of this merge.

## Atomic current-cycle group

Treat this complete tuple as one value:

- `objective`
- `intent_scope`
- `modules`
- the complete `stage` value, including status, callers and iteration
- `native_change`
- `next_action`
- the complete ordered `run_consents` collection
- the complete ordered `blockers` collection
- the complete ordered `awaiting_human` collection

Compare typed values, not JSON whitespace. Ordered collections and nested stage values compare in full. Let `ours_changed` mean the ours tuple differs from base, and define `theirs_changed` likewise. Select base when neither side changed, ours when only ours changed, theirs when only theirs changed, and either identical tuple when both sides made the same change. If both sides changed and the tuples differ, refuse with a `current-cycle conflict` diagnostic naming the differing member fields.

There is no field-by-field fallback. For example, one branch changing only `next_action` and another changing only `native_change` is still a conflict because selecting both would manufacture a cycle that neither side stored. A union of consents, blockers or human gates is equally invalid.

## Validation, refusal and side effects

Every input must independently parse and pass the existing strict schema 2 checkpoint validation. Schema 1 in any position, mixed schema versions, unsupported versions, malformed JSON, wrong types and unknown fields at any level fail closed. The merge command neither migrates schema 1 nor creates a schema 1 backup.

After constructing the candidate, run full schema 2 validation before serialization. Serialize in the existing pretty JSON form with one trailing newline, parse and validate those output bytes again as schema 2, and enforce both the existing serialized checkpoint validation bounds and the 65536-byte emitted-record ceiling. This includes module, caller and collection limits. In particular, a valid pair of additions whose union exceeds 200 entries or whose result exceeds a byte bound is a refusal, not a truncated merge.

Perform no write until every step succeeds. Every refusal leaves base, ours and theirs byte-identical and creates no transition, gate, approval, review, consent, migration, observation or bug record. Successful selection or merging also creates none of those records. In particular, merged awaiting-human values do not open or close gates, merged consents do not authorize anything new, and merged stage values do not infer a transition. The resulting checkpoint remains data for subsequent normal validation and native authority checks.

## Compatibility and migration limits

The schema remains 2 and the canonical path remains `.chough/session.json`. Existing schema 1 context reads and explicit migration behavior are untouched. No shard, journal, cache or derived checkpoint is retained. No case number is allocated, and no checkpoint, transition, evidence, artifact, validation, snapshot, case, bug, HI, requirement or SpecSync identity changes.

## From the change's testing.md

# Testing

This is the verification plan, not a record that these checks have run.

## Failure first and later pass

1. Before any `src/` implementation edit, add the checkpoint merge regressions to `tests/stages.rs`. Include a real Git fixture with a committed schema 2 base, two branches and `.chough/session.json merge=chough-session`. Configure the same per-clone driver that the documentation gives, with the built `chough` on Git's PATH. Give ours and theirs compatible collection edits, including a one-side removal and an identical addition, while leaving the current-cycle group unchanged. Assert the exact merged bytes and clean Git status.
2. At the unmodified tip, first run the same branch fixture with no Chough driver configured and retain the current ordinary Git conflict or incorrect merged bytes. Then run the new regression with its driver configured and retain its failed assertion or unsupported `--merge` failure. Put both commands, exit statuses, stderr and resulting checkpoint bytes in `.chough/artifacts/checkpoint-merge-real-git-pre-fix.log`. The ordinary Git reproduction is required; an unsupported flag alone is not evidence of the current conflicting behavior. Record the actual failure with the existing observation schema and no case link:

   ```sh
   chough evidence add checkpoint-merge-real-git-pre-fix \
     --criterion FLOW-13 --criterion LIFE-3 --requirement REQ-cli-007 \
     --contract specs/cli/cli.spec.md --kind product-test --outcome failed \
     --environment local-macos-arm64 --method automated --track functional \
     --artifact .chough/artifacts/checkpoint-merge-real-git-pre-fix.log \
     --summary "Real Git cannot yet merge compatible parallel schema 2 checkpoint changes"
   ```

3. Run `chough bug search "checkpoint parallel merge"`. Unless an existing record is the same defect, use `chough bug add` with title, symptom, trigger, expected and actual text from this reproduction, `--environment local-macos-arm64`, both criteria, REQ-cli-007, `--classification product-defect`, `--scope in-scope` and `--observation checkpoint-merge-real-git-pre-fix`. Link the observation before implementation and use the returned bug ID. Do not allocate a legacy numeric ID or a new case ID.
4. After implementation, rerun the same unchanged real Git regression and save its log as `.chough/artifacts/checkpoint-merge-real-git-post-fix.log`. Record `checkpoint-merge-real-git-post-fix` with the same links and method but outcome `passed` and a summary of the clean compatible merge. Append it to the same bug with `chough bug update <returned-id> --observation checkpoint-merge-real-git-post-fix --status verified --note "The unchanged real Git regression passed after the merge implementation"`. The pass must be observed strictly later than the latest linked failure. Never rewrite the failed observation or infer a pass from a structural check.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-007 | For every exact-string history and reference collection, use a base with retained and removed entries, ours with a removal plus additions, and theirs with different additions. Assert surviving base order, then ours-only order, then theirs-only order. An identical addition on both sides occurs once, a one-side removal stays absent, and strings that differ by case, whitespace or punctuation remain distinct. Cover references to criteria, requirements, cases, design and files, plus decisions, open questions, completed work, observations and bugs. |
| REQ-cli-007 | Exercise the current-cycle group as a unit. A one-side change to each of objective, intent scope, modules, stage, native change and next action selects that side's whole group. Identical two-side changes pass. Disjoint edits on the two sides, even to different members, refuse with a current-cycle conflict diagnostic naming the differing members rather than combining fields. Additive collections still merge when the selected current-cycle group is unchanged. |
| REQ-cli-007, REQ-cli-013 | One-side and identical changes to `run_consents`, including complete consent fields and timestamps, are retained exactly with the selected group. Divergent consent changes refuse. One-side and identical blocker or awaiting-human changes behave the same way, and conflicting gate changes refuse without appending gate-opened or gate-closed transitions. Merge never grants a new consent, writes a standing preference or authenticates a human decision. |
| REQ-cli-007 | For every refusal, hash base, ours and theirs before and after and snapshot the surrounding `.chough/` records. All input hashes stay equal and no transition, evidence, bug, preference, approval, review or consent record appears. Successful explicit merges replace only `ours`; base, theirs and unrelated records stay byte-identical. The canonical live checkpoint remains `.chough/session.json`, with no second store or duplicate native lifecycle state. |
| REQ-cli-006, REQ-cli-007 | Put schema 1 in each input position and mix schemas 1 and 2 in both directions; every attempt refuses without migration or a schema 1 backup. Also refuse unsupported versions, malformed JSON and unknown top-level or nested fields in each position. Existing schema 1 context and status reads, explicit migration backup preservation and stage-command migration refusal remain green. |
| REQ-cli-006, REQ-cli-007 | Refuse an input over 65536 bytes in each position. Separately use individually valid inputs whose additions would exceed 200 entries or whose pretty-serialized merged result would exceed 65536 bytes. Refusal occurs before any write. Valid results at the existing bounds pass, and existing module, caller, ID, path and stage validation remains enforced. |
| REQ-cli-007 | In the real Git fixture, the compatible merge uses `.gitattributes` and the actual configured driver, produces the same result as the explicit command and leaves no Chough side-effect record. A second real merge with divergent current-cycle groups exits nonzero, leaves the working checkpoint equal to ours and reports the path unresolved. Git's own merge metadata is expected, but Chough writes no record. After `git merge --abort`, invoke `chough checkpoint --merge <base> <ours> <theirs>` directly on the same three fixture copies to prove the no-driver fallback and its refusal atomicity. |
| REQ-cli-001, REQ-cli-007 | In `tests/cli.rs`, initialize a real Git repository with the attributes declaration, hash `.git/config`, run `chough init`, and assert the config bytes are unchanged and local `merge.chough-session.driver` remains unset. The test separately configures the driver for the real merge, so documentation setup is not confused with init behavior. |
| REQ-cli-016, REQ-cli-019 | In `tests/package.rs`, install the retained pre-change `references/checkpoint.md`, assert its recorded LF SHA-256 digest, require `known-outdated`, update it to the new bundle and verify an edited old copy remains byte-identical. `chough skill status` reports the canonical `.agents/` and `.claude/` copies current after refresh, and the existing no-dash and complete-bundle checks pass. |

## Focused, full and CI-matching checks

Run `cargo test --locked --test stages`, `cargo test --locked --test cli` and `cargo test --locked --test package`. The stages invocation includes both real Git tests and must not use a simulated three-way text merge. Run the pre-fix and post-fix observations in the order above, then `cargo test --locked` and `cargo build --locked --release`. Where the declared minimum toolchain is installed, also run `cargo +1.88.0 test --locked`; otherwise leave the hosted MSRV result unclaimed.

With HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2, run `fledge run fmt`, `fledge run lint` (Clippy over all targets with warnings denied), `fledge run test`, `fledge lanes run verify` and `fledge lanes run pre-push`. Verify includes the configured format, lint, test, HI, strict SpecSync, native audit, case-definition, exercise and build steps. Report hosted Ubuntu, macOS, Windows and MSRV checks only from their actual runs.

## Evidence boundaries

The two readable observation IDs are instances of the existing evidence scheme. No TC-cli number, case definition, observation schema or other identity format is added or changed. The pre-fix failure may become historical after implementation and remains reproduction evidence. Only the later real pass can verify the linked bug. Structural success, a local bug status and an actor label remain distinct from human acceptance, native finalization, merge, release and deployment.

## Where these lessons go

- `specs/cli/context.md`
