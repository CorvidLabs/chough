# Lesson bundle: bound-transition-history-and-preserve-referenced-proof

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Bound transition history and preserve referenced proof
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/main.rs, src/history.rs, src/checkpoint.rs, src/stage.rs, src/records.rs, src/timing.rs, src/evidence.rs, tests/history.rs, tests/stages.rs, tests/timing.rs, tests/evidence.rs, tests/next.rs, tests/package.rs, tests/common/mod.rs, docs/architecture.md, docs/cli.md, docs/FRD.md, docs/ROADMAP.md, CHANGELOG.md, skills/chough-workflow/references/checkpoint.md, src/skill.rs, .chough/cases/cli.cases.toml
- **Acceptance**: Legacy logs and recorded proof remain byte-identical; explicit adoption writes independent immutable transition events with a cursor in the sole checkpoint; compatible branch events integrate without loss and divergent current-cycle cursors refuse; normal reads and per-cycle storage have tested byte and event limits; timing keeps lineages separate and reports missing or pruned history as partial or unknown; retention previews exact protected and eligible sets and applies only with explicit authorization and fresh protection checks; crash, collision, unsafe-path and interruption fixtures preserve active state; roadmap and backlog mappings distinguish delivered work from open criteria

## Evidence

- Verification commit: `70421962a477ad39423031414ad4cc892b28b4ec`
- Base commit: `825a3805530456005e361dfa6d96d93eee52cd09`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #31 asks for bounded history without losing resumability, independent branch events or proof. 0xLeif said "Ok continue" after the recommendation to reconcile stale tracking, prepare this definition and present scope approval before implementation. That is authorization to prepare this package, not approval of its proposed persisted format or policy.

Baseline: origin/main 825a3805530456005e361dfa6d96d93eee52cd09, merged PR #36. Main CI and Push on main both completed successfully. There were no open PRs at pickup. This definition is on 0xleif/feat/bounded-history-definition.

Existing intent: LIFE-2, LIFE-3, LIFE-6, LIFE-7, LIFE-8 and LIFE-10; FLOW-2, FLOW-7 and FLOW-13. No HI criterion is added or reworded. The single checkpoint and its current-cycle merge group from PR #29 remain authoritative. SpecSync owns approval, verification, review and archives. Current-agent-first remains selected. Rune, APS and a second writable checkpoint remain outside scope.

Prior acceptance remains unchanged: PR #35's cold-start proof failed; PR #36's Kimi baseline, change and real-repository runs were not run by 0xLeif's choice. This package requests no substitute grade and changes no recorded evidence or frozen archive.

Read before drafting: AGENTS.md, INTENT.md, hi/life.md, docs/FRD.md, specs/cli/cli.spec.md and companions, the workflow stage, plan and checkpoint references, and specs/cli/context.md lessons. The latest recorded failure is OBS-product-test-bfc159865a39075249fb6e40, linked to the verified identity-main librarian bug. Source lessons require bounded fixtures, explicit unknowns, source-linked proof and closeout before native verification.

## Implementation observations

2026-11-02, materialization order (observation): SpecSync 6.1.2 refused `change check` while implementation tasks were open, before adding the new requirement IDs that bound case validation needs. The drafted last task mixed those prerequisites with post-materialization case/lane execution. Resequence the task checklist honestly: finish implementation and Rust regression checks, materialize through the native owner, then execute all bound cases and the mandatory verify lane as explicit remaining obligations. No check is marked passed before it runs, and no acceptance criterion or approved delta is changed. Source: the native refusal and the existing specs/cli/context.md closeout-order lesson. Scope: this native workflow version; revisit if SpecSync adds separate materialization.

2026-11-02, cleanup ordering (observation): an intermediate development fixture reported one removal where two eligible removals were expected because deleting a newer cycle first hid an ancestor needed by fresh protection checks. Source: .chough/artifacts/history-retention-ordering-development.log. Limits: that intermediate source identity was not retained, so this log is development evidence, not historical native qualification. Current code orders candidates by selected ancestry from oldest to newest; the real Git and retention fixtures assert retained hashes and explicit checkpoint selection. Future action: preserve causal links needed by revalidation and keep timestamp ties protected; lexical/random identity order is not chronology.

