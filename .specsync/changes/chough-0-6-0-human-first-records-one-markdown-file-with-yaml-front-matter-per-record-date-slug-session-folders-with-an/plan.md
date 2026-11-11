---
change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
artifact: plan
---

# Plan

## Approach

Replace Chough's storage layer in one change, behind the commands people and agents
already use. Build a small shared Markdown record layer first, move each record kind
onto it, delete the 0.5.x machinery the new layout makes unnecessary (event files,
trims, ledger, vault copies), then write the one-way migration, and finally migrate
this repository and finish this change's own session in the new format. Keep every
behaviour a requirement still promises; port each existing test's assertions to the
new layout instead of deleting them, and delete a test only with the requirement
that retires its behaviour.

## Out of scope

- Any repository other than chough. They move after 0.6.0 ships at 95 percent
  confidence or better.
- Publishing 0.6.0 to crates.io. The version bump and changelog are prepared here;
  publication waits for Leif's one-line go in the release stage (LIFE-25).
- Rewriting Git history to remove values that leaked before 0.6.0.
- Changing SpecSync, HI or Fledge. Their records are linked, never written, except
  the approval or review a human's gate decision drives (REQ-records-008).
- Backfilling journal posts for changes archived before 0.6.0; the index links
  `.specsync/archive/changes/` for those.
- The 1.0 host-matrix and integrations work (LIFE-21, LIFE-22, LIFE-24).

## Steps

Sizes are relative (S, M, L, XL). History is too thin for time ranges: `chough
timing` holds one cycle for this session.

