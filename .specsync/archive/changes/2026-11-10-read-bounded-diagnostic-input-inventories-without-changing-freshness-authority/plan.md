---
change: read-bounded-diagnostic-input-inventories-without-changing-freshness-authority
artifact: plan
---

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
