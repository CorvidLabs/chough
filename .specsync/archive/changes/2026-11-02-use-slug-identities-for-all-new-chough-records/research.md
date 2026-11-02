---
change: use-slug-identities-for-all-new-chough-records
artifact: research
---

# Research

## Repository evidence

`src/cases.rs` currently enforces exactly four numeric case digits. Its loader already holds one `BTreeSet` across all case files, so global exact duplicate refusal exists and must be retained when the grammar widens. Case IDs and definition digests are stable separately from execution records.

Automatic observation IDs in `src/cases.rs` combine a lowercase case ID with `now()` in seconds and then try up to 1000 numeric suffixes. `src/validate.rs` repeats the same pattern without the 1000-attempt cap. Two independent roots with the same case and second choose the same base and first available suffix for different records. Repeating an identical run within one second has the same candidate base. A local counter can preserve records in one root, but it cannot provide branch-independent identity and makes allocation depend on prior local history.

`src/library.rs` allocates `<date>-<number>` by scanning existing directories. The same date and local directory sequence therefore produce the same path in independent roots, even for different reasons or source content. Existing snapshot manifests carry a creation timestamp, reason, revision and source digests, so changing the directory identity requires no manifest schema change.

`src/records.rs` supplies a 100-byte general record-name limit and `OpenOptions::create_new(true)` file creation. Existing prechecks do not replace atomic creation, because another writer can win after a check. The recent bug change demonstrates the project's intended collision behavior: one candidate, one create-new attempt, different-content refusal, preserved bytes and no numeric or digest fallback.

Legacy observations and validation runs have readable names under the general bounded-name rule rather than one strict historical grammar. Legacy snapshots use dated directories. Read compatibility therefore cannot be implemented by accepting only the new regexes. Write-time rejection of an explicitly supplied old observation ID must not narrow loading, linking or resume.

The lockfile already resolves getrandom 0.4.3. It is not yet a direct Chough dependency in `Cargo.toml`; declaring that exact existing version allows direct use without selecting another package or adding a runtime. The existing bug implementation uses SHA-256 over a canonical creation payload and has fixed known answers that must not move onto random generation.

## Recent native deltas

The archived `use-collision-resistant-slug-identities-for-new-bugs` delta modified REQ-cli-012 only. It established shared ASCII slug normalization, a 71-byte bug slug, 24 lowercase hexadecimal digits, legacy dual-form reads and atomic different-content refusal. Its design explicitly excluded evidence, artifact, validation, snapshot and case identities. This change reuses only the common normalization behavior and leaves bug payload generation and vectors fixed.

The archived `make-checkpoint-merges-safe-across-parallel-changes` delta modified REQ-cli-007 and preserved the single checkpoint. It also demonstrates failure-first real fixtures, ordered ownership and explicit refusal preservation. This change follows those planning constraints without adding ID state to the checkpoint, transitions or merge behavior.

No canonical requirement, contract or frozen archive was edited during this research.

## Alternatives compared

| Approach | Collision behavior | Repeated identical history | Decision |
|---|---|---|---|
| Bare slug | Same title, case or reason produces the same ID | Collapses or refuses repeats | Reject as the sole identity; retain a slug only for readability |
| Timestamp | Same clock tick produces the same base; skew and backward clocks remain possible | Identical same-tick runs still need a counter | Reject as uniqueness; keep timestamps only in existing chronology fields |
| Deterministic content hash | Distinct content is collision-resistant; identical content yields the same candidate | Intentionally identical inputs and time produce the same ID | Reject for event history; retain for existing bug creation identity |
| Random 96-bit suffix | Independent roots need no shared state; a rare collision remains possible | Identical repeats independently allocate distinct candidates with high probability | Select with one getrandom fill and mandatory atomic refusal |

A deterministic digest is correct for bugs because the ID belongs to one immutable creation record. The archived bug contract intentionally makes an identical creation payload select the same candidate and refuse an existing record. It is wrong for observations, validation attempts and snapshots because their purpose is to retain every event, including events whose content and timestamp are identical. Adding a local counter or timestamp to such a digest would merely hide the same coordination and clock problems inside the hash.

Bare slugs remain useful as prefixes because they make records recognizable, but they carry no uniqueness guarantee. Random suffixes avoid central coordination and preserve repeat history, at the cost of non-deterministic IDs and a small collision probability. The 96-bit space is not a proof of uniqueness, so no-replace creation and no retry are part of the selected design rather than optional error handling.

## Design conclusions

One normalizer can serve bugs, curated case titles and random-suffix records if each caller supplies its fallback and byte budget. The 100-byte ceiling leaves 71 slug bytes for OBS and VAL, 70 for SNAP, and `96 - module.len()` for a case title slug. The suffix is never truncated.

Case IDs remain curated because definitions are permanent and title edits must not rename them. Observation and validation slugs derive from case identity, direct evidence without a case derives from its kind, and snapshots derive from their reason. These sources affect readability only.

Existing create-new file persistence covers observations, validation runs and logs. Snapshots need atomic final-directory creation with `create_dir`, create-new children and a manifest written last. This does not justify claiming a crash-proof transaction across several files. All pre-existing records remain untouched on random-source, collision and write failures, and no alternative candidate is attempted.

No external repository or upstream change is required. The remaining uncertainty is test fixture fidelity: controlling time, constructing equivalent independent roots, injecting random and write failures, forcing directory and file collisions, and retaining genuine legacy coverage. Those are verification tasks with test-only seams, not unresolved product design questions and not reasons for a production override.
