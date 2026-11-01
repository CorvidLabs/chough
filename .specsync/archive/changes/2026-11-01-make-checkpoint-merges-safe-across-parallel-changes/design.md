---
change: make-checkpoint-merges-safe-across-parallel-changes
artifact: design
---

# Design

## Command and Git integration

Add `chough checkpoint --merge <base> <ours> <theirs>` with exactly three path operands. The merge option is mutually exclusive with every existing checkpoint update option. It reads only the three supplied files and never reads or writes the canonical session merely because `--root` names a project.

The command follows Git's merge-driver convention: base is `%O`, ours is `%A`, and theirs is `%B`. On a compatible merge it replaces only ours and exits zero. On a refusal it exits nonzero with a named diagnostic and does not insert conflict markers. The repository declares:

```gitattributes
.chough/session.json merge=chough-session
```

The documentation gives this per-clone setup command:

```sh
git config merge.chough-session.driver 'chough checkpoint --merge %O %A %B'
```

Initialization does not run this configuration command or otherwise mutate Git config. Without the driver, the user can run the same explicit command on three files. The command performs no shell evaluation and introduces no background process, runtime or service.

## Input and path safety

Each operand must be an existing regular file whose final component is not a symbolic link. Read each input as bounded UTF-8 with the existing 65536-byte structured-record ceiling. The three resolved paths must be distinct so the read-only base and theirs operands cannot also be the output path.

Git can place `%O`, `%A` and `%B` outside the project and can use absolute temporary paths. Therefore these command operands must not go through the project-relative `local_path` rule. This exception applies only to the three explicitly named merge files. Existing validation of `references.design`, `references.files` and every managed project path remains unchanged.

Only ours is writable. After all parsing, merging, serialization and output validation succeed, write the complete result to a create-new sibling temporary file, sync it, and rename it over ours. Do not create parent directories. Remove the temporary file on every pre-rename failure. Base and theirs are never opened for writing. No other persistent file or record is created. The transient sibling is renamed over ours on success and removed on every pre-rename failure.

This is a Git merge operation, not a lock or a new policy for concurrent live checkpoint writers. Existing one-owner live writes remain the rule.

## Exact-string collection algorithm

Apply the same ordered three-way membership merge independently to `references.criteria`, `references.requirements`, `references.cases`, `references.design`, `references.files`, `decisions`, `open_questions`, `completed`, `observations` and `bugs`.

Strings compare exactly and case-sensitively, with no trimming or normalization. Interpret each input collection by membership and emit each retained string at most once. For a string present in base, retain it only when it is present in both ours and theirs. Thus a removal on either side is preserved when the other side is unchanged. For a string absent from base, retain it when either side contains it. Thus one-side additions survive, and identical additions from both sides appear once.

Construct the output order as follows:

1. Walk base order and emit each base string retained by the membership rule.
2. Walk ours order and emit each retained string that was absent from base and has not already been emitted.
3. Walk theirs order and emit each retained string that was absent from base and has not already been emitted.

For example, base `[a, b, c]`, ours `[a, c, o, shared]`, and theirs `[b, c, t, shared]` produce `[c, o, shared, t]`. The one-side removals of `a` and `b` survive, base order leads, ours-only additions precede theirs-only additions, and the identical addition `shared` appears once.

This rule does not reorder retained base entries to match either branch. Reordering alone has no membership effect. If a later requirement needs order editing as distinct behavior, it needs a separate contract rather than an implicit extension of this merge.

## Atomic current-cycle group

Treat this complete tuple as one value:

- `objective`
- `intent_scope`
- `modules`
- the complete `stage` value, including status, callers and iteration
- `native_change`
- `next_action`
- the complete ordered `run_consents` collection
- the complete ordered `blockers` collection
- the complete ordered `awaiting_human` collection

Compare typed values, not JSON whitespace. Ordered collections and nested stage values compare in full. Let `ours_changed` mean the ours tuple differs from base, and define `theirs_changed` likewise. Select base when neither side changed, ours when only ours changed, theirs when only theirs changed, and either identical tuple when both sides made the same change. If both sides changed and the tuples differ, refuse with a `current-cycle conflict` diagnostic naming the differing member fields.

There is no field-by-field fallback. For example, one branch changing only `next_action` and another changing only `native_change` is still a conflict because selecting both would manufacture a cycle that neither side stored. A union of consents, blockers or human gates is equally invalid.

## Validation, refusal and side effects

Every input must independently parse and pass the existing strict schema 2 checkpoint validation. Schema 1 in any position, mixed schema versions, unsupported versions, malformed JSON, wrong types and unknown fields at any level fail closed. The merge command neither migrates schema 1 nor creates a schema 1 backup.

After constructing the candidate, run full schema 2 validation before serialization. Serialize in the existing pretty JSON form with one trailing newline, parse and validate those output bytes again as schema 2, and enforce both the existing serialized checkpoint validation bounds and the 65536-byte emitted-record ceiling. This includes module, caller and collection limits. In particular, a valid pair of additions whose union exceeds 200 entries or whose result exceeds a byte bound is a refusal, not a truncated merge.

Perform no write until every step succeeds. Every refusal leaves base, ours and theirs byte-identical and creates no transition, gate, approval, review, consent, migration, observation or bug record. Successful selection or merging also creates none of those records. In particular, merged awaiting-human values do not open or close gates, merged consents do not authorize anything new, and merged stage values do not infer a transition. The resulting checkpoint remains data for subsequent normal validation and native authority checks.

## Compatibility and migration limits

The schema remains 2 and the canonical path remains `.chough/session.json`. Existing schema 1 context reads and explicit migration behavior are untouched. No shard, journal, cache or derived checkpoint is retained. No case number is allocated, and no checkpoint, transition, evidence, artifact, validation, snapshot, case, bug, HI, requirement or SpecSync identity changes.