Integration update: approved main d839fef (#47) arrived after the9d02 qualification. Its unchanged setup/hint inputs are imported under the archived upstream change. Earlier exact-source results remain retained; final qualification must cover this new combined source before closeout. No new product policy or human decision is inferred.

## From the change's design.md

# Design

## Proposed persisted format

Keep .chough/session.json as the only writable checkpoint. Add an optional history_cursor containing a cycle ID and head event ID; it participates in the current-cycle merge group. No second mutable head/index is authoritative.

New transition events live under .chough/history/cycles/<cycle-id>/<event-id>.json. Cycle and event IDs use independent 96-bit random suffixes, one candidate and create-new refusal, with no fallback or retry. An event envelope has a version, its IDs, parent ID, native-change context, and the unchanged schema-1 transition payload. A cycle root records its predecessor cycle as a historical link, not permission to join independent timelines. Parent links determine order within a lineage; time fields determine durations. Timestamps and random suffixes never establish causality across branches.

A new project initializes this history format. An existing project stays in legacy mode until chough history adopt --apply. Without --apply, adopt is a read-only preview. Apply verifies the legacy file, records its digest, byte length and line count as an anchor, and writes the optional cursor atomically. It leaves transitions.jsonl and session.v1.json byte-identical. Repeating adoption verifies the existing anchor and does not duplicate history. Unexpected legacy changes after adoption refuse new writes and produce an actionable diagnostic.

Publish complete event files before atomically replacing the checkpoint and cursor. A failed checkpoint save leaves any published event unselected and visible as an orphan; it cannot advance the checkpoint or count as a completed transition. Gate changes publish their complete event chain before saving checkpoint fields and the final cursor together. Concurrent live checkpoint writers remain unsupported under the existing one-owner rule.

## Branch integration and resume

Independent branch event files have distinct paths and survive Git integration. The chosen checkpoint cursor selects one coherent current lineage. Divergent cursor edits are a current-cycle merge conflict even if stage names happen to match. Other branch histories remain stored and inspectable. A missing parent, conflicting event identity, fork without a selected tip or malformed envelope is partial/unknown, never an invented order. The current stage still comes from the checkpoint and native gates remain authoritative. Reset guidance must not use an unrelated branch's reset event.

A stage reset starts a new cycle and head. Changes of next_action or native-change metadata alone do not create a cycle. timing reports lineages separately and never calculates a stage/gate span across unrelated heads or omitted parents. Legacy timing preserves its existing limitations and labels.

## Proposed bounds

- One event: at most 65,536 bytes, including its envelope.
- Recent guidance: follow at most 128 parent events and read at most 512 KiB; retain existing output caps. No directory-wide event scan for next/status/context/stage show.
- One new cycle: at most 4,096 event files and 16 MiB of event bytes. Check before mutation. At capacity, ordinary writes refuse without changing checkpoint state; a reasoned reset can start a new cycle without deleting the old one. These are local append limits. Git may combine independently written immutable sets above the capacity; retain every imported event, refuse further ordinary appends, and use a reasoned reset for new work. This is not a hard bound on a merged directory.
- A timing request: at most 4,096 events and 16 MiB of transition bytes across selected lineages; reaching a bound is explicit partial history with unknown affected spans.
- Maintenance enumeration: at most 16,384 source files and 64 MiB of metadata per invocation. Exceeding a bound yields an incomplete preview and refuses apply; protection sources are never silently truncated.

These are proposed defaults, tested at their exact boundaries. They bound routine work and per-cycle storage. Total protected proof has no artificial disk cap; it can prevent reclamation. No claim of globally bounded disk consumption is made when proof must stay.

## History retention

chough history prune previews by default. A cycle is closed for deletion only when every retained tip ends at done and its associated native change authenticates as archived, or an explicit reset closed a lineage without leaving an unresolved tip. Incomplete or ambiguous lineage closure protects the whole cycle. Preview lists complete closed cycles, exact file paths/digests/bytes and every protection reason. Preserve the active cycle, unresolved forks/orphans, the newest three closed cycles, cycles closed less than 14 days ago, and anything referenced by checkpoints, bug fixtures/notes, case artifacts, native manifests or frozen evidence. Any unavailable protection source blocks deletion. Never prune the legacy anchor/log, migration backup, cases, observations or native archives through this command.

--apply requires the explicit history prune invocation plus the existing human retention authorization, and rebuilds protection and file identities immediately before removal. The existing retention auto setting does not start history pruning as a side effect of cases or validation. Applying the implementation to this repository requires a later explicit human operation; fixtures use disposable roots.

Publish an immutable receipt under .chough/history/pruned/<cycle-id>.json before deletion. It names every planned path and digest, the reason, authorization source and cycle identity; cap a receipt at 1 MiB and refuse larger plans. A receipt plus absent content means pruned; a receipt with still-present matching content means retained or interrupted. A repeated apply may resume only the identical receipted plan after fresh protection checks; a conflicting receipt or changed file is refused. Unselected orphan files remain protected and do not count as completed events. Missing unlisted content is unavailable. A deletion interrupted midway remains partial, never a completed or successful timeline. Readers do not synthesize duration evidence from receipts.

## Existing evidence retention

Keep existing retention defaults and ledger shape. A new preview-to-apply change to references must protect that set. Changed, missing or unsafe candidate paths cause conservative skip/refusal with explicit results. Shared artifacts stay while any record references them. Revalidate hashes and path identity before removal and never apply based on stale preview output. The one-owner maintenance boundary is documented; arbitrary external filesystem writers are not claimed to be serialized.

## From the change's testing.md

# Testing

These are planned checks, not observed passes. Use controlled native fixtures for deterministic behavior and real native integration for lifecycle qualification. All cleanup and migration tests use disposable roots, never the repository's recorded proof.

| Case proposed | Requirement | Observable assertions |
|---|---|---|
| TC-cli-history-adoption | REQ-cli-025 | Legacy preview changes no byte; adoption preserves log and schema-1 backup; repeats do not duplicate; changed anchors, unsupported envelopes and old-reader compatibility are explicit; failed save preserves checkpoint bytes and leaves new orphans unselected |
| TC-cli-history-branches | REQ-cli-025 | Real Git branches add event files at the same controlled timestamp, preserving both hashes after integration; compatible checkpoint changes merge; divergent cursors refuse as one current-cycle group; timing never spans independent forks; unrelated reset cannot affect current guidance |
| TC-cli-history-bounds | REQ-cli-026 | Exact 65,536-byte event boundary, 128-event/512-KiB recent bounds, 4,096-event/16-MiB cycle and timing bounds, and 16,384-source/64-MiB maintenance bounds; instrument read counts and bytes; cap refusal leaves checkpoint byte-identical and a reset can start a fresh cycle |
| TC-cli-history-retention | REQ-cli-026 | Preview is read-only with exact path/digest/byte accounting; newest three, 14-day, active, unresolved fork/orphan and every reference source protect; protected-only fixture is truthful no-op; authorized apply preserves anchor, migration backup and frozen proof; interrupted receipts distinguish present, pruned and unavailable |
| TC-cli-retention-revalidation | REQ-cli-027 | Reference added after preview protects the observation and cycle at apply; shared artifacts survive; changed/missing inputs, symlinks/traversal, malformed protection sources and interrupted removal never broaden deletion or claim completed cleanup |

Additional checks: force one random-source failure and one identical event/cycle candidate collision; assert one draw and no overwrite/fallback/retry. Inject failure before and after event publication and checkpoint replacement, and before/after receipt publication and each removal. Missing parents, clock ties, descending clocks, malformed envelopes and oversize plans must be partial/unknown or refused. Compare recursive byte/hash inventories for protected files.

The real Git fixture combines a supported history cleanup with a second branch's new evidence and immutable events, then rechecks every retained reference and artifact hash. It also covers unrelated current-cycle edits and leaves unsupported merges actionable instead of inventing a unified history.

Run focused history, stages, timing, next and evidence suites plus existing schema migration, package and identity regressions. Then Fledge fmt, test, lint and lanes run verify using HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2. Run tests appropriate to an implementation; definition preparation does not claim product behavior. Observe hosted Ubuntu, macOS, Windows and MSRV checks only after they actually run. No live adopt/prune or cold-start exercise is authorized here.

## Where these lessons go

- `specs/cli/context.md`
