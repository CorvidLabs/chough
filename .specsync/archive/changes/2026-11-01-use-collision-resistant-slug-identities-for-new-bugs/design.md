---
change: use-collision-resistant-slug-identities-for-new-bugs
artifact: design
---

# Design

## ID form and title normalization

Every newly created bug receives `BUG-<slug>-<digest>`, where `<digest>` is exactly 24 lowercase hexadecimal characters. The slug is derived once from the creation title:

1. Lowercase ASCII `A` through `Z`, retain ASCII letters and digits, replace each maximal run of every other character with one `-`, and trim leading and trailing `-`. There is no Unicode transliteration.
2. If nothing remains, use `bug`.
3. Limit the slug to 71 bytes and trim any trailing `-` left by truncation.

The 71-byte cap reserves four bytes for `BUG-`, one for the separator and 24 for the complete digest, so the whole ASCII ID is at most the existing 100-byte `valid_name` limit. Truncation never shortens or alters the digest. Validation accepts both this new shape and the exact legacy `BUG-NNNN` shape.

## Canonical creation payload

A shared generator builds the ID after the initial schema 1 record values are known and before the first save. Hash the ASCII bytes `chough/bug-id/v1`, one zero byte, then a dedicated canonical JSON creation payload. The domain separator and payload layout are version 1. Any future incompatible layout requires a new domain version rather than silently changing version 1.

The payload fields, in serialization order, are `schema_version`, `title`, `symptom`, `trigger`, `expected`, `actual`, `criteria`, `requirements`, `cases`, `environment`, `revision`, `steps`, `fixture`, `observations`, `impact`, `classification`, `scope`, the initial `status`, `regression_baseline`, `duplicate_of`, `related`, `source`, the initial `notes`, and `created_at_unix_seconds`. The payload excludes `id` and `updated_at_unix_seconds`. Each initial note retains its `at_unix_seconds`, `text` and `observations` fields in that order.

Canonical JSON is compact UTF-8 with the field order above, standard JSON string escaping, explicit `null` for absent optional values, creation order for arrays, and no unordered maps or insignificant whitespace. A dedicated serializable payload type, rather than serializing a map or the final record, fixes this contract. Hash the domain and payload with the existing `sha2` 0.10 dependency, take the first 12 digest bytes, and encode them as 24 lowercase hexadecimal characters without adding a hex dependency.

The timestamp is only one creation field; the complete payload, not the timestamp or legacy high-water mark, supplies digest variation. Manual addition and automatic capture call this same generator whenever they create a new record. Automatic capture that appends to an existing open bug does not create another ID.

The ID is generated once and stored in both the record and filename. Updates, including changes to fields represented in the original creation payload, never recompute or rename it. No canonical payload or new schema field is persisted.

## Atomic creation and collisions

Creation makes one candidate ID and one atomic create-new attempt using the existing `create_new(true)` persistence path. An existing candidate path fails closed. Same-ID different-content produces an explicit collision error and preserves the existing bytes; identical existing content follows the normal already-exists error and is also not overwritten. There is no numeric fallback, digest retry or second candidate. This handles both a digest collision and concurrent creation without silently replacing a report.

## Compatibility and limits

Loading, listing, search, show, report, status, updates and evidence references treat both accepted ID forms as bug IDs. `duplicate_of` and `related` resolve either form, including links in both directions between legacy and new records. Existing records remain at their current paths with schema 1 and their original IDs. There is no bulk migration, renumbering or rewrite of old observations and archives.

Documentation must state that older Chough versions that accept only `BUG-NNNN` need an upgrade before reading or referencing new IDs. No evidence, artifact, validation, snapshot, transition, case, HI, requirement or SpecSync ID scheme changes.
