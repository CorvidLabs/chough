# Lesson bundle: finish-the-bounded-capacity-proposal-with-named-refusals-and-accurate-provenance

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Finish the bounded capacity proposal with named refusals and accurate provenance
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/checkpoint.rs, tests/cli.rs, CHANGELOG.md, hi/life.md, specs/cli/context.md, specs/cli/requirements.md, docs/cli.md
- **Acceptance**: The401-observation refusal explicitly names400 and preserves record bytes; strengthened test fails before repair and passes afterward unchanged. Current HI/contract/context/changelog accurately separate the actual human capacity goal and finish-through-PR instruction from the agent-selected131072/400values. Existing checkpoint-only/list-only isolation and all other bounds/path/symlink/atomic protections remain. Fresh final-source verification covers post-archive provenance edits, and every old record/archive stays byte-exact. Independent review and all exact-head checks precede authorized merge. Import final accepted PR50 before final committed qualification.

## Evidence

- Verification commit: `5e90711d684049095ef34bbd9e13601cdd0393e1`
- Base commit: `dbb66d979e2c34e41926401641d58971c0d65f65`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Capacity proposal final followup

## Actual authority and value selection

Acting agent: corvid-agent, root session. Actual human sources in this conversation:0xLeif standing authorization that changes may be made through PRs; explicit "Merge approved, green PRs and resynchronize"; and the latest direct request "Awesome. Can you finish these PRs up and get them merged and everything?". The existing PR51 also relays the qualitative direction to raise progress-file/test-result capacity. Root selects the already implemented proposal131072bytes/400observation pointers as bounded engineering values within that concrete capacity work. These are agent-selected values, not numbers specified by the human. No approval is inferred from the earlier ambiguous2. Scope permission, implementation review, exact-head green merge and optional release remain distinct.

## Existing problem

Current dbb66d9 code isolates checkpoint bytes at131072 and observation pointers at400, but ensure_ids reports only too many observation for401. REQ-cli-032 requires the named bound. Current provenance corrections in four documents also occurred after old archived acceptance. Preserve that original archive; this new bounded followup owns final code/prose qualification.

## Scope and design

Keep the existing constants and boundary behavior. Include the actual limit in the ID-list diagnostic and assert400without weakening existing tests. Correct current prose to label values as agent-selected under actual standing/finish authority. Import accepted final PR50, preserving all cleanup/indexed history and genuine record-file pins through proper three-way semantics. No second checkpoint/lifecycle ledger.

## Out of scope

New values beyond the existing proposal, any other bound increase, old record/archive edits, forced Git, pruning, cold-start grades, release and deployment.

## Ownership and estimate

Root alone owns this native scope and checkpoint/evidence/closeout. Worker owns only src/checkpoint.rs and tests/cli.rs with no source beyond this contract diagnostic. Small fix plus bounded provenance/integration checks; native/hosted duration remains uncertain.

## Constraints consulted

AGENTS, LIFE-11/3 and existing FRD/REQ-cli-032/007, unchanged generic-record/list/history/output bounds, workflow plan/lifecycle, and context.md source-freshness/final verification lessons.

## From the change's design.md

# Bounded followup design

## Approach

Preserve the proposal constants SESSION_LIMIT=131072 and MAX_OBSERVATIONS=400. Add the actual limit to the existing ID-list refusal and require400in the existing401observation regression. Keep the old message phrase for compatible diagnostics. No path, symlink, atomic writer, input-size, output-size, timeout, history or other-list/record policy change.

## Authority and provenance

The human supplies the capacity goal and latest through-PR completion/merge request. Root adopts the existing proposed values as engineering choices and labels them agent-selected. No human numeric statement or approval is invented. Current prose/HI retains the actual qualitative source while the fresh followup qualifies the final corrected bytes; the old archive and its original verification remain untouched.

## Integration

Import the final accepted PR50 before committed verification through a normal merge. Use proper checkpoint collection semantics, preserve PR50intentional old-note moves, frozen index,200existing observation IDs and all direct file pins, while selecting one coherent root followup cycle. The new implementation permits400observation pointers but does not require using that capacity.

## Verification

Genuine unchanged failure-first diagnostic test, later pass, Fledge9, boundary/resource-isolation tests, independent current-source/record-provenance review, native closeout and all exact-head checks. No structural pass is product proof. Settle task/provenance text before final observations, then commit all records before final check--commit.

## Constraints consulted

AGENTS, existing HI/FRD/REQ-cli-032/007, workflow lifecycle and context.md freshness/ordering lessons; no model client/runtime/service or duplicate lifecycle store.

## From the change's testing.md

# Final verification

Unchanged focused observation-bound assertion failed before the sole diagnostic repair and passed afterward, with genuine separate source/input/output records and linked local bug. Original archive/proof remains unchanged. That pair becomes historical after accepted PR50 integration; final Fledge9/allRust boundary and imported regressions plus3supported actualMac report cases provide new current proof. Other newly curated bounds/CLI/identity cases retain declared Linux/Windows environments, with genuine hosted qualification required. No Mac result is relabelled.

Current agent-selected provenance, canonical capacity contract and task text settle before final execution. Verify the actual400/401diagnostic requirement,131072exact/overflow inputs and other-record/list/path/symlink/atomic invariants. Independent source review confirms unchanged isolation and preserved parent behavior. Independent metadata audit covers proper3way removals, all old literal proof,28incoming record pins and160whole case tables. Record exact source/case/artifact/env identities, pin each actual new record, and link a fresh final pass to the already reproduced diagnostic bug.

Native check enters verifying before audit coverage. Commit all final records before check--commit and write no Chough record between that and consecutive root review/ship. Hosted success/independent current-head approval precede squash merge. No cold-start/humanPass/release/deployment/pruning claim.

## Where these lessons go

- `specs/cli/context.md`
