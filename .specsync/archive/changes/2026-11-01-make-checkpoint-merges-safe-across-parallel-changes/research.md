---
change: make-checkpoint-merges-safe-across-parallel-changes
artifact: research
---

# Research

## Repository evidence

The existing checkpoint is one pretty-printed JSON value. Its `deny_unknown_fields` structs reject unknown input, `validate_v2` enforces schema 2 and the existing module, stage, ID, path and collection limits, and `save_checkpoint` uses validated bounded serialization and atomic replacement. Normal checkpoint updates add unique exact strings to ordered collections. There is no merge operation today, so ordinary Git compares lines without understanding those collection or current-cycle semantics.

Three repository merges demonstrate the recurring cost:

- `b2e2f19d904aec2a5acd73569c8439f5583c01f9` explicitly lists `.chough/session.json` as conflicted. Its resolution combines historical collections and selects current-cycle state by hand.
- `e31fa7c768625b8cb2c2b1b12ce77c195deac99d` records another conflict resolution. Its message says `.chough/session.json` kept the branch checkpoint, showing that current-cycle selection required human judgment rather than a safe textual union.
- `14b4e7f508b9701a7fbf19fb752c8176a66293b4` again lists `.chough/session.json` as conflicted during integration of main into the bug-identity branch.

These commits justify the design problem, but they do not replace the required new failure-first observation. The implementation plan therefore starts with a fresh real Git reproduction linked to a local bug and ends with a later pass.

Recent native deltas show the expected contract style: modify REQ-cli-007 rather than allocate a new requirement, state observable acceptance behavior, and preserve unrelated schemas and identities. No canonical delta is written by this research artifact.

## Alternative: append-only journal

A journal could store each checkpoint update as an event and make parallel appends easier to combine. It would avoid some whole-file conflicts, but every reader would need deterministic replay, ordering and compaction rules. Current-cycle replacement, removals, consent clearing and gate clearing would need event semantics, and concurrent events would still need conflict resolution.

Keeping the existing snapshot plus a replayable writable journal would create a second checkpoint store. Replacing the snapshot with a journal would be a broad storage migration and would change cold-start reads. Reusing `.chough/transitions.jsonl` would also be incorrect because that log records stage and gate history, not arbitrary checkpoint state. This option is larger and duplicates authority rather than solving only Git integration.

## Alternative: per-change shards

Each native change could own a separate checkpoint fragment, leaving Git to combine distinct files. This would reduce collisions between independent branches, but readers would need rules for discovering shards, choosing the active cycle, resolving shared fields, cleaning stale shards and handling two branches that edit the same shard.

A set of writable fragments beside `.chough/session.json` is a second checkpoint store and can create competing current-cycle authorities. Making the canonical file a generated aggregate adds another generation and consistency problem. Shards also do not remove the semantic conflict when two branches name different native changes or stages. This option is rejected for its storage and lifecycle expansion.

## Selected alternative: bounded three-way merge

A single explicit command can understand the existing schema without changing storage. Exact-string collections have a small deterministic membership rule that preserves base order, ours-only additions, theirs-only additions, deduplication and one-side removals. The current-cycle group uses ordinary three-way selection and refuses the cases that need human judgment.

The implementation is limited to command parsing, bounded input and output, one atomic file replacement, an attributes declaration and local driver documentation. It adds no dependency, runtime, service or persistent derived state. It also gives a direct fallback for users who have not configured Git.

The tradeoffs are explicit. `.gitattributes` cannot transport Git config, so every clone that wants automatic merging must run the documented driver setup. Init remains non-mutating with respect to Git config. Current-cycle conflicts are unavoidable when both sides change the group differently, and the driver must leave them unresolved rather than manufacture a coherent-looking cycle. Overflow and unsupported input likewise refuse rather than degrade to a partial merge.

## Git behavior and validation questions

Git supplies base, ours and theirs as `%O`, `%A` and `%B`; the driver must write ours and return zero only for a clean result. These paths can be absolute and outside the repository, so tests must exercise real temporary-path behavior rather than only project-relative fixtures. A nonzero result must leave Git with an unresolved path and preserve ours without synthetic conflict markers.

The main fixture uncertainty is cross-platform driver invocation and isolation from developer Git config. Real Git tests need an explicit executable path, repository-local identity and driver config, and isolated global and system configuration. They must inspect both the merged bytes and the unmerged index state, not infer success from command output alone.

The relative estimate is medium because the semantic algorithm is compact but failure-first and real Git evidence require careful fixtures. No estimate assumes that all supported hosts have identical shell quoting or temporary-directory layouts.
