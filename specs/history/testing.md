# History testing

Product checks run through `fledge run test`; structural checks through `fledge run
spec`; case definitions through `fledge run cases`. Stable case definitions for this
module are in `.chough/cases/history.cases.toml`. TC-history-0050 to TC-history-0061
bind the reference-aware retention integration tests in tests/evidence.rs; the
curated `TC-history-` slug cases bind tests/history.rs and tests/evidence.rs, and
TC-history-legacy-timing-bounds binds tests/timing.rs.
tests/spec_split.rs proves the 2026-11-03 split from cli kept every moved
requirement's wording.

| Requirement | Verification responsibility | Cases |
|-------------|-----------------------------|-------|
| REQ-history-001 (formerly REQ-cli-018) | Mixed-ID retention previews/apply, reference protection, pruning labels, byte accounting and unchanged ledger entries | Existing retention regressions and mixed-ID fixtures |
| REQ-history-002 to REQ-history-004 (formerly REQ-cli-025 to REQ-cli-027) | Adoption, events and cursor; exact bounds and protected whole-cycle retention; revalidated evidence cleanup | See the table below |

## Bounded transition history and protected retention

REQ-history-002 to REQ-history-004 (formerly REQ-cli-025 to REQ-cli-027) are exercised by the following curated automated cases:

| Case | Direct bound regression |
|---|---|
| TC-history-adoption | Explicit adoption preserves original log and migration bytes and repeats without duplication |
| TC-history-branches | Real Git branch events integrate while divergent checkpoint selection remains explicit |
| TC-history-bounds | One bound test executes exact event, cycle, recent-read and maintenance count/byte fixtures |
| TC-history-retention | Read-only preview, authorized apply and newly protected cycle references |
| TC-history-retention-revalidation | Changed, missing and newly referenced evidence/artifacts are retained |
| TC-history-prune-protection-race | A protection appearing between removals restores every removed byte and preserves the new reference |
| TC-history-prune-restores-partial-removal | Real CLI observation-removal failure restores artifacts with exact bytes and permissions |
| TC-history-prune-restores-earlier-sets | A later protected set rolls back every completely removed earlier set |
| TC-history-prune-restore-preserves-recreated-file | Rollback keeps recreated files and preserves uncommitted original bytes in a named hold |
| TC-history-prune-capture-byte-authority | Replacement bytes captured after eligibility validation survive; earlier completed sets restore bytes, records and modes |
| TC-history-prune-unsafe-restore-continuation | File/ancestor symlinks retain writer inode/target; originals survive in a named hold, earlier restores continue and every ledgered set gets restored accounting |

The full Rust suites add unknown legacy payloads, migration, random/collision refusal, failed checkpoint saves and orphan visibility, unsafe/malformed receipts, interrupted cleanup/resume, separate timing tips, and real Git retention alongside another branch's new proof. Package regressions independently hash the prior checkpoint reference and preserve owner edits during skill updates. The mandatory Fledge verify lane also checks existing behavior, native structure, traceability and the real CLI exercise. Observations and current IDs live in the Chough checkpoint and evidence records; this plan does not invent a passing grade or hosted result. No cold-start rerun, real-project adoption or live cleanup is part of this qualification.

## Case ID mapping (2026-11-03 split)

Cases moved from cli.cases.toml with their bindings, paths, environments and
expectations unchanged. `TC-cli-<rest>` became `TC-history-<rest>`, and a repeated
`history-` was dropped: TC-cli-0050 to TC-cli-0061 are TC-history-0050 to
TC-history-0061; TC-cli-history-adoption, -branches, -bounds, -retention,
-legacy-timing-bounds, -frozen-reference-aliases, -closing-reset-age,
-post-prune-reference and -stale-receipt-age are TC-history-adoption and so on; and
TC-cli-retention-revalidation is TC-history-retention-revalidation. Earlier
observations and bug records keep the old IDs as history.

## PR #197 corrective boundaries

The two new cases live in `.chough/cases/history.pr197-p1.cases.toml`. Their
executable failure-first regressions are in src/evidence.rs, alongside the prior
race and private-mode tests. The capture test exercises both a first candidate
and a later candidate after an earlier set was completely removed. The unsafe
restore binding executes both final-file and ancestor symlink variants and checks
writer inode/target, recovery bytes, earlier record/artifact permissions and two
restored ledger events. The same bindings must pass after correction; controlled
mutations independently remove each correction to verify the assertions detect it.
The old archive/review remain historical; corrected code needs fresh acceptance.
