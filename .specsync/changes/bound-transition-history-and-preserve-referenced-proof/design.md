---
change: bound-transition-history-and-preserve-referenced-proof
artifact: design
---

# Design

## Proposed persisted format

Keep .chough/session.json as the only writable checkpoint. Add an optional history_cursor containing a cycle ID and head event ID; it participates in the current-cycle merge group. No second mutable head/index is authoritative.

New transition events live under .chough/history/cycles/<cycle-id>/<event-id>.json. Cycle and event IDs use independent 96-bit random suffixes, one candidate and create-new refusal, with no fallback or retry. An event envelope has a version, its IDs, parent ID, native-change context, and the unchanged schema-1 transition payload. A cycle root records its predecessor cycle as a historical link, not permission to join independent timelines. Parent links determine order within a lineage; time fields determine durations. Timestamps and random suffixes never establish causality across branches.

A new project initializes this history format. An existing project stays in legacy mode until chough history adopt --apply. Without --apply, adopt is a read-only preview. Apply verifies the legacy file, records its digest, byte length and line count as an anchor, and writes the optional cursor atomically. It leaves transitions.jsonl and session.v1.json byte-identical. Repeating adoption verifies the existing anchor and does not duplicate history. Unexpected legacy changes after adoption refuse new writes and produce an actionable diagnostic.

Publish complete event files before atomically replacing the checkpoint and cursor. A failed checkpoint save leaves any published event unselected and visible as an orphan; it cannot advance the checkpoint or count as a completed transition. Gate changes publish their complete event chain before saving checkpoint fields and the final cursor together. Concurrent live checkpoint writers remain unsupported under the existing one-owner rule.

## Branch integration and resume

Independent branch event files have distinct paths and survive Git integration. The chosen checkpoint cursor selects one coherent current lineage. Divergent cursor edits are a current-cycle merge conflict even if stage names happen to match. Other branch histories remain stored and inspectable. A missing parent, conflicting event identity, fork without a selected tip or malformed envelope is partial/unknown, never an invented order. The current stage still comes from the checkpoint and native gates remain authoritative. Reset guidance must not use an unrelated branch's reset event.

A stage reset starts a new cycle and head. Changes of next_action or native-change metadata alone do not create a cycle. timing reports lineages separately and never calculates a stage/gate span across unrelated heads or omitted parents. Legacy timing preserves its existing limitations and labels.

## Proposed bounds

- One event: at most 65,536 bytes, including its envelope.
- Recent guidance: follow at most 128 parent events and read at most 512 KiB; retain existing output caps. No directory-wide event scan for next/status/context/stage show.
- One new cycle: at most 4,096 event files and 16 MiB of event bytes. Check before mutation. At capacity, ordinary writes refuse without changing checkpoint state; a reasoned reset can start a new cycle without deleting the old one.
- A timing request: at most 4,096 events and 16 MiB of transition bytes across selected lineages; reaching a bound is explicit partial history with unknown affected spans.
- Maintenance enumeration: at most 16,384 source files and 64 MiB of metadata per invocation. Exceeding a bound yields an incomplete preview and refuses apply; protection sources are never silently truncated.

These are proposed defaults, tested at their exact boundaries. They bound routine work and per-cycle storage. Total protected proof has no artificial disk cap; it can prevent reclamation. No claim of globally bounded disk consumption is made when proof must stay.

## History retention

chough history prune previews by default. A cycle is closed for deletion only when every retained tip ends at done and its associated native change authenticates as archived, or an explicit reset closed a lineage without leaving an unresolved tip. Incomplete or ambiguous lineage closure protects the whole cycle. Preview lists complete closed cycles, exact file paths/digests/bytes and every protection reason. Preserve the active cycle, unresolved forks/orphans, the newest three closed cycles, cycles closed less than 14 days ago, and anything referenced by checkpoints, bug fixtures/notes, case artifacts, native manifests or frozen evidence. Any unavailable protection source blocks deletion. Never prune the legacy anchor/log, migration backup, cases, observations or native archives through this command.

--apply requires the explicit history prune invocation plus the existing human retention authorization, and rebuilds protection and file identities immediately before removal. The existing retention auto setting does not start history pruning as a side effect of cases or validation. Applying the implementation to this repository requires a later explicit human operation; fixtures use disposable roots.

Publish an immutable receipt under .chough/history/pruned/<cycle-id>.json before deletion. It names every planned path and digest, the reason, authorization source and cycle identity; cap a receipt at 1 MiB and refuse larger plans. A receipt plus absent content means pruned; a receipt with still-present matching content means retained or interrupted. A repeated apply may resume only the identical receipted plan after fresh protection checks; a conflicting receipt or changed file is refused. Unselected orphan files remain protected and do not count as completed events. Missing unlisted content is unavailable. A deletion interrupted midway remains partial, never a completed or successful timeline. Readers do not synthesize duration evidence from receipts.

## Existing evidence retention

Keep existing retention defaults and ledger shape. A new preview-to-apply change to references must protect that set. Changed, missing or unsafe candidate paths cause conservative skip/refusal with explicit results. Shared artifacts stay while any record references them. Revalidate hashes and path identity before removal and never apply based on stale preview output. The one-owner maintenance boundary is documented; arbitrary external filesystem writers are not claimed to be serialized.