| # | Increment | Size | Main uncertainty |
|---|---|---|---|
| 1 | RED: the #165 regression in the 0.5.x layout (seven archived sessions, `knowledge/` neither tracked nor ignored, accept to done) fails on 0.5.0; local bug linked | S | none |
| 2 | Record layer: front matter subset reader and writer, section appends, bounds, names with clash suffix and aliases, the leak guard | L | edge cases in the subset parser |
| 3 | Sessions: `session.md` checkpoint and story, Timeline entries, active pointer and lock, `agent/` and inbox, finish into one story with the committed-bytes rule, close, shelve | XL | keeping every 0.5.x session behaviour (kept_open, release, #181) |
| 4 | Evidence and cases: entries, local logs, key failing lines, external artifacts, screenshots, receipts and freshness over them | L | freshness parity for receipts |
| 5 | Bugs as Markdown, aliases, Markdown-aware `bug merge` | M | merge driver body rules |
| 6 | Gates hold a stage (#237, option A), `--drives`, human identity and approval or review as the human, `gate record` and `change ship` | M | Leif's #237 choice |
| 7 | Actor on every record; `chough correct` | S | none |
| 8 | Knowledge: `index.md`, journal and feed, delete trims, ledger, attachments, roll-up and generated notes | M | mostly deletion; `library index --check` parity |
| 9 | Timing reads timelines and receipts | M | lineage logic goes away; segment parity |
| 10 | `chough migrate` preview and apply, fail-closed reader, doctor findings | XL | mapping 644 history events and 60 rolled sessions exactly |
| 11 | Docs, skill bundle and installed copy, AGENTS.md, README, CHANGELOG, `.gitattributes`, version 0.6.0 | M | skill digest pins |
| 12 | The open bugs not covered by the steps above: #12 closeout, #190, #76, BUG-0001, BUG-0020, BUG-0022, BUG-0023, #148 and the flakes (testing.md) | L | the flakes may not reproduce |
| 13 | Retire KNOW-2, 7, 18, 19, 22, 23, 25, 28 and 29 with `hi retire`, in the commits that remove their requirements and cases | S | none |
| 14 | Dogfood: migrate this repository, continue this session to done in 0.6.0 | M | first real migration |

Order: 1, then 2, then 3 to 9 (3 first; 4 to 9 can run in parallel after it), then
10, 11, 12 and 13, and 14 last. Step 10 depends on every format being final; step 13
goes with the removals in steps 3 and 8.

## Ownership if several agents build

One agent owns the live session and every Chough record write; others report with
`chough inbox add`. Suggested file ownership: step 2 `src/markdown.rs`,
`src/identity.rs`, `src/leaks.rs`; step 3 `src/sessions.rs`, `src/checkpoint.rs`,
`src/history.rs`, `src/stage.rs`; step 4 `src/evidence.rs`, `src/cases.rs`,
`src/local.rs`; step 5 `src/bugs.rs`; step 6 `src/gates.rs`, `src/humans.rs`,
`src/change.rs`; step 8 `src/library.rs`, `src/vault.rs`, `src/journal.rs`; step 9
`src/timing.rs`; step 10 `src/migrate.rs`, `src/doctor.rs`; `src/main.rs` and
`tests/common/mod.rs` stay with the step 3 owner, who merges the others' wiring.

## Migration plan

1. Write `chough migrate` against fixtures first: a legacy single checkpoint with
   adopted history, open and archived sessions, rolled sessions in the vault, bugs
   of each ID form, inbox entries, external and committed artifacts, and old proof.
   Each fixture asserts the plan, the written tree, the receipt, aliases, removed IDs
   reading `removed`, idempotence and interrupted-apply recovery.
2. Run it read-only on a fresh clone of this repository at the branch head and
   review the plan: counts per kind, every redaction, every removal.
3. Apply in a throwaway clone, then check: every open and archived session readable;
   every timeline entry from the selected lineage present once, in order; every bug
   alias resolving; `chough timing` over the result; `chough doctor` clean;
   `fledge lanes run verify` green; the receipt's before digests equal to the files at
   the previous commit.
4. Apply on the branch as its own commit (step 14), so the diff is the migration. The
   76 bug records become Markdown bugs named by date and slug, each keeping its
   `BUG-NNNN` or `BUG-<slug>-<hex>` ID as an alias.

## Dogfood plan (chough repository only)

1. Keep a 0.5.0 build of this branch's base outside the working tree to drive this
   session until the migration commit, so half-built code never writes the records.
2. When steps 1 to 13 pass, build 0.6.0 and run `chough migrate`, then
   `chough migrate --apply`, and commit the result with the receipt.
3. From that commit on, drive this session with the 0.6.0 build only: test, accept
   and done in the new format. The move into done must write this change's story,
   its journal post and feed entry, keep every earlier archived story byte-identical
   (the #165 path for real), and leave a clean `chough doctor`.
4. Check the fail-closed path for real: the 0.6.0 build on an unmigrated clone of
   main refuses with `refused:unmigrated`; the 0.5.0 build on the migrated tree is
   observed and its behaviour written down for the CHANGELOG.
5. Run `fledge lanes run verify` on macOS and in the Linux images
   (`fledge run linux-verify`), and let CI's archive authentication run over the
   migrated tree; record each as evidence.
6. Touch no other repository.

## Risks

- **Scale.** Nearly every source and test file changes. Mitigation: the record layer
  first, assertions ported rather than dropped, one owner for `main.rs` and the test
  fixtures, and the full suite on every increment.
- **Migration loses something.** Mitigation: refuse unless every removed byte is in
  Git; create-new only; read-back; a full before and after receipt; dry run on a
  clone first; the receipt is checked against the previous commit.
- **Archive validity after migration.** One probe showed no change (research.md);
  the CI archive authentication over the migrated tree is the real check and gates
  the dogfood.
- **Older Chough on a migrated tree.** 0.5.x sees no checkpoint and may start a new
  one. Mitigation: the CHANGELOG says to upgrade every agent before migrating; only
  chough migrates in this change; `doctor` reports a stray 0.5.x file.
- **People edit records by hand.** Mitigation: the tolerant reader keeps unknown keys
  and prose; a refusal names file, line and rule; `doctor` catches a rewritten
  timeline.
- **Two-step finish.** Removing only committed bytes means a finished session's
  folder leaves in the commit after the story. Mitigation: the skill's closeout says
  so, and `session new`, `session use` and `doctor --fix` finish it.
- **Leak guard false positives.** A real path or name in prose is refused. Mitigation:
  `--redact`, messages naming field and kind, automatic redaction for failing lines,
  and configured human names allowed.
- **Same-day name clash across branches.** Two branches that create the same
  date and slug in the same folder conflict on merge. It needs the same title on the
  same day in the same session; the merge shows it plainly.
- **Gate decision.** If Leif chooses option B for #237, REQ-cli-030 is rewritten
  before approval; nothing else changes.
- **Pinned digests.** `tests/spec_split.rs` and the skill release manifest pin
  bodies and bundle bytes; both are updated naming this change.

## Constraints consulted

AGENTS.md (owners of each record, preserve others' files, never invent results, no
model client, runtime, service or duplicate lifecycle state, Fledge first, CI tool
pins); the CorvidLabs conventions (minimal dependencies); the cli, history and
reporting lessons (delta frozen at approval; no checkboxes for human gates or case
runs; test in adopted projects, not only fresh ones; add no native read to the move
into done; two open changes on one branch stale each other, so build starts only
after this branch is rebased onto main with #212 merged, leaving this change the
only open one); Leif's decisions listed in context.md.

## Requirement, criterion and case map

### Requirements

Of 71 existing requirements, 32 are modified, 11 removed and 28 unchanged; the records module adds REQ-records-001 to 013 and history adds REQ-history-015. An unchanged requirement that says transition, observation or checkpoint reads, in 0.6.0, a timeline entry, an evidence entry and the session front matter.

| Requirement | Subject | 0.6.0 |
|---|---|---|
| REQ-cli-001 | The helper SHALL initialize a resumable session and discoverable workf | unchanged |
| REQ-cli-002 | Focus context for resumption | unchanged |
| REQ-cli-003 | The helper SHALL report actual native SpecSync status and guidance, wi | unchanged |
| REQ-cli-004 | Bind observations to content and environment | unchanged |
| REQ-cli-005 | Validate traceability | unchanged |
| REQ-cli-006 | The helper SHALL bound assembled context and reject unsupported record | unchanged |
| REQ-cli-007 | Canonical checkpoint and explicit migration | modified |
| REQ-cli-008 | Stage routing bounded by native gates | unchanged |
| REQ-cli-009 | Validated test definitions | unchanged |
| REQ-cli-010 | Focused case selection | unchanged |
| REQ-cli-011 | Honest case execution | modified |
| REQ-cli-012 | Local bug backlog | modified |
| REQ-cli-013 | Preferences and run consent | unchanged |
| REQ-cli-014 | Optional release routing | modified |
| REQ-cli-015 | Project librarian | modified |
| REQ-cli-016 | Host skill installation | unchanged |
| REQ-cli-017 | Observation schema 2 | modified |
| REQ-cli-019 | Installable package | modified |
| REQ-cli-021 | Focused next action | modified |
| REQ-cli-022 | Approved-delta guard | modified |
| REQ-cli-023 | Slug and random record identities | modified |
| REQ-cli-029 | Agent-operated native workflow | unchanged |
| REQ-cli-030 | Open human gates hold forward moves | modified |
| REQ-cli-031 | New-project native setup hint | unchanged |
| REQ-cli-032 | Checkpoint capacity | modified |
| REQ-cli-033 | Post-archive summary | removed; carried by REQ-history-006, REQ-records-012 |
| REQ-cli-034 | Spec files have no size bound | unchanged |
| REQ-cli-036 | Backfilled stage moves | unchanged |
| REQ-cli-037 | Active session checkpoint | modified |
| REQ-cli-038 | Native verification that runs no project command | unchanged |
| REQ-cli-039 | Native check result in next | unchanged |
| REQ-cli-040 | Serialized checkpoint writers | modified |
| REQ-cli-041 | Trimming at done | removed; carried by REQ-history-006, REQ-records-011 |
| REQ-cli-042 | Native sources are read at full size | unchanged |
| REQ-cli-043 | Knowledge directory built and kept current | modified |
| REQ-cli-044 | Knowledge vault | modified |
| REQ-cli-045 | Closed-cycle references at done | removed; carried by REQ-history-006 |
| REQ-cli-046 | Loose artifacts and vault attachments | removed; carried by REQ-records-005, REQ-records-013 |
| REQ-cli-047 | No conflicting files | modified |
| REQ-cli-048 | Closed bugs release proof | removed; carried by REQ-cli-012 |
| REQ-cli-049 | Retention ledger rotation | removed; carried by none (ledger removed) |
| REQ-cli-050 | Relayed decision at a human gate | modified |
| REQ-cli-051 | Session notes | removed; carried by REQ-records-011 |
| REQ-cli-052 | Several forward moves on one native read, and one read into done | modified |
| REQ-cli-054 | Bug value, deduplication and report mappings | unchanged |
| REQ-cli-055 | Chough doctor | unchanged |
| REQ-cli-056 | Projects in a subdirectory | unchanged |
| REQ-cli-053 | Responsive archived reports with bounded authenticated reuse | unchanged |
| REQ-cli-057 | SpecSync change steps through Chough | unchanged |
| REQ-cli-058 | Linux test environment | unchanged |
| REQ-cli-059 | Ship through Chough | modified |
| REQ-cli-060 | Direct lifecycle writes noted | unchanged |
| REQ-cli-061 | Gate lists | modified |
| REQ-history-001 | Reference-aware retention | removed; carried by REQ-history-006, REQ-records-009, REQ-records-012 |
| REQ-history-002 | Immutable transition history | modified |
| REQ-history-003 | Bounded history lifecycle | modified |
| REQ-history-004 | Conservative evidence cleanup | removed; carried by REQ-records-012 |
| REQ-history-005 | Backfill on transition records | modified |
| REQ-history-006 | Sessions per objective | modified |
| REQ-history-007 | Local state directory | modified |
| REQ-history-008 | No orphan event from a vanished checkpoint | removed; carried by REQ-history-002, REQ-cli-040 |
| REQ-history-009 | Inbox entries and their sessions | modified |
| REQ-history-010 | Relayed decision on gate records | modified |
| REQ-history-011 | Archived session roll-up | removed; carried by REQ-history-006, REQ-records-012 |
| REQ-history-012 | Doctor findings and fixes | modified |
| REQ-history-013 | Direct-write events | modified |
| REQ-history-014 | Summary digests on relays | modified |
| REQ-reporting-001 | Timing report | modified |
| REQ-reporting-002 | Timing native reads | unchanged |
| REQ-reporting-003 | Request-local reporting reads | unchanged |
| REQ-reporting-004 | Timing reads native records at full size | unchanged |

### Retired criteria

Each criterion is retired in build with `hi retire <ID> "Superseded by KNOW-30 to KNOW-35: the vault links, never copies or moves; Chough 0.6.0 removes the behaviour"`, in the same commit that removes the requirements and cases below.

| Criterion | Requirements | Cases bound to it or its requirements (each disposition is in the case map) | Safeguard kept |
|---|---|---|---|
| KNOW-2 | REQ-cli-015 (rewritten, no longer traces KNOW-2) | 7: TC-cli-0035, TC-cli-0036, TC-cli-0132, TC-cli-knowledge-archive-entries, TC-cli-knowledge-entries-bounded, TC-cli-librarian-default-on-knowledge, TC-cli-slug-snapshot-allocation | none; the vault links .specsync/archive/ (KNOW-30, KNOW-31) |
| KNOW-7 | REQ-cli-033 (removed), REQ-cli-041 (removed) | 19: TC-cli-archive-summary-apply, TC-cli-archive-summary-changed-during-check, TC-cli-archive-summary-fails-closed, TC-cli-archive-summary-no-clobber-restore, TC-cli-archive-summary-preview, TC-cli-archive-summary-protection-before-each-removal, TC-cli-archive-summary-protection-during-check, TC-cli-archive-summary-stale-holds, TC-cli-archive-summary-unfiltered-head-compare, TC-cli-at-most-eight-validity-reads, TC-cli-done-trims-cold-proof-by-default, TC-cli-open-change-keeps-proof-at-done, TC-cli-recorded-retention-off-not-trimmed-at-done, TC-cli-reduce-attributed-by-cycle-time, TC-cli-reduce-parallel-validity, TC-cli-reduce-shared-artifact-released, TC-cli-reduce-superseded-runs, TC-cli-retention-manual-opts-out-at-done, TC-cli-validity-readers-on-any-host | only committed bytes ever leave the tree: REQ-records-012 |
| KNOW-18 | REQ-cli-046 (removed), REQ-cli-048 (removed) | 20: TC-cli-closed-bug-fixture-moves-to-vault, TC-cli-closed-bug-key-items-move-to-vault, TC-cli-failed-copy-keeps-other-writer, TC-cli-open-bug-keeps-links, TC-cli-rollback-keeps-changed-vault-copies, TC-cli-vault-attachment-path-bound, TC-cli-vault-attachment-written-mid-apply, TC-cli-vault-checkpoint-folder-reference, TC-cli-vault-closed-bug-releases-fixture, TC-cli-vault-identical-attachment-survives-restore, TC-cli-vault-key-items-move, TC-cli-vault-long-archive-short-folder, TC-cli-vault-loose-artifacts-trimmed, TC-cli-vault-loose-file-protected-mid-apply, TC-cli-vault-loose-group-gains-released-file, TC-cli-vault-mixed-age-folder-stays, TC-cli-vault-moved-key-item-mode, TC-cli-vault-occupied-attachment-kept, TC-cli-vault-open-bug-folder-fixture, TC-cli-vault-restore-keeps-mode | working files leave only once committed: REQ-records-012 |
| KNOW-19 | REQ-cli-046 (removed), REQ-cli-048 (removed) | 20: TC-cli-closed-bug-fixture-moves-to-vault, TC-cli-closed-bug-key-items-move-to-vault, TC-cli-failed-copy-keeps-other-writer, TC-cli-open-bug-keeps-links, TC-cli-rollback-keeps-changed-vault-copies, TC-cli-vault-attachment-path-bound, TC-cli-vault-attachment-written-mid-apply, TC-cli-vault-checkpoint-folder-reference, TC-cli-vault-closed-bug-releases-fixture, TC-cli-vault-identical-attachment-survives-restore, TC-cli-vault-key-items-move, TC-cli-vault-long-archive-short-folder, TC-cli-vault-loose-artifacts-trimmed, TC-cli-vault-loose-file-protected-mid-apply, TC-cli-vault-loose-group-gains-released-file, TC-cli-vault-mixed-age-folder-stays, TC-cli-vault-moved-key-item-mode, TC-cli-vault-occupied-attachment-kept, TC-cli-vault-open-bug-folder-fixture, TC-cli-vault-restore-keeps-mode | key screenshots beside the story: REQ-records-013 (KNOW-35) |
| KNOW-22 | REQ-cli-048 (removed) | 3: TC-cli-closed-bug-fixture-moves-to-vault, TC-cli-closed-bug-key-items-move-to-vault, TC-cli-open-bug-keeps-links | a closed bug keeps its record and receipts: REQ-cli-012 |
| KNOW-23 | REQ-cli-049 (removed) | 4: TC-cli-ledger-keeps-newest-and-moves-the-rest, TC-cli-moved-ledger-entries-are-read, TC-cli-rotation-leftover-read-once, TC-history-session-archives-itself-at-done | none; the ledger is removed with retention |
| KNOW-25 | none traces it | 0:  | the index links every lesson, per module and cross-project: REQ-cli-043 |
| KNOW-28 | none traces it | 1: TC-cli-knowledge-index-bounded-timeline | the journal and feed are the timeline of archived changes: REQ-records-011, REQ-cli-043 |
| KNOW-29 | REQ-cli-051 (removed), REQ-history-011 (removed) | 4: TC-cli-done-fresh-vault-session-links, TC-cli-session-notes-union, TC-history-keep-five-archived-sessions, TC-history-roll-up-keeps-the-anchor-holder | every archived story stays (KNOW-34): REQ-records-012, REQ-history-006 |

### Cases

283 of the repository's 426 cases are affected: 109 removed, 4 re-linked, 25 rewritten and 145 ported. A ported test keeps its function name, so its case binding does not change; a removed case goes in the same commit as the requirement or behaviour it proved, and the commit names it. Every other case links only unchanged requirements and keeps its definition.

#### Removed with the behaviour they prove (109)

| Case | File | Links | Why |
|---|---|---|---|
| TC-cli-rollback-keeps-changed-vault-copies | cli.attachment-rollback | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-0016 | cli | cli-007 | schema 1 migration; replaced by TC-records-migrate-from-0-5 |
| TC-cli-0035 | cli | cli-015 | library snapshot is removed; replaced by TC-cli-librarian-links-only |
| TC-cli-0036 | cli | cli-015 | library snapshot is removed |
| TC-cli-slug-snapshot-allocation | cli | cli-015, cli-023 | library snapshot is removed |
| TC-cli-checkpoint-merge-enforces-the-raised-bounds | cli | cli-007, cli-032 | checkpoint --merge is removed |
| TC-cli-knowledge-archive-entries | cli | cli-015, KNOW-2 | KNOW-2 retired; replaced by TC-cli-index-links-every-lesson |
| TC-cli-archive-summary-preview | cli | cli-033, KNOW-7 | its requirement is removed |
| TC-cli-archive-summary-apply | cli | cli-033, KNOW-7 | its requirement is removed |
| TC-cli-knowledge-entries-bounded | cli | cli-015, KNOW-2 | KNOW-2 retired; the bounded, refused index is in TC-cli-index-links-every-lesson |
| TC-cli-archive-summary-fails-closed | cli | cli-033 | its requirement is removed |
| TC-cli-archive-summary-changed-during-check | cli | cli-033, KNOW-7 | its requirement is removed |
| TC-cli-archive-summary-unfiltered-head-compare | cli | cli-033, KNOW-7 | its requirement is removed |
| TC-cli-archive-summary-protection-during-check | cli | cli-033 | its requirement is removed |
| TC-cli-archive-summary-protection-before-each-removal | cli | cli-033 | its requirement is removed |
| TC-cli-archive-summary-no-clobber-restore | cli | cli-033, KNOW-7 | its requirement is removed |
| TC-cli-archive-summary-stale-holds | cli | cli-033 | its requirement is removed |
| TC-cli-adopt-race-writers-save-or-refuse | cli | cli-040 | session adopt and legacy writers are removed |
| TC-cli-done-trims-cold-proof-by-default | cli | cli-041, KNOW-7 | its requirement is removed |
| TC-cli-retention-manual-opts-out-at-done | cli | cli-041 | its requirement is removed |
| TC-cli-open-change-keeps-proof-at-done | cli | cli-041 | its requirement is removed |
| TC-cli-recorded-retention-off-not-trimmed-at-done | cli | cli-041 | its requirement is removed |
| TC-cli-done-refreshes-knowledge-after-trim | cli | cli-043 | no trim at done; TC-cli-index-links-every-lesson covers the refresh |
| TC-cli-vault-archive-note-properties | cli | cli-044 | no archived-change notes (KNOW-2 retired) |
| TC-cli-vault-module-notes | cli | cli-044 | no module notes (KNOW-25 retired) |
| TC-cli-vault-intent-notes | cli | cli-044 | no intent notes (KNOW-31) |
| TC-cli-vault-rules-note | cli | cli-044 | no rules note (KNOW-31) |
| TC-cli-vault-sub-criterion-ids | cli | cli-044 | Trace reading served only intent and archive notes |
| TC-cli-vault-trace-and-family-forms | cli | cli-044 | Trace reading served only intent and archive notes |
| TC-cli-vault-inline-trace-crlf-comments | cli | cli-044 | Trace reading served only intent and archive notes |
| TC-cli-vault-readme-upgrade | cli | cli-044 | knowledge/README.md is no longer generated |
| TC-cli-vault-crlf-readme-unmatched-backtick | cli | cli-044 | knowledge/README.md is no longer generated |
| TC-cli-reduce-superseded-runs | cli | cli-033 | its requirement is removed |
| TC-cli-reduce-attributed-by-cycle-time | cli | cli-033 | its requirement is removed |
| TC-cli-reduce-done-drops-closed-cycle-refs | cli | cli-045 | its requirement is removed |
| TC-cli-reduce-parallel-validity | cli | cli-033 | its requirement is removed |
| TC-cli-reduce-shared-artifact-released | cli | cli-033 | its requirement is removed |
| TC-cli-reduce-unreadable-reference-kept | cli | cli-045 | its requirement is removed |
| TC-cli-vault-loose-artifacts-trimmed | cli | cli-046, KNOW-18 | its requirement is removed |
| TC-cli-vault-closed-bug-releases-fixture | cli | cli-046, KNOW-18 | its requirement is removed |
| TC-cli-vault-key-items-move | cli | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-vault-loose-group-gains-released-file | cli | cli-046, KNOW-18 | its requirement is removed |
| TC-cli-vault-loose-file-protected-mid-apply | cli | cli-046, KNOW-18 | its requirement is removed |
| TC-cli-vault-long-archive-short-folder | cli | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-vault-attachment-path-bound | cli | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-vault-moved-key-item-mode | cli | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-vault-restore-keeps-mode | cli | cli-046 | its requirement is removed |
| TC-cli-vault-occupied-attachment-kept | cli | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-vault-attachment-written-mid-apply | cli | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-vault-identical-attachment-survives-restore | cli | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-vault-open-bug-folder-fixture | cli | cli-046, KNOW-18 | its requirement is removed |
| TC-cli-vault-checkpoint-folder-reference | cli | cli-046, KNOW-18 | its requirement is removed |
| TC-cli-vault-mixed-age-folder-stays | cli | cli-046, KNOW-18 | its requirement is removed |
| TC-cli-closed-bug-key-items-move-to-vault | cli.closed-bugs-release-proof | cli-048, KNOW-19, KNOW-22 | its requirement is removed |
| TC-cli-closed-bug-fixture-moves-to-vault | cli.closed-bugs-release-proof | cli-048, KNOW-18, KNOW-22 | its requirement is removed |
| TC-cli-open-bug-keeps-links | cli.closed-bugs-release-proof | cli-048, KNOW-22 | its requirement is removed |
| TC-cli-done-drops-file-and-design-refs | cli.closed-cycle-file-refs | cli-045 | its requirement is removed |
| TC-cli-archived-session-refs-protect-nothing | cli.closed-cycle-file-refs | cli-045 | its requirement is removed |
| TC-cli-delivery-kinds-kept-dry-run-not | cli.delivery-kinds | cli-014 | replaced by TC-cli-delivery-kinds-as-receipts |
| TC-cli-done-fresh-vault-session-links | cli.done-vault-current | cli-051, KNOW-29 | its requirement is removed |
| TC-cli-knowledge-index-bounded-timeline | cli.knowledge-index-bound | cli-044, KNOW-28 | KNOW-28 retired; the journal is the timeline, TC-records-journal-and-feed |
| TC-cli-ledger-keeps-newest-and-moves-the-rest | cli.ledger-rotation | cli-049, KNOW-23 | its requirement is removed |
| TC-cli-moved-ledger-entries-are-read | cli.ledger-rotation | cli-049, KNOW-23 | its requirement is removed |
| TC-cli-rotation-leftover-read-once | cli.ledger-rotation | cli-049, KNOW-23 | its requirement is removed |
| TC-cli-librarian-off-references-skipped | cli.nine-criteria-assertions | cli-045 | its requirement is removed |
| TC-cli-no-reset-drops-no-observation | cli.nine-criteria-assertions | cli-045 | its requirement is removed |
| TC-cli-session-notes-union | cli.session-notes | cli-051, KNOW-29 | its requirement is removed |
| TC-cli-advance-to-single-checkpoint | cli.stage-advance-to | cli-052 | the single-checkpoint mode is removed |
| TC-cli-failed-copy-keeps-other-writer | cli.staged-vault-copies | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-validity-readers-on-any-host | cli.validity-reader-ceiling | cli-033 | its requirement is removed |
| TC-cli-at-most-eight-validity-reads | cli.validity-reader-ceiling | cli-033 | its requirement is removed |
| TC-history-roll-up-keeps-the-anchor-holder | history.anchor-holder-stays | history-011, KNOW-29 | its requirement is removed |
| TC-history-0050 | history | history-001 | its requirement is removed |
| TC-history-0051 | history | history-001 | its requirement is removed |
| TC-history-0052 | history | history-001 | its requirement is removed |
| TC-history-0053 | history | history-001 | its requirement is removed |
| TC-history-0055 | history | history-001 | its requirement is removed |
| TC-history-0056 | history | history-001 | its requirement is removed |
| TC-history-0057 | history | history-001 | its requirement is removed |
| TC-history-0058 | history | history-001 | its requirement is removed |
| TC-history-0059 | history | history-001 | its requirement is removed |
| TC-history-0060 | history | history-001 | its requirement is removed |
| TC-history-adoption | history | history-002 | history adopt is removed |
| TC-history-branches | history | history-002 | no cursors; replaced by TC-history-append-only-timeline |
| TC-history-retention | history | history-003 | history prune is removed |
| TC-history-retention-revalidation | history | history-004 | its requirement is removed |
| TC-history-legacy-timing-bounds | history | history-003 | the legacy log is removed by migration |
| TC-history-frozen-reference-aliases | history | history-003, history-004 | history prune is removed |
| TC-history-closing-reset-age | history | history-003, history-004 | history prune is removed |
| TC-history-post-prune-reference | history | history-003, history-004 | history prune is removed |
| TC-history-stale-receipt-age | history | history-003, history-004 | history prune is removed |
| TC-history-retention-protects-inbox-observations | history | history-009 | retention is removed |
| TC-history-retention-refuses-unreadable-inbox-entry | history | history-009 | retention is removed |
| TC-history-vanished-session-leaves-no-event | history | history-008 | its requirement is removed |
| TC-history-saved-event-survives-a-move | history | history-008 | its requirement is removed |
| TC-history-no-sessions-local-holds-only-lock | history | history-007 | the no-sessions mode is removed |
| TC-history-session-after-legacy-adoption | history | history-006 | legacy adoption is removed |
| TC-history-direct-write-oldest-reader-named | history.direct-writes | history-013 | clause dropped: 0.6.0 is the oldest reader of every 0.6.0 record, in the CHANGELOG |
| TC-history-summary-digest-oldest-reader-named | history.gate-lists | history-014 | clause dropped: 0.6.0 is the oldest reader of every 0.6.0 record, in the CHANGELOG |
| TC-history-keep-five-archived-sessions | history.keep-five-sessions | history-011, KNOW-29 | its requirement is removed |
| TC-history-missing-adopted-log-refuses | history.missing-log-113 | history-002, history-006 | legacy adoption is removed |
| TC-history-never-adopted-log-remains-writable | history.missing-log-113 | history-002, history-006 | legacy adoption is removed |
| TC-history-earlier-null-anchor-does-not-hide-adopter | history.missing-log-113 | history-002, history-006 | legacy adoption is removed |
| TC-history-prune-capture-byte-authority | history.pr197-p1 | history-004 | its requirement is removed |
| TC-history-prune-unsafe-restore-continuation | history.pr197-p1 | history-004 | its requirement is removed |
| TC-history-prune-protection-race | history.prune-race | history-004 | its requirement is removed |
| TC-history-prune-restores-partial-removal | history.prune-race | history-004 | its requirement is removed |
| TC-history-prune-restores-earlier-sets | history.prune-race | history-004 | its requirement is removed |
| TC-history-prune-restore-preserves-recreated-file | history.prune-race | history-004 | its requirement is removed |

#### Kept, re-linked (4)

| Case | File | Links | Why |
|---|---|---|---|
| TC-history-0054 | history | history-001 | to REQ-records-009: removed records read removed, never passed (LIFE-8) |
| TC-history-0061 | history | history-001 | to REQ-cli-012: bug show labels each observation once |
| TC-history-session-use-waits-for-stage-advance | history | history-008 | to REQ-cli-040: session use waits for a running writer |
| TC-history-session-archives-itself-at-done | history.session-archives-at-done | cli-049, history-006 | drops REQ-cli-049; asserts the finish into one story |

#### Kept, rewritten to the 0.6.0 behaviour (25)

| Case | File | Links | Why |
|---|---|---|---|
| TC-cli-0015 | cli | cli-007 | init --objective writes the first session folder |
| TC-cli-0063 | cli | cli-019 | init installs the skill and creates a session only with --objective |
| TC-cli-0073 | cli | cli-017 | migrated receipts without a content identity keep revision-bound freshness |
| TC-cli-0110 | cli | cli-019 | the SpecSync config comment joins the version agreement (BUG-0001) |
| TC-cli-0125 | cli | cli-012 | date and slug bug names; 0.5.x IDs as aliases |
| TC-cli-slug-observation-allocation | cli | cli-011, cli-017, cli-023 | date and slug evidence names |
| TC-cli-slug-validation-allocation | cli | cli-011, cli-023 | one entry per repeated validation run |
| TC-cli-checkpoint-byte-bound-is-131072 | cli | cli-032 | the session record bound of 1 MiB |
| TC-cli-delta-guard-refusals-name-the-agent-actor | cli | cli-022 | the remedy names the scope gate first, then the agent approval for standing authority |
| TC-cli-change-walkthrough-runs-chough | cli.change-ship | cli-059 | review recorded through the human gate decision |
| TC-cli-done-self-archive-vault-current | cli.done-vault-current | cli-043 | library index --check passes right after the finish |
| TC-cli-empty-change-dir-not-open | cli.empty-change-dirs | cli-043 | every reader of open changes skips an empty change directory (#148) |
| TC-cli-one-checkpoint-store-no-aps | cli.nine-criteria-assertions | cli-007 | one store, the active session file |
| TC-cli-bug-id-upgrade-note | cli.nine-criteria-assertions | cli-012 | the note says 0.5.x cannot read 0.6.0 records |
| TC-cli-relay-never-touches-native | cli.relayed-decisions | cli-050 | a gate that drives nothing leaves SpecSync unchanged |
| TC-cli-union-only-generated-notes | cli.union-generated-notes | cli-047 | only index.md and feed.xml merge by union |
| TC-history-bounds-pinned-exactly | history.bound-pins | history-003 | timeline entry and count bounds |
| TC-history-bounds | history | history-003 | timeline bounds |
| TC-history-split-keeps-requirement-wording | history | history-001, history-002, history-003, history-004 | pins updated for the removed and modified bodies, naming this change |
| TC-history-sessions-local-holds-pointer-and-lock | history | history-007 | local/ holds the listed machine state |
| TC-history-shelve-and-robust-sessions | history.checkpoint-cap | history-006 | shelve a session folder; no --legacy |
| TC-history-doctor-fixes-session-debt | history.doctor | history-012 | unmigrated replaces legacy_checkpoint |
| TC-history-relay-invalid-payloads | history.relayed-decisions | history-010 | invalid relay fields on timeline entries |
| TC-reporting-0086 | reporting | reporting-001 | bug fix times from entries, receipts and the bug Log; no ledger |
| TC-reporting-split-keeps-requirement-wording | reporting | reporting-001 | pin for REQ-reporting-001 updated naming this change |

#### Kept, binding ported (same assertions, new layout) (145)

| Case | File | Requirements |
|---|---|---|
| TC-cli-0017 | cli | cli-007 |
| TC-cli-open-human-gates-refuse-advance-and-skip | cli | cli-030 |
| TC-cli-open-human-gates-leave-other-moves-and-agree-in-next | cli | cli-030 |
| TC-cli-0021 | cli | cli-014 |
| TC-cli-0026 | cli | cli-011, cli-017 |
| TC-cli-0027 | cli | cli-011 |
| TC-cli-0028 | cli | cli-014, cli-017 |
| TC-cli-0029 | cli | cli-012 |
| TC-cli-0030 | cli | cli-012 |
| TC-cli-0031 | cli | cli-012 |
| TC-cli-0032 | cli | cli-012 |
| TC-cli-0034 | cli | cli-011 |
| TC-cli-0101 | cli | cli-011, cli-012 |
| TC-cli-0102 | cli | cli-007 |
| TC-cli-0103 | cli | cli-011, cli-014 |
| TC-cli-0039 | cli | cli-007 |
| TC-cli-0045 | cli | cli-011 |
| TC-cli-0046 | cli | cli-011 |
| TC-cli-0047 | cli | cli-011 |
| TC-cli-0048 | cli | cli-011 |
| TC-cli-0062 | cli | cli-019 |
| TC-cli-0064 | cli | cli-019 |
| TC-cli-0065 | cli | cli-019 |
| TC-cli-0066 | cli | cli-007 |
| TC-cli-0082 | cli | cli-007 |
| TC-cli-0074 | cli | cli-017 |
| TC-cli-0076 | cli | cli-012 |
| TC-cli-0077 | cli | cli-012 |
| TC-cli-0078 | cli | cli-012 |
| TC-cli-0079 | cli | cli-012 |
| TC-cli-0090 | cli | cli-019 |
| TC-cli-0091 | cli | cli-019 |
| TC-cli-0092 | cli | cli-012 |
| TC-cli-0097 | cli | cli-011 |
| TC-cli-0098 | cli | cli-011 |
| TC-cli-0099 | cli | cli-011 |
| TC-cli-0111 | cli | cli-019 |
| TC-cli-0112 | cli | cli-019 |
| TC-cli-0113 | cli | cli-019 |
| TC-cli-0114 | cli | cli-019 |
| TC-cli-0105 | cli | cli-007, cli-021 |
| TC-cli-0106 | cli | cli-007 |
| TC-cli-0120 | cli | cli-021 |
| TC-cli-0121 | cli | cli-021 |
| TC-cli-0122 | cli | cli-021 |
| TC-cli-0123 | cli | cli-022 |
| TC-cli-0124 | cli | cli-022 |
| TC-cli-0126 | cli | cli-021 |
| TC-cli-0127 | cli | cli-021 |
| TC-cli-0128 | cli | cli-021 |
| TC-cli-0129 | cli | cli-021 |
| TC-cli-0130 | cli | cli-021 |
| TC-cli-0131 | cli | cli-019 |
| TC-cli-0132 | cli | cli-015 |
| TC-cli-0133 | cli | cli-021 |
| TC-cli-bug-add-accepts-values-that-start-with-two-hyphens | cli | cli-012 |
| TC-cli-native-next-action-is-not-repeated-in-the-next-line | cli | cli-021 |
| TC-cli-exercise-cleanup-handles-a-relative-tmpdir | cli | cli-011 |
| TC-cli-same-second-runs-in-two-git-roots-stay-distinct | cli | cli-023 |
| TC-cli-independently-added-records-merge-cleanly | cli | cli-023 |
| TC-cli-observation-pointer-bound-is-400 | cli | cli-032 |
| TC-cli-librarian-default-on-knowledge | cli | cli-015 |
| TC-cli-writers-share-the-project-lock-across-tmpdir | cli | cli-040 |
| TC-cli-checkpoint-lock-in-project-ignored | cli | cli-040 |
| TC-cli-next-shows-open-questions | cli | cli-021 |
| TC-cli-folded-inbox-question-in-next | cli | cli-021 |
| TC-cli-stop-reaches-setsid-descendant | cli | cli-011 |
| TC-cli-stop-reaches-orphaned-descendant | cli | cli-011 |
| TC-cli-stop-test-leaves-other-runs-alone | cli | cli-011 |
| TC-cli-git-grace-kill-reported | cli | cli-011 |
| TC-cli-failed-head-revision-reported | cli | cli-011 |
| TC-cli-init-builds-knowledge | cli | cli-043 |
| TC-cli-init-preserves-knowledge | cli | cli-043 |
| TC-cli-init-unbounded-knowledge-refused | cli | cli-043 |
| TC-cli-done-refreshes-knowledge-untrimmed | cli | cli-043 |
| TC-cli-librarian-off-no-knowledge-at-done | cli | cli-043 |
| TC-cli-vault-front-matter-on-every-note | cli | cli-044 |
| TC-cli-vault-links-resolve | cli | cli-044 |
| TC-cli-vault-regenerates-identically | cli | cli-044 |
| TC-cli-change-ship-closes-gate-and-moves-to-done | cli.change-ship | cli-059 |
| TC-cli-change-ship-refusal-passes-through | cli.change-ship | cli-059 |
| TC-cli-change-ship-checks-before-running | cli.change-ship | cli-059 |
| TC-cli-change-ship-names-remaining-step | cli.change-ship | cli-059 |
| TC-cli-change-ship-outlives-read-bound | cli.change-ship | cli-059 |
| TC-cli-checkpoint-at-the-cap | cli.checkpoint-cap | cli-032 |
| TC-cli-full-stress-shape | cli.checkpoint-cap | cli-021 |
| TC-cli-every-delivery-kind-recorded-as-itself | cli.delivery-kinds | cli-014, cli-017 |
| TC-cli-readme-helper-table-covers-help | cli.docs-accuracy | cli-019 |
| TC-cli-readme-verify-list-matches-fledge | cli.docs-accuracy | cli-019 |
| TC-cli-documented-flags-exist | cli.docs-accuracy | cli-019 |
| TC-cli-doc-links-resolve | cli.docs-accuracy | cli-019 |
| TC-cli-documented-merge-drivers-match | cli.docs-accuracy | cli-047 |
| TC-cli-docs-name-pinned-versions | cli.docs-accuracy | cli-019 |
| TC-cli-closeout-closes-review-gate | cli.docs-accuracy | cli-050 |
| TC-cli-skill-615b587b-known-outdated | cli.docs-accuracy | cli-019 |
| TC-cli-cli-command-table-matches-help | cli.docs-accuracy | cli-019 |
| TC-cli-finished-session-not-counted-open | cli.finished-sessions | cli-037 |
| TC-cli-gate-list-summaries-and-digests | cli.gate-lists | cli-061 |
| TC-cli-gate-record-listed-gates-only | cli.gate-lists | cli-061 |
| TC-cli-gate-record-refusals-recover | cli.gate-lists | cli-061 |
| TC-cli-gate-verifying-closes-through-ship | cli.gate-lists | cli-059, cli-061 |
| TC-cli-gate-record-other-session-refused | cli.gate-lists | cli-061 |
| TC-cli-sigterm-stops-native-process | cli.native-process-liveness | cli-059 |
| TC-cli-cases-split-per-change | cli.no-conflicting-files | cli-047 |
| TC-cli-knowledge-index-check | cli.no-conflicting-files | cli-047 |
| TC-cli-bug-merge-driver | cli.no-conflicting-files | cli-047 |
| TC-cli-manual-record-returns-observation-id | cli.observation-ids-60 | cli-011 |
| TC-cli-evidence-add-returns-observation-id | cli.observation-ids-60 | cli-017 |
| TC-cli-prototype-hidden-panels-stay-hidden | cli.prototype-hidden-guard | cli-019 |
| TC-cli-prototype-styles-0-4-0-known-outdated | cli.prototype-hidden-guard | cli-019 |
| TC-cli-relay-closes-one-gate-with-four-values | cli.relayed-decisions | cli-050 |
| TC-cli-relay-refusals-recover | cli.relayed-decisions | cli-050 |
| TC-cli-relay-clear-records-none | cli.relayed-decisions | cli-030, cli-050 |
| TC-cli-skill-names-advance-to | cli.skill-advance-to | cli-052 |
| TC-cli-advance-to-one-read-in-a-session | cli.stage-advance-to | cli-052 |
| TC-cli-advance-to-no-prerequisite | cli.stage-advance-to | cli-052 |
| TC-cli-advance-to-refusal-records-nothing | cli.stage-advance-to | cli-052 |
| TC-cli-advance-into-done-one-read | cli.stage-advance-to | cli-052 |
| TC-cli-plain-advance-unchanged | cli.stage-advance-to | cli-052 |
| TC-cli-supported-platforms | cli.supported-platforms | cli-019 |
| TC-history-inbox-bounds-pinned-exactly | history.bound-pins | history-006 |
| TC-history-timing-skips-direct-writes | history.case-split-196 | history-013 |
| TC-history-inbox-add-never-stranded-by-archive | history | history-009 |
| TC-history-inbox-add-killed-leaves-no-partial-entry | history | history-009 |
| TC-history-archived-session-listed-archived | history | history-006 |
| TC-history-unrecorded-inbox-observation-warns | history | history-006 |
| TC-history-changelog-sessions-under-0-4-0 | history.changelog-release-placement | history-006 |
| TC-history-changelog-111-under-0-4-0 | history.changelog-release-placement | history-006 |
| TC-history-direct-write-field-only-on-its-kind | history.direct-writes | history-013 |
| TC-history-finished-session-archived-without-native-read | history.finished-sessions | history-006 |
| TC-history-session-close-with-reason | history.finished-sessions | history-006 |
| TC-history-relay-summary-fields-both-or-neither | history.gate-lists | history-014 |
| TC-history-switch-keeps-gated-finished-sessions-open | history.gated-switch-181 | history-006, history-012 |
| TC-history-switch-archives-sessions-left-open | history.session-archives-at-done | history-006 |
| TC-reporting-0080 | reporting | reporting-001 |
| TC-reporting-0081 | reporting | reporting-001 |
| TC-reporting-0082 | reporting | reporting-001 |
| TC-reporting-0083 | reporting | reporting-001 |
| TC-reporting-0084 | reporting | reporting-001 |
| TC-reporting-0085 | reporting | reporting-001 |
| TC-reporting-0087 | reporting | reporting-001 |
| TC-reporting-0088 | reporting | reporting-001 |
| TC-reporting-0089 | reporting | reporting-001 |
| TC-reporting-0100 | reporting | reporting-001 |
| TC-reporting-0104 | reporting | reporting-001 |
