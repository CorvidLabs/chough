# Lesson bundle: read-bounded-diagnostic-input-inventories-without-changing-freshness-authority

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Read bounded diagnostic input inventories without changing freshness authority
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/evidence_inventory_compatibility.rs, tests/fixtures/evidence-inventory/OBS-tc-cli-release-corpus-identity-ef1726110e673bd961ec3f56.json, tests/fixtures/evidence-inventory/OBS-tc-cli-release-edited-preservation-0e13b37adb18b0e04c90dd23.json, tests/fixtures/evidence-inventory/OBS-tc-cli-release-host-current-missing-ee55b3fe9e5380440663ed62.json, tests/fixtures/evidence-inventory/OBS-tc-cli-release-legacy-policy-9aaea89ee334873bcfa7b314.json, tests/fixtures/evidence-inventory/OBS-tc-cli-release-name-byte-matching-9218194092c97b28451038d3.json, tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7.json, tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b.json, tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619.json, tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e.json, specs/cli/cli.spec.md, specs/cli/requirements.md, specs/cli/tasks.md, specs/cli/testing.md, specs/cli/cli.3md, CHANGELOG.md, knowledge/INDEX.md, .chough/cases/cli.inventory-compat.cases.toml, .chough/sessions/evidence-inventory-compat.json
- **Acceptance**: Read all nine exact retained schema2 observations with optional snapshot.input_inventory v1 while preserving their bytes, outcomes, provenance, historical aggregate freshness and incomplete inventory flags. Absent inventory preserves legacy behavior. Strict typed v1 fields, <=128 entries and <=32768 compact serialized bytes; invalid paths/digests/domain-key relations/duplicates/types/unknown fields and unsupported versions refuse explicitly. No inventory-based promotion or path attribution; no aggregate hash/freshness/writer/schema/policy change. Actual old candidate refusal followed by meaningful process RED and SAME GREEN is required under separate lease. Independent review/human acceptance remain pending.

## Evidence

- Verification commit: `2f120a671ed4a3f06a75cb5dc1fe9bcf6b728384`
- Base commit: `ada04832ac7e82bc2f52ba9011db0a7e03ad7382`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The tested released-skills candidate d2a98422bffa41a869fee3f4bb914bd2e99fd1fd89e07573acddf77db93c700a refuses nine retained observations at snapshot.input_inventory. Compatible ef3289 reads unchanged bytes and labels all nine historical/stale-revision-or-inputs with incomplete inventory unavailable for paths. F2 disposition and all93 receipts remain sealed in evidence/chough102-next-owner-repair. Original ADA tree retains only its already-authorized ownsession correction dirty; it is untouched here.

This separate tree is based on exact ada04832ac7e82bc2f52ba9011db0a7e03ad7382. cli frontmatter owns src/evidence.rs. REQ-cli-004 owns aggregate freshness; REQ-cli-006 owns record bounds. Preserve both full requirement bodies. The actual private producer schema is Snapshot.input_inventory Option<InputInventory>, strict inventory {version:u32,complete:bool,entries:Vec<InputIdentity>}, strict identity {domain:u8,key:String,path:String,digest:String}. Version1 cap128 entries/32768 compact serialized bytes. Its same_inputs does not read inventory. The field is advisory but part of retained record identity; removing it loses information and breaks seals. Existing public observation read limit remains unchanged.

Chosen compatibility policy: accept validated v1 only, explicitly refuse other versions and malformed/capped metadata. Absence/null matches Option None legacy behavior. Preserve complete=false exactly; no filename guesses or path-attribution view is added. This is bounded forward-safe refusal, not a promise to interpret future formats. No new producer capability, no serializer migration, no schema bump, no denial-policy weakening. New ordinary writer snapshots remain inventory=None and omit the field.

Inherited issue102 next_owner is a separate known predecessor metadata incompatibility. Do not edit/adopt that session. Own new session has no next_owner; future fixture tests isolate that earlier session dependency and test original observations without rewriting them. Human gates remain pending.

## Exact affected paths

