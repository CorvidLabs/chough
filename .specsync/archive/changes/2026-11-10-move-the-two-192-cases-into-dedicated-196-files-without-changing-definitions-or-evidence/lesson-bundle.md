# Lesson bundle: move-the-two-192-cases-into-dedicated-196-files-without-changing-definitions-or-evidence

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Move the two #192 cases into dedicated #196 files without changing definitions or evidence
- **Kind**: Refactor
- **Paths**: .chough/cases/cli.change-steps.cases.toml, .chough/cases/history.direct-writes.cases.toml, .chough/cases/cli.case-split-196.cases.toml, .chough/cases/history.case-split-196.cases.toml, knowledge/INDEX.md, .chough/sessions/chough-196-case-files.json, .chough/history/cycles/CYC-00f626375dc00fca193eb5c5/
- **Acceptance**: Move only TC-cli-change-new-no-spec-change from .chough/cases/cli.change-steps.cases.toml to .chough/cases/cli.case-split-196.cases.toml and TC-history-timing-skips-direct-writes from .chough/cases/history.direct-writes.cases.toml to .chough/cases/history.case-split-196.cases.toml. Preserve both raw case blocks, all fields/IDs/bindings/links and the entire catalog count, ID set, normalized definitions and actual case digests. Keep every other case, runtime/test source, canonical contract/requirements, HI, policy, accepted/inherited records and evidence files unchanged. Actual pinned chough cases check and library index --check pass after only supported own knowledge regeneration. Prove the preservation oracle discriminates missing/altered case data in private controls. Finish a clean committed local source and honest exact-tool results; no Rust binding/full/site execution is claimed without a separate lease, and no approval/review/finalize/delivery is performed by this author.

## Evidence

- Verification commit: `aac844ae4e40780dcb93973a4077acaaed6cda07`
- Base commit: `4bbd5a6373d26c0ad069878f8f543842a4564a6f`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

## Concrete problem and source

Live Chough issue196, filed by0xLeif and explicitly assigned by coordinator t19f, asks to move two cases appended by192 into files of their own under existing main AGENTS.md/DRIVE-12. The exact live base is4bbd5a6373d26c0ad069878f8f543842a4564a6f; GitHub has14 open issues and0 open PRs at preparation. PR198 is merged and is not recreated.

TC-cli-change-new-no-spec-change currently lives in cli.change-steps.cases.toml. TC-history-timing-skips-direct-writes currently lives in history.direct-writes.cases.toml. This is file ownership hygiene, not a test-semantic or runtime repair.

## Authority and ownership

Actor E/t221 prepares and later implements only196 in the isolated ch-queue-196-case-files worktree. Authority source is t19f NEW AUTHORIZED ASSIGNMENT relaying Leif's request to work through all Chough PRs/issues. This relay is recorded as its source, not invented verbatim human acceptance. Root retains native approval/review/finalize/merge; author waits for the exact scope receipt before any case edit. Scope authorization does not record genuine human implementation acceptance/review or delivery. No additional permission is requested from Leif for already-authorized bounded work.

One writer per change; lifecycle ownership transfers explicitly for root approval. Existing de31/153/147d, archives, inherited sessions/evidence/native changes and all other workers are preserved. No compile/full/site without a separate root lease; no GitHub mutation/push/merge/install.

## Current evidence and uncertainty

The unchanged actual pinned Chough cases list and cases check pass before implementation. The full catalog/digests and source snapshots are external under program/evidence/chough-queue-work. The native forwarding route is an exact shell exec wrapper to immutable SpecSync e971, not a symlink; initial pre-write guard failure is retained. Own session creation moved no inherited sessions; preservation guard passed. No implementation pass, Rust binding execution or human gate is inferred from this preparation.

## From the change's design.md

# Design

## File mapping

| Existing case ID | Old file | New file |
| --- | --- | --- |
| TC-cli-change-new-no-spec-change | .chough/cases/cli.change-steps.cases.toml | .chough/cases/cli.case-split-196.cases.toml |
| TC-history-timing-skips-direct-writes | .chough/cases/history.direct-writes.cases.toml | .chough/cases/history.case-split-196.cases.toml |

Each new file retains profile chough-testing/1 and its original module. Every original case ID appears exactly once globally. The unchanged parser/digest code continues to load definitions; source filename changes while semantic case digests, bindings and evidence links remain unchanged. Canonical requirements are already satisfied by the same cases; no delta or permanent HI/REQ ID is created. This is nonvisual, so no visual-design artifact/approval is needed. No public CLI/API or persisted case schema changes.

Knowledge/INDEX.md may be regenerated using the supported command after own bookkeeping; all actual generated paths must be enumerated. Only this change's session and cycle plus native workspace are written. Inherited source/records remain exact.

## From the change's testing.md

# Testing

## Baseline and positive controls

Before editing, the actual immutable Chough helper returns a valid full cases list/digest catalog and cases check0. Save every TOML case by ID/module and every field, the count/ID set, raw moved blocks and evidence/accepted-record byte-mode map. After moving, require exact equality of the full semantic catalog and actual case digests; require exactly the planned two source-file mappings and no duplicates. No other case content may change. Compare src/tests/config/contracts/HI blobs to base unchanged.

## Nonvacuous negative controls

Use dedicated private copies of the baseline catalog/proposed data only: omit one named case and alter a named expected-outcome/binding field. The unchanged preservation oracle must reject each while unchanged and correctly relocated controls pass. No mutation touches original fixtures, case files or records. Actual current-ID/global uniqueness and module checks run through Chough, independently of the mechanical extraction.

## Commands and proof limits

Run pinned chough cases check and chough library index --check after supported regeneration. Both must actually return0; no empty selection or speculative pass. Capture exact argv/cwd/HEAD/tool paths/versions/hashes and raw failures inside named HC helpers. No additional repository tests are introduced for this low-impact data move. The existing cargo-test bindings and their exact assertions remain unchanged but are NOT rerun without a compile/heavy lease; original archived passes are historical and never relabelled as new execution. Full/site/Trust/hosted CI/human review/delivery remain separate. Optional native structural verification is not runtime proof.

## Actual allocated execution

Source implementation committed clean at bb699104eb37f45d8cb9481685ae6a659900fbb8. Actual immutable Chough reader preserves all400 IDs/fields/digests, only the two allocated file mappings differ; independent TOML full-definition equality passes. Actual cases check0. Same physical-file preservation oracle returns intact0, missing-case1 and altered-expectation1; actual Chough reader returns structurally valid399/400 and a changed semantic digest for the mutation, proving the separate oracle active. No original data mutated by controls. Supported knowledge generation changed only knowledge/INDEX.md; actual library index --check0. Five protected local candidate trees retain their heads/statuses/typed nonbuild bytes/modes. No src/tests/contracts/HI/policy alteration, Rust binding execution, compile/full/site/CI or human acceptance/review/delivery was performed. Final own native structural verification is separate from this nonvacuous catalog preservation proof.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
