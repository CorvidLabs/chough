---
change: read-bounded-diagnostic-input-inventories-without-changing-freshness-authority
artifact: context
---

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