- `src/evidence.rs`
- `tests/evidence_inventory_compatibility.rs`
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-corpus-identity-ef1726110e673bd961ec3f56.json`
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-edited-preservation-0e13b37adb18b0e04c90dd23.json`
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-host-current-missing-ee55b3fe9e5380440663ed62.json`
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-legacy-policy-9aaea89ee334873bcfa7b314.json`
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-name-byte-matching-9218194092c97b28451038d3.json`
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7.json`
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b.json`
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619.json`
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e.json`
- `specs/cli/cli.spec.md`
- `specs/cli/requirements.md`
- `specs/cli/tasks.md`
- `specs/cli/testing.md`
- `specs/cli/cli.3md`
- `CHANGELOG.md`
- `knowledge/INDEX.md`
- `.chough/cases/cli.inventory-compat.cases.toml`
- `.chough/sessions/evidence-inventory-compat.json`

## Nine immutable future fixtures

- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-corpus-identity-ef1726110e673bd961ec3f56.json` SHA256 `eef88566047783ad44c244fb205e6021ec8a502cfbf89b987d9daf459f3c1d84` bytes 33157; original retained OBS unchanged.
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-edited-preservation-0e13b37adb18b0e04c90dd23.json` SHA256 `3feb28f66e781a02414fbac8f1ebabff8ec8eca51387eb4fa1a29fc3180c3f36` bytes 33162; original retained OBS unchanged.
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-host-current-missing-ee55b3fe9e5380440663ed62.json` SHA256 `d17e7975311ca456d098357e733d3be9ac271224262fdcde24410151af68c1f1` bytes 33157; original retained OBS unchanged.
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-legacy-policy-9aaea89ee334873bcfa7b314.json` SHA256 `60711e54532e5e287c07bd1c08138fb3de371c1dea803f684bf5d47842c4c460` bytes 33155; original retained OBS unchanged.
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-name-byte-matching-9218194092c97b28451038d3.json` SHA256 `8cefa2683f41815e013f85bcb1c127a3cec94aa5875dd809f7689c78b7e3fb45` bytes 33161; original retained OBS unchanged.
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7.json` SHA256 `d6c3f27880495cca8de70287170d93e670705fb6a08707e121b128d88263d5ff` bytes 33146; original retained OBS unchanged.
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b.json` SHA256 `006be4baf62fb33f8c008003e4be964a256fb383f78619952bc183cf3881a6fb` bytes 33150; original retained OBS unchanged.
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619.json` SHA256 `eb7f6638c470816b1624c7e0ef4fc9002bb05bb9270568c93b20c9a71d3b3136` bytes 33159; original retained OBS unchanged.
- `tests/fixtures/evidence-inventory/OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e.json` SHA256 `042eb1f89d752370bb739a02242c549476f57f08b656e64c2115f2eb7c3fbd30` bytes 33163; original retained OBS unchanged.

## From the change's design.md

# Design

No visual design. Reader extension only; exact schema/policy in Context and full digest-bound Plan.

# Plan

## Approach

Add strict optional read representation to Snapshot, reuse current fingerprint_key for validation only. Initialize None at existing runtime constructor and unit-test literal. Accept validated v1 retained data, refuse unsupported versions or invalid/capped entries. Do not generate inventory. No new command, report renderer, runtime dependencies or hashing exclusions.

## Out of scope

All original OBS/session changes, next_owner schema, record migration/re-recording, inventory production, path explanation UI, full private freshness feature integration, hash stream/exclusions/normalization, artifact/case/environment/ancestry authority, approval/review policy, schema versions, global serde relaxation, HI, source outside evidence.rs, build/full/Trust/hosted/install without allocation.

## Steps

1. Freeze concrete draft/digest; root records actual bounded scope separately.
2. After build lease, commit standalone process regressions plus nine SHA-bound original fixtures and run meaningful RED on unchanged production. Capture corpus identity before runtime and distinguish setup failures.
3. Implement minimal reader/types/validation in evidence.rs, preserving hash_input/snapshot traversal and same_inputs/freshness predicate bodies exactly. SAME GREEN plus legacy/invalid/bounds and old focused controls/fmt/lint as leased.
4. Finish owned cases/retained receipts/local bug/companions/changelog and INDEX-only generation after preview; ordinary native strict/current freeze. Other-author review and human acceptance remain open.

## Risks

Tolerant parsing could hide malformed or oversized metadata; strict v1 structural/semantic validation and fail-safe unsupported-version refusal prevent this. Advisory inventory must never promote an old pass. Partial data is not evidence of absence. Nine observations describe capture-context inventory, not original historical RED inputs. New fixture copies preserve all bytes; no transformation or historical fresh-run claim. Inherited next_owner can block broad candidate next and must be independently repaired/qualified; reader tests cannot conceal that dependency.

## Constraints consulted

Applicable AGENTS/Chough workflow/INTENT/FRD/HI LIFE2..7; actual cli owner/REQ004/006; pinned ef3289 compatible source at8a37 and actual d2a candidate parser; sealed F2 report/schema census/raw refusals and nine exact files. Direct Leif all-issues authority relayed by root grants preparation only. Other writers active; original trees/records and all gates preserved.

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Planned assertion | Boundary |
|---|---|---|
| REQ-cli-004 | All9 exact originals preserve failed/passed outcomes, historical/stale aggregate labels and stored incomplete flag; inventory changes cannot promote freshness; old absent/null input_inventory behavior and artifact/case/environment/legacy revision controls unchanged | Future actual standalone/runtime tests; compatible-tool baseline is not fixed reader proof |
| REQ-cli-006 | Strict v1 schema/types/unknown fields; duplicate paths, traversal/absolute/unsafe names, invalid SHA/domain/key,128/129 entries,32768/32769 bytes, unknown version and existing whole-record cap | Actual bounds and semantic checks, no ignored malformed complete claims |

## Discriminating process regression

Before any product invocation hash all nine fixture bytes against exact map. Run unchanged old d2a against private copied records to reproduce unknown snapshot.input_inventory. New standalone test initializes private normal fixtures/own compatible sessions without next_owner; original9 observations retain byte identity and provenance. Isolate known earlier session defect explicitly. Preserve all9 historical/noncurrent classifications, complete=false retained; fresh controlled observations vary inventory alone and remain governed solely by content/fingerprint/artifact/case/environment. Test missing inventory and null Option equivalent. Reject future-version metadata explicitly, including complete=true; malformed unknown/type/duplicate/path/cap cases cannot become complete/current. No inventory path-attribution renderer introduced.

## Planned commands and limits

Fledge run test -- --locked --test evidence_inventory_compatibility under future separate focused lease; old relevant evidence controls, fmt/lint only as allocated. No compilation during preparation, no full/Trust/site/hosted or platform claims. Tests must execute nonzero meaningful assertions; compile errors are not RED. Nine recorded runs remain historical, never re-recorded as fresh.

## Diagnostic inventory reader compatibility (acceptance pending)

Retained actual t2sm atbe17 compiled the original6:1PASS/5unknown-input_inventory FAIL after valid baselines; t2vg ata7b2543 executed only2alias tests:0PASS/2intended invalid-alias-accepted FAIL,6filtered. Source fix rejects noncanonical spelling via unchanged canonical_path before raw classification, without rewriting records. One independently confirmed uppercase digest oracle was corrected to preserve existing is_digest compatibility.

Actual t2vj atd0125 passed8compatibility (original6 with ONE uppercase oracle correction +SAME2aliases),14 existing evidence module and30 evidence integration tests=52PASS. All9 original records remain historical/partial with2failed7passed/exact values and bytes; legacy/null/strictcaps/advisory/no-promotion and every alias-loop assertion executed. Declaredfmt failed only test formatting; lint thenNOTRUN. Pinned standalone canonical formatting at4b35 is byte-equal to rustfmt(before), with no assertion edit. Actual t2vn at4b35 declaredfmt0/alltargetlint0; NO behavioral rerun. Prior52PASS is source-equivalent, not fresh4b35execution.

Source/test/corpus/DUT/tool/argv/fences and raw failures are retained in the external program evidence/chough-inventory-compat-* packets. These are agent-assisted retained receipts, not authenticated test execution or human acceptance. Full/Trust/site/hosted/platform/humanreview are not established. Inventory is diagnostic only, never aggregate freshness/path attribution/authority. Known inherited issue102 next_owner is preserved and separate; tested own next read0 here.

## Where these lessons go

- `specs/cli/cli.3md`
