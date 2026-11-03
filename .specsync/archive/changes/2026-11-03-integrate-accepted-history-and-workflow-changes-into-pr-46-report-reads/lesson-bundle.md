# Lesson bundle: integrate-accepted-history-and-workflow-changes-into-pr-46-report-reads

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Integrate accepted history and workflow changes into PR 46 report reads
- **Kind**: Operations
- **Specs**: cli
- **Paths**: src/main.rs, src/evidence.rs, src/skill.rs, tests/evidence.rs, tests/next.rs, tests/package.rs, tests/common/mod.rs, .chough/cases/cli.cases.toml, .chough/session.json, .chough/transitions.jsonl, CHANGELOG.md, docs/FRD.md, docs/ROADMAP.md, docs/architecture.md, docs/cli.md, skills/chough-workflow, .agents/skills/chough-workflow, .claude/skills/chough-workflow, specs/cli/cli.spec.md, specs/cli/context.md, specs/cli/requirements.md, specs/cli/testing.md
- **Acceptance**: Accepted history and workflow behavior and all approved requirements coexist with request-local report reads and typed error diagnostics. Preserve immutable proof, the PR46 atomic current-cycle group, additive collections and real transition occurrences. Reconcile monotonic canonical version metadata. On final combined source, required Fledge verification, Rust1.95 and Rust1.88 tests and applicable history, report and workflow cases pass. Actual merged44 source identity is checked before closeout; native verification is current and root alone records review and ship.

## Evidence

- Verification commit: `9bea9ddf2f1abdba0426abdbc815792b1a5f94dc`
- Base commit: `d23ef42534b8d1d59229d4e84d8274072dbd6f74`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

PR46 report reads were accepted, archived and qualified at d23ef42. PR37 history was accepted, archived and merged as a788869. PR44 workflow is separately accepted and archived; root supplies its final qualified tip before final integration. This new operations scope owns only their bounded reconciliation and qualification. Existing semantic requirements remain unchanged.

Human SOURCE: 0xLeif instructed "Merge approved, green PRs and resynchronize" after the prior standing authorization "And I already gave it some approval and told it can do any changes. It just needs a PR." Actor corvid-agent records the operation under that source; the label is not human authentication. Root remains the sole implementation review and ship writer.

Applicable sources: AGENTS.md, INTENT.md, LIFE-1/2/3/6/7/8/10 and FLOW-5/6/7/10/13, docs/FRD.md, approved REQ-cli-025/026/027/028/029/030/031, and specs/cli/context.md diagnostic, evidence ordering and predecessor bundle lessons. No cold-start result is added.

## From the change's design.md

# Design

No architecture or persisted schema is added by this scope. Preserve the accepted history cursor, immutable event envelopes and fail-closed retention semantics. The report cache remains ephemeral and root-bound per request. Native tools retain lifecycle authority and typed diagnostics. The checkpoint remains the only current-state store; current-cycle fields stay atomic while additive histories are merged. Imported archived implementation ownership is retained; actual reconciliation has this active native owner.

## From the change's testing.md

# Testing

Initial37 integration: run focused report case-read/invalidations/trace/typed-error regressions and history tests, plus format/lint as needed. Do not run full Fledge, MSRV, final cases or committed verification until root provides final44 source.

Final combined source: run all nine mandatory Fledge verification steps, all tests on Rust1.95 and Rust1.88, and applicable bound history, report and workflow cases. Use actual execution counts. Compare immutable archives, evidence, artifacts and cold-start blobs from every parent byte-for-byte. Check requirements, checkpoint collections and transition occurrence preservation. Compare final44 main squash source identity with its premerged qualified tip. Refresh native checks and audit; root alone records review/ship. Hosted checks are separate after an authorized push. Preserve failures without weakened assertions.

## Where these lessons go

- `specs/cli/context.md`
