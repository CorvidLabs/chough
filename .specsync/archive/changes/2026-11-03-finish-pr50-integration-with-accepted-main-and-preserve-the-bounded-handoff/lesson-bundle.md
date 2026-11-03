# Lesson bundle: finish-pr50-integration-with-accepted-main-and-preserve-the-bounded-handoff

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Finish PR50 integration with accepted main and preserve the bounded handoff
- **Kind**: Operations
- **Specs**: cli
- **Paths**: src, tests, skills, .github, docs/cli.md, CHANGELOG.md, specs/cli
- **Acceptance**: Actual main747a and published PR50 parent671 are ancestors; accepted report reads, two-worker archive executor, identity acceptance and CLI fixes all remain. Proper three-way checkpoint collection semantics preserve all old evidence, archives, human/binding text,24direct pins and200observation IDs;10superseded coordination and41completed notes move verbatim into pinned immutable raw/indexed proof. Final checkpoint remains at most65536bytes/200IDs and all other existing bounds. Genuine final-source tests/cases and artifact/input hashes are recorded after native task text settles; new observations use exact record-file pins. Independent review and all exact-head checks pass before authorized merge. No capacity raise or LIFE-11 work here.

## Evidence

- Verification commit: `4a635a5287bdee7d0b98b5034193dc586587db3d`
- Base commit: `6715944ff6c4d0423f59e38d76a9cba6ea89e708`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# PR50 final integration and bounded handoff

## Actual authority

Acting project agent: corvid-agent, root session. Actual human sources:0xLeif said "I already gave it some approval and told it can do any changes. It just needs a PR.", "Can you finish up all these?", "finish up ur prs and then check everyhting", explicitly "Merge approved, green PRs and resynchronize", then "get the 3 prs merged?" in this conversation on2026-11-02. Those authorize this bounded integration, proof-preserving cleanup, qualification and green reviewed merge. Agent GitHub comments coordinate ownership and do not create human authority. No specific new capacity values are approved or inferred from "2′". Existing through-PR implementation authority is distinct from the final independent review and green merge gates.

## Goal and approach

Import accepted main747a12cc6fc396f0e8646fee1f2801e02b7d3035 into PR50 parent6715944ff6c4d0423f59e38d76a9cba6ea89e708 with normal Git merge. Preserve current main report reads, identity hooks/tests and CI checker; preserve reviewed CLI fixes and their tests. Canonical main25 remains authoritative.

Use exact raw checkpoints from base16fe55d, parent671 and main747a. The owner-selected projection moves only the reviewed10obsolete coordination strings and41completed work notes verbatim into .chough/artifacts/pr50-checkpoint-coordination-index/policy.json, with full source indices/hashes and frozen raw inputs. Preserve every other decision, all human sources/limits, all200active observation IDs and all24existing direct record pins. The measured prototype is58567bytes; revalidate actual live cycle and final refs. Each genuine new observation gets its own exact .chough/evidence/<id>.json file reference, protecting its record and artifacts.

## Out of scope

PR51capacity/LIFE-11, changed reader/output/time/item limits, source evidence rewrites, archive edits, cleanup/prune application, new cold-start proof, release and deployment.

## Ownership and estimate

Root owns integration coordination, native artifacts, checkpoint/projection/evidence, review/ship and guarded publication. A worker may own only declared Rust/example/doc merge conflicts. All workers preserve other edits. Relative size: small source adaptation, moderate provenance/qualification work; native/hosted latency is uncertain and no target duration is claimed.

## Constraints consulted

AGENTS, INTENT, LIFE-1/2/3/6/7/8, FRD, cli contract/companions, workflow plan/lifecycle, context.md lesson about final verification covering Chough records, source-working-content freshness, and the prior raw/refused/selected projection proofs. Bare union and the decision-only projection are rejected as over-bound or without practical headroom.

## From the change's design.md

# Integration design

## Approach

No visual or interaction redesign. Merge accepted CLI fixes with accepted report Reads, identity test hooks and bounded two-worker archive execution. Preserve current main canonical25 and all parent requirement bodies.

## Data and ownership

The only live Chough checkpoint remains .chough/session.json. The indexed artifact is immutable historical provenance, with exact raw inputs, literal old-note texts, indices and hashes; it is not another current ledger. SpecSync remains the only lifecycle owner. Select one coherent current-cycle group before the helper merges ordered collections.

## Bounds and safety

Keep65536bytes and200observation IDs. Preserve all existing IDs and24record pins. New observations use exact record-file pins; this protects record and artifacts without changing their revision, source identity, result, method or freshness. All other list/output/history bounds remain unchanged.

## Alternatives

Raw union is rejected because it resurrects24intentional removals. Decision-only cleanup is rejected because it leaves insufficient headroom. Capacity raise is a separate PR51 choice. Old evidence relabelling and archive edits are rejected.

## Constraints consulted

Accepted contracts and source, direct-pin protection, independent readonly prototype58567bytes and the current source-content fingerprint rules.

## From the change's testing.md

# Qualification

All source-affecting native artifact/task text settles before actual final execution. Run the full Fledge9lane and three formal report cases that explicitly list actual local-macos-arm64. The unchanged new Grok CLI/identity case definitions list Linux/Windows; their underlying regressions run in the whole Rust suite locally and final hosted Linux/Windows jobs. Do not relabel Mac as Linux or change definitions just to qualify them. Capture real outputs and source/case/artifact identities, preserve all earlier historical records/failures, and pin each new exact record path without changing the200IDindex.

Independent applied review confirms all protected parent Git blobs,157whole case tables, main contracts/HI,267transition occurrences and24direct pins. The exact51literal moves retain all other human/binding text; aliases resolve the original frozen policy paths without editing it.

Ordinary native check enters verifying before active-file audit. After genuine execution, commit all Chough records before final check--commit and write none between that and consecutive root review/ship. Hosted success and exact-head independent approval remain merge gates. No new cold-start, release, deployment or pruning.

## Where these lessons go

- `specs/cli/context.md`
