---
change: use-slug-identities-for-all-new-chough-records
artifact: design
---

# Design

## Shared normalization and grammars

Add one crate-internal identity module for slug normalization, new-form validation, random suffix encoding and candidate construction. Reuse the normalizer from the existing bug generator without changing its output, domain separator, canonical payload, known answers or update behavior.

Normalization iterates over input bytes, lowercases ASCII `A` through `Z`, retains ASCII `a` through `z` and `0` through `9`, replaces each maximal run of every other byte with one hyphen, and trims leading and trailing hyphens. There is no Unicode transliteration. The caller supplies a nonempty ASCII fallback for an empty result. Truncate the normalized slug to the caller's byte budget and remove trailing hyphens left by truncation. Output is ASCII, so byte truncation cannot split UTF-8. A valid slug is nonempty, contains only lowercase ASCII letters, digits and single interior hyphens, and neither starts nor ends with a hyphen.

All new forms remain at most 100 bytes, including every separator and all 24 suffix digits:

| Record | Exact new grammar | Slug byte budget | Total overhead |
|---|---|---:|---:|
| Case | `TC-<module>-<title-slug>` | `96 - module.len()` | 4 plus module length |
| Observation | `OBS-<slug>-<24-lowercase-hex>` | 71 | 29 |
| Validation run | `VAL-<slug>-<24-lowercase-hex>` | 71 | 29 |
| Snapshot | `SNAP-<slug>-<24-lowercase-hex>` | 70 | 30 |

For random-suffix forms, the suffix is exactly 24 characters from `[0-9a-f]`, obtained from 12 bytes. The slug is the part before the final separator and must satisfy the shared slug grammar and prefix-specific budget. These forms are valid under the existing path and record-name rules.

Case validation accepts either the exact legacy form `TC-<module>-NNNN`, where `N` is four ASCII digits, or the new curated form. `<module>` is the exact already validated case-file module stem, and the title slug uses the shared grammar and module-aware budget. Normalization helps an author derive a new ID from the title, but validation does not recompute an ID from the current title. Later title edits therefore do not rename a case. The existing loader-wide set refuses an exact duplicate across all case files, not merely within one file. Existing numeric IDs and their definition digests remain unchanged; every case definition added for this change uses a slug ID.

## Random allocation and slug sources

Use getrandom 0.4.3 directly as a declared dependency and call its fill operation once with a 12-byte array. Encode each byte as two lowercase hexadecimal digits without adding a formatting dependency. One allocation consumes one random result and constructs one candidate. Do not include a timestamp or content digest in the suffix and do not read the destination to decide whether to generate again.

Slug sources are stable and readable rather than uniqueness mechanisms:

- `cases run` and `cases record` normalize the full case ID for an observation.
- Direct `evidence add` without an explicit ID normalizes the first linked case ID when supplied, otherwise the observation kind.
- A validation run normalizes its case ID.
- A librarian snapshot normalizes its reason.

Fallbacks are `observation`, `validation` and `snapshot` for their respective record types, and `case` when initially deriving a case title slug. Random suffixes distinguish identical repeated runs even when slug, content and timestamp are equal. Existing `observed_at_unix_seconds`, `started_at_unix_seconds` and snapshot `created_at_unix_seconds` fields remain unchanged and determine chronology. ID sorting never replaces timestamp sorting.

Bug IDs remain deterministic for a different reason: a bug is one creation record keyed by its canonical initial content, so identical creation payloads intentionally produce the same candidate and refuse rather than manufacture another report. An observation, validation attempt or snapshot is history of an event, so identical repeated events still need separate identities.

## Explicit observations and legacy reads

`cases run --observation-id`, `cases record --observation-id` and the positional ID on `evidence add` accept an optional exact new-form observation ID. `evidence add` can omit its positional ID and allocate automatically, matching the case commands when their option is absent. An explicitly supplied new-form ID is used once as supplied; it is not regenerated.

When a new write supplies an otherwise legacy-valid observation ID that does not match the new form, refuse before creating an observation or automated artifact. The remedy says to omit the ID for automatic allocation, including removing `--observation-id` where applicable. Malformed path or record names retain the ordinary invalid-ID error. No validation or snapshot explicit-ID option is added.

Read and reference validation remains dual-form where a defined legacy grammar exists and permissive under the existing bounded `valid_name` rule where old observation and validation IDs were arbitrary readable names. Existing timestamp observations, numeric-suffixed validation runs and dated snapshot directories continue to load. `validate resume` resolves both old and new run IDs. This is write-time allocation policy, not a rewrite of stored records.

## Atomic no-replace creation

For observation JSON, validation JSON and automated log files, retain the existing `OpenOptions::create_new(true)` persistence path as the atomic allocation mechanism. Existence prechecks may improve diagnostics but are not the uniqueness mechanism. Map an already-exists result to a refusal, read the existing item within its applicable bound, and distinguish same-ID different-content from identical already-existing content. Both refuse and preserve existing bytes. A different-content refusal names the identity collision. There is no overwrite, replacement rename, timestamp suffix, numeric suffix, digest retry or second random candidate.

An automated run allocates its observation ID once and uses exactly `.chough/artifacts/<observation-id>.log`. The log and observation are separate no-replace writes; there is no claim of a multi-file transaction. A failure after one write preserves every pre-existing record and reports the incomplete attempt rather than recording success or retrying under another ID.

For snapshots, replace date-counter selection with `<library-dir>/snapshots/SNAP-<slug>-<24-lowercase-hex>`. Create the parent if needed, then use `create_dir`, not `create_dir_all`, on the final snapshot directory so an existing final component refuses atomically. Populate copied files only through create-new writes and write `manifest.json` last as the completion record. An existing directory is never removed or reused. If its stored manifest and referenced content can be compared and differ from the candidate, report a same-ID different-content collision; an unreadable or incomplete existing directory also remains preserved and refused. The manifest schema, entry shape, source bytes and source digests remain unchanged.

Random-source failures occur before any destination write. Create-new and later write failures never modify pre-existing records and never trigger another candidate. Snapshot population is not presented as a crash-proof multi-file transaction: manifest-last publication makes an interrupted new snapshot incomplete rather than a valid completed snapshot, and later readers must not describe an unreadable manifest as valid history.

## Compatibility and unchanged ownership

Mixed old and new IDs continue through observation loading, bug evidence links, checkpoint references, status and next, timing, retention and pruning, native manifests, validation status and resume, and librarian indexes. Retention continues to key observation sets by exact IDs and artifact paths. Pruning ledger entries gain no ID. Snapshot indexes discover legacy dated directories and new slug directories by their stored paths and timestamps.

No bulk migration or schema version change occurs. Observation schemas 1 and 2, validation schema 1, snapshot manifest schema 1 and bug schema 1 stay as they are. Bug known-answer vectors remain byte-for-byte unchanged. Transitions, preferences, consents, stop markers and pruning entries keep their shapes. No second lifecycle or checkpoint store, runtime, service or external coordination is introduced. Older Chough versions require an upgrade for slug case IDs and the other new forms; legacy records do not require a downgrade or migration.
