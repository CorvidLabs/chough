## ADDED

### REQUIREMENT REQ-cli-025

Immutable transition history. The helper SHALL offer read-only `history adopt` and explicit `history adopt --apply`, preserving the byte-identical legacy transitions log and schema-1 migration backup. Adopted and newly initialized projects SHALL write immutable versioned event files with independent random 96-bit cycle/event identities, parent links, native-change context and the existing transition payload. The single schema-2 checkpoint SHALL hold an optional history cursor as part of its atomic current-cycle merge group. Event publication SHALL precede atomic checkpoint/cursor publication; failures SHALL preserve prior checkpoint state and mark unselected events explicitly. No second mutable checkpoint or inferred native lifecycle state SHALL be introduced. Trace: LIFE-2, LIFE-3, LIFE-6, LIFE-7, FLOW-2, FLOW-13.

Acceptance Criteria
- Adoption preview changes no file; explicit apply and repeat preserve original log and migration bytes, validate a digest/length/count anchor and do not duplicate events. Unexpected anchor changes refuse new writes.
- Legacy checkpoints without a cursor and explicit schema-1 migration remain supported. Old readers of adopted checkpoints fail visibly; they never silently discard the cursor.
- Independent real Git branches writing at the same controlled timestamp retain both immutable event paths and hashes. Divergent cursor edits refuse as a current-cycle group, while compatible merges preserve the selected lineage.
- New event/cycle IDs use one random candidate with atomic no-replace refusal and no retry or fallback. Failed random, event or checkpoint writes preserve existing bytes; orphan events cannot count as completed transitions.
- Parent links and the selected checkpoint cursor determine lineage. Missing parents, conflicting identities and unrelated branch reset events cannot manufacture order, change current guidance or bypass native gates.

### REQUIREMENT REQ-cli-026

Bounded history lifecycle. The helper SHALL bound each event at 65,536 bytes, recent traversal at 128 events and 512 KiB, each new cycle at 4,096 events and 16 MiB, and each timing request at 4,096 events and 16 MiB of transition content. Maintenance enumeration SHALL be limited to 16,384 source files and 64 MiB of metadata, with incomplete scans refusing apply. Ordinary appends at cycle capacity SHALL refuse before checkpoint mutation; a reasoned reset MAY start a new cycle while preserving the previous one. Recent guidance SHALL follow the selected cursor without a directory-wide event scan. Timing SHALL keep independent lineages separate and report omitted, malformed, missing and pruned history as partial or unknown, never zero or verified. Trace: LIFE-3, LIFE-7, LIFE-8, LIFE-10, FLOW-13.

Acceptance Criteria
- Instrumented synthetic fixtures prove each exact byte/event bound, bounded routine reads and byte-identical state on refusal; output retains existing focused-view limits.
- `history prune` previews exact candidate cycle files, digests, bytes and protection reasons without writing. It protects active, newest-three, younger-than-14-day and unresolved fork/orphan cycles and every checkpoint, bug, case, native-manifest and frozen-evidence reference. Unreadable protection sources prevent deletion.
- Only an explicit `history prune --apply` with existing human retention authorization may remove eligible complete closed cycles; evidence auto-retention never starts history pruning. Legacy anchors/logs, migration backups and native archives remain untouched.
- An immutable receipt bounded at 1 MiB names planned paths/digests and authorization before deletion. Present files remain retained, absent receipted files are pruned and absent unlisted files are unavailable. Interruption is explicit partial cleanup, never completed timing evidence.
- Protected-only data yields a truthful no-op. The report explains that retained proof may grow beyond the routine retention target; no global disk-size guarantee is inferred.
- A real Git fixture integrates supported retention with another branch's new events/evidence, retaining all surviving references and artifact hashes.

### REQUIREMENT REQ-cli-027

Conservative evidence cleanup. Evidence retention SHALL keep the existing eligibility thresholds, protection semantics and ledger entry shape, rebuild protection at apply and revalidate candidate identity and safe paths immediately before removal. Changed, missing, shared, newly referenced or unsafe files SHALL be retained or refused with explicit outcomes. Interrupted cleanup SHALL distinguish present content, recorded removal and unavailable history. It SHALL preserve the active checkpoint, migration backup, historical retained proof, native archives and every artifact still referenced by another observation. Trace: LIFE-2, LIFE-7, LIFE-8, FLOW-7, FLOW-13.

Acceptance Criteria
- Adding a checkpoint, bug, case or native protection reference after preview protects the relevant observation/cycle when apply runs.
- Shared artifacts remain present and byte-identical; missing/changed candidates, malformed sources, path traversal and symlinks cannot broaden deletion or be reported as a successful complete cleanup.
- Fault injection around publication/removal yields an actionable partial outcome and leaves active state and protected bytes intact. No existing observation, artifact fingerprint or evidence-ledger schema is rewritten.
