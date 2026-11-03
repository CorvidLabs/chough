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
