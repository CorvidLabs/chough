---
change: make-checkpoint-merges-safe-across-parallel-changes
artifact: plan
---

# Plan

## Approach

Prove the current real Git conflict before implementation, then add one bounded three-way command and connect it to Git through an attributes declaration and documented per-clone setup. Keep collection membership separate from the atomic current-cycle group, validate the complete output before replacing ours, and prove both a later compatible merge and an unresolved current-cycle conflict in real Git.

Ownership is ordered by file group. One evidence and regression owner takes `tests/stages.rs` and, only if needed, `tests/common/mod.rs`. The checkpoint implementation owner then takes `src/checkpoint.rs`, `src/main.rs` and, only if the atomic operand writer cannot remain local, `src/records.rs`. The Git and documentation owner takes `.gitattributes`, `docs/cli.md` and the approved checkpoint guidance. The native contract owner takes this change's remaining artifacts and `deltas/cli.md`, with canonical materialization only through the native tool. One records integrator owns all new `.chough/` bug and evidence writes, with no concurrent `.chough/` writers.

The relative estimate is medium. The merge algorithm is small, but fixture behavior is the main uncertainty: real Git temporary paths, driver exit semantics, executable quoting across supported hosts, isolated Git configuration, and proof that every refusal preserves all three inputs. Timing history is advisory only.

## Out of scope

No append-only checkpoint journal, per-change shards, second writable store, automatic schema 1 migration, schema 3, conflict-marker synthesis, field-level current-cycle merge, inferred lifecycle event, automatic Git configuration, new dependency, runtime or service. No new case number or other ID scheme. No edits to frozen archives, no release or deployment, and no claim that structural checks are product proof.

## Steps

1. The evidence and regression owner builds a disposable real Git fixture at the unfixed revision with a base checkpoint and two branches. One branch adds distinct history and reference strings while both also change the current-cycle group. Run ordinary `git merge` without the new driver and retain the observed checkpoint conflict as a failed observation with an explicit readable ID. The records integrator creates or updates the local merge-defect bug and links that failure before any source implementation. Do not allocate a TC number.
2. The checkpoint implementation owner adds the three-operand option and makes it mutually exclusive with existing updates. Implement bounded regular-file reads, schema 2-only strict parsing, the ordered membership rule, atomic current-cycle selection, post-merge output validation and atomic replacement of ours. Keep all lifecycle and canonical-session side effects out of this path. Add focused regression cases for ordering, deduplication, removals, every current-cycle selection branch, mixed independent field conflicts, malformed and unknown input, schema 1 and mixed versions, unsafe operands, collection and byte overflow, and byte-identical refusal behavior.
3. The Git and documentation owner adds `.chough/session.json merge=chough-session`, documents `git config merge.chough-session.driver 'chough checkpoint --merge %O %A %B'` and the explicit fallback, and states that init does not alter Git config. Update command help and affected guidance without implying that a driver is installed automatically.
4. The evidence and regression owner adds real Git tests with isolated global and system configuration and a local test identity. A compatible merge must exit zero, leave no unmerged index entries and produce the exact expected valid checkpoint. A current-cycle conflict must make Git report an unresolved merge, preserve the ours bytes without conflict markers and add no Chough records. Cover direct fallback operation, absolute temporary paths and unchanged local Git config across init. Run these tests against the implementation only after step 1 is retained and linked.
5. The native contract owner completes the remaining selected artifacts and the REQ-cli-007 delta to match the implemented behavior. Use the native SpecSync materialization path at the authorized point rather than hand-editing canonical requirements or contracts. Keep `.chough/` out of native affected paths.
6. Rerun the original real Git scenario after the fix. Record a strictly later passing observation with a different readable ID, link it to the same local bug, and verify that the compatible case passes while the independent current-cycle case still refuses. The records integrator performs these writes sequentially.
7. Run focused Rust tests and affected existing cases, then formatting, Clippy with warnings denied, the full Rust suite, real Git merge tests, `fledge lanes run verify` and pre-push with the CI-matching tool versions. Report only observed results.

Post-verification native review, any requested commit or push, hosted checks and human implementation review remain plain-text lifecycle steps, not task checkboxes or approval recorded by this plan.

## Risks

- A set union would resurrect one-side removals, while last-writer-wins would lose additions. Use the explicit base-membership rule and the ordering example as a known-answer test.
- Independent field selection could combine an objective, native change, consent and human gate from different cycles. Compare the entire group and test two branches that change different members.
- Writing ours before all validation finishes would violate refusal preservation. Keep output construction and revalidation before the only atomic replacement, and compare all input bytes after every refusal.
- Restricting operands to project-relative paths would break real Git temporary files. Test absolute paths while continuing to reject symlink operands and nonregular files.
- A merge driver is local configuration and is not transported by `.gitattributes`. Documentation must make per-clone setup visible, and the explicit command must remain tested as the fallback.
- Clean collection merges can exceed existing collection or byte bounds. Test overflow and refuse rather than truncate or choose one side.
- Real Git tests can accidentally use a developer's global driver or identity. Isolate global and system config and set repository-local test configuration.
- Formatting-only differences must not create semantic current-cycle conflicts, but reordered group arrays are real value changes. Compare typed values and cover both cases.

## Constraints consulted

`AGENTS.md`, `INTENT.md`, FLOW-13 and LIFE-3, current REQ-cli-007, `docs/FRD.md`, `specs/cli/cli.spec.md`, `specs/cli/context.md`, `src/checkpoint.rs`, `src/records.rs`, `.gitattributes`, recent native requirement deltas, and merge commits `b2e2f19`, `e31fa7c` and `14b4e7f`. The design preserves one canonical checkpoint, native SpecSync authority, explicit schema 1 migration, existing bounds, stable IDs and failure-first evidence followed by a later pass.
