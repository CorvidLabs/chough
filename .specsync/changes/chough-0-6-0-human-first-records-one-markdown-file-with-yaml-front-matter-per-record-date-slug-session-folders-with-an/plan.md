---
change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
artifact: plan
---

# Plan

## Approach

Replace Chough's storage layer in one change, behind the commands people and agents
already use. Build a small shared Markdown record layer first, move each record kind
onto it, delete the 0.5.x machinery the new layout makes unnecessary (event files,
trims, ledger, vault copies), then write the one-way migration. This change runs on the 0.5.0 build; once it is
archived, this repository is migrated and the next change is the dogfood (KNOW-11). Keep every
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
| 12 | The open bugs not covered by the steps above: #12 (records out of SpecSync's verification, and the closeout), #190, #76, BUG-0001, BUG-0020, BUG-0022, BUG-0023, #148 and the flakes (testing.md) | L | the flakes may not reproduce |
| 13 | Retire the 18 criteria #212 marks superseded (KNOW-2, 7, 8, 9, 13, 16 to 23, 25 to 29) with `hi retire`, in the commits that remove their requirements and cases | S | none |
| 14 | Dogfood: rehearse the migration on a clone before accept; after archive, migrate this repository and run the next change intent to done in 0.6.0 | M | first real migration |

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

1. Write `chough migrate` against fixtures first: one captured from a project at
   every released version, 0.1.0 to 0.5.0, built from its release tag (#213), and
   synthetic ones for a legacy single checkpoint with adopted history, open and
   archived sessions, rolled sessions in the vault (the adopter among them), bugs of
   each ID form, inbox entries, external and committed artifacts, home-folder paths
   in records, and old proof. Each fixture asserts the plan and its digest, the
   written tree, the receipt, aliases, removed IDs reading `removed`, redactions,
   idempotence, interrupted-apply recovery, and the restore-and-refuse path when an
   archive would read invalid.
2. Run it read-only on a fresh clone of this repository at the branch head and
   review the plan: counts per kind, every redaction (the audit counted 96 committed
   files under `.chough/` and `knowledge/` holding home-folder paths), every removal,
   the 60 rolled-up sessions coming back as archived stories, and the plan digest.
3. Apply in a throwaway clone, then check: every open and archived session readable;
   every timeline entry from the selected lineage present once, in order; every bug
   alias resolving; `chough timing` over the result; `chough doctor` clean;
   `fledge lanes run verify` green; the receipt's before digests equal to the files at
   the previous commit.
4. Apply on the branch as its own commit (step 14) with `chough migrate --apply
   <digest>` from that preview, so the diff is the migration and the archive check
   runs against all 169 archives (at most 8 reads at once). Git history is not
   rewritten (Leif: "No, clean from 0.6.0 on"): it keeps the old paths, which are
   the user name and folder layout only, since a secret-pattern scan of origin/main
   found no keys or tokens; the receipt says so. The
   76 bug records become Markdown bugs named by date and slug, each keeping its
   `BUG-NNNN` or `BUG-<slug>-<hex>` ID as an alias.

## Dogfood plan (chough repository only)

The upgrade never runs while a native change is open (KNOW-11, as #212 carries it
forward), so this repository migrates after this change is archived, and the dogfood
is the next change on this branch.

1. Drive this whole change, intent to done, with a 0.5.0 build of this branch's base
   kept outside the working tree, so half-built code never writes the records. During
   build, add `.chough/sessions/`, `.chough/archive/` and `.chough/bugs/` to
   `verification_ignored_paths` in `.specsync/sdd.json` (REQ-records-014). This
   change's affected paths name `.chough/cases/`, `.chough/README.md` and
   `.specsync/sdd.json` rather than all of `.chough/`, because SpecSync refuses a
   change whose paths overlap that list.
2. Before accept, rehearse the migration of this branch on a throwaway clone with the
   0.6.0 build: preview, apply, receipt, and every archive still valid. Leif's
   acceptance review covers the code and that rehearsal.
3. After the move into done, with no native change open, build 0.6.0 and run `chough
   migrate`, then `chough migrate --apply <digest> --consent "<source>"`, and commit
   the result with the receipt. The consent is Leif's: approving this plan's scope
   consents to this one migration of this repository, and the agent records it citing
   that approval and naming itself (FLOW-24). Close the bug records already fixed
   elsewhere with their links (#148 via #157, BUG-0001, the history anchor via #142,
   the trace flake via #172) and BUG-0002 and BUG-0003 as `fixed-upstream`
   (spec-sync #823 and #824, re-checked by t37b on 46f55652).
4. Then the dogfood cycle: the 0.6.0 release preparation (the CHANGELOG release date
   and the dogfood results in the upgrade notes, with any fix the dogfood finds), as
   one small native change driven intent to done with the 0.6.0 build only. It must
   show: Chough writes after `change check --commit` leave verification current (the
   #12 path for real); approval and review as the human; the move into done writes its
   story, journal post and feed entry and keeps every earlier archived story, migrated
   ones included, byte-identical (the #165 path for real); the two-step closeout
   (commit the story, then `chough doctor --fix` and commit that; a session only ever
   closes itself, Q3); and a clean `chough doctor`.
5. Check the fail-closed path for real: the 0.6.0 build on an unmigrated clone of
   main refuses with `refused:unmigrated`; the 0.5.0 build on the migrated tree is
   observed and its behaviour written down for the CHANGELOG.
6. Run `fledge lanes run verify` on macOS and in the Linux images
   (`fledge run linux-verify`), and let CI's archive authentication run over the
   migrated tree; record each as evidence.
7. The pull request merges only after the dogfood cycle, so main never holds 0.6.0
   code with 0.5.x records. Touch no other repository.

## Ticket ledger

Leif (2026-11-11, relayed by the coordinator): "we need to make sure we use chough to
fix and close all the bugs and tickets for 0.6.0 before tagging 0.6.0.. we need to
dogfood and make sure chough is perfect". Every open GitHub issue and pull request in
CorvidLabs/chough and every open local bug record gets exactly one disposition:

1. fixed in 0.6.0, naming the regression case that proves it;
2. already fixed on main, naming the proof; closed after a fresh re-check;
3. superseded by 0.6.0; closed with a link once 0.6.0 merges;
4. after 0.6.0: only the per-repository dogfood issues (#221 to #229), the 1.0 freeze
   and epic (#231, #232) and anything Leif himself deferred, quoted.

No agent defers anything else. Every 1, 2 and 3 on GitHub is in the "Chough 0.6.0"
milestone; nothing is closed yet. Read on 2026-11-11.

### Open GitHub issues

| Issue | Disposition | Proof |
|---|---|---|
| #213 Migration fixtures from every 0.x tag | 1 | TC-records-migrate-from-0-5 (a fixture from every release, 0.1.0 to 0.5.0, built from its tag) |
| #214 Per-session slug folders | 1 | TC-history-sessions-in-folders |
| #215 Digest-committed evidence receipts | 1 | TC-records-proof-split-and-external (receipt with command, key lines and fingerprint; no raw log), TC-history-doctor-findings-0-6 (`oversize` fails the verify lane) |
| #216 All of KNOW: librarian, index, observations | 1 | TC-records-journal-and-feed, TC-records-tree-and-readme, and the KNOW gap list check (testing.md, Dogfood checks) |
| #217 Flake tracker and two-week clean window | needs Leif: 4 proposed | not in this plan; the two-week window needs two weeks of CI history after the fixes |
| #218 Host-matrix e2e harness | needs Leif: 4 proposed | not in this plan |
| #219 integrations/ folder | needs Leif: 4 proposed | not in this plan; it depends on #218 |
| #220 Qualify one local agent setup | needs Leif: 4 proposed | not in this plan; it depends on #218 |
| #221 to #229 Dogfood other repositories | 4 | Leif, 2026-11-10: "the other repos update only after that dogfood passes" (roadmap M6) |
| #230 Weekly 0.x releases | needs Leif: 4 proposed | a cadence that starts after 0.6.0; 0.6.0 itself waits for Leif's go |
| #231 Freeze the format and ship 1.0 | 4 | named by the coordinator's rule |
| #232 Chough 1.0 epic | 4 | named by the coordinator's rule |
| #234 Committed records must never leak | 1 | TC-records-leak-guard |
| #235 Human-first records, one source of truth | 1 | TC-records-files-are-markdown-and-tolerant, TC-records-journal-and-feed |
| #236 Gate approval drives SpecSync as that human | 1 | TC-records-approval-as-the-human |
| #237 An open gate blocks every stage advance | 1 | TC-cli-237-gate-holds-its-stage |

### Open pull requests

| Pull request | Disposition | Proof |
|---|---|---|
| #208 External artifacts by fingerprint (#14) | 3, closed 2026-11-11, folded into #238 | folded into REQ-records-005, the record holding a neutral ID instead of the path |
| #210 Append-only corrections (#167) | 3, closed 2026-11-11, folded into #238 | folded into REQ-history-015 |
| #211 Roll-up copy-never-move (#165) | 3, closed 2026-11-11, folded into #238 | superseded by KNOW-30 and KNOW-34 |
| #233 Agent, model and terminal on records (#201) | 3, closed 2026-11-11, folded into #238 | folded into REQ-records-007 |

### Open local bug records

| Bug | Disposition | Proof |
|---|---|---|
| BUG-0001 config comment names stale versions | 2 | the comment on main names 6.2.0; TC-cli-0110 keeps it in the version check |
| BUG-0002 approval stays valid after a delta edit | 2 | fixed upstream, spec-sync #823; t37b re-checked on spec-sync 46f55652 |
| BUG-0003 a failing check records passed | 2 | fixed upstream, spec-sync #824; t37b re-checked on spec-sync 46f55652 |
| BUG-0020 delta guard uses the latest approval | 1 | TC-cli-bug-0020-guard-uses-effective-approval |
| BUG-0022 local tools differ from CI | 1 | TC-cli-bug-0022-toolchain-matches-ci |
| BUG-0023 cold-start output hard to scan | 1 | TC-cli-bug-0023-status-leads-with-summary |
| BUG-…-empty-change-directory (#148) | 1 | TC-cli-empty-change-dir-not-open (#157 fixed the index; 0.6.0 covers every reader) |
| BUG-…-history-writes-refuse-once-the-adopting-session-rolls-into-the-vault | 2 | #142 (merged 90e5de6f); TC-records-migrate-vaulted-adopter stays as the regression |
| BUG-…-full-macos-lane-checkpoint-writer-lock-timeout | 1 | TC-cli-132-fork-inheritance-stress |
| BUG-…-timing-native-budget-…-no-initial-stub-call | 1 | TC-reporting-budget-starts-at-first-read |
| BUG-…-timing-trace-…-cross-a-second | 2 | #172 (merged b8088923), TC-reporting-timing-trace-straddles-a-second |
| BUG-…-verify-lane-test-step-fails-intermittently-under-load | 1 | TC-cli-132-fork-inheritance-stress |

### Reopened issues

#12, #14, #76, #132, #165, #167, #190 and #201 were closed on 2026-11-11 by the
`corvid-agent` account, with no comment and no fix, and were reopened the same day and
put in the Chough 0.6.0 milestone. Each is disposition 1, fixed in 0.6.0 with its case
(testing.md): TC-cli-12-native-closeout-stays-current and the two #12 repro cases,
TC-records-proof-split-and-external, TC-cli-76-next-guides-a-fresh-agent,
TC-cli-132-fork-inheritance-stress, TC-history-165-earlier-archives-stay,
TC-history-append-only-corrections, TC-cli-190-gate-summary-counts-the-change and
TC-records-actor-recorded.

Counted: 32 items in 0.6.0 (16 issues, 12 local bugs, and 4 pull requests already
closed on 2026-11-11 and folded into #238), 5 of them already fixed on main or upstream;
11 after 0.6.0 (#221 to #229, #231, #232); 5 wait for Leif's word (#217 to #220, #230).

## Release gate

The release gate (Leif, 2026-11-11, relayed by the coordinator), verbatim: every item
in that milestone closed through Chough (bug record -> fix -> regression case ->
evidence -> closed), the chough-repo dogfood complete with every bug it finds fixed the
same way, then Leif says go.

Earlier, Leif (2026-11-10): "yes.. but we dog food and fix all bugs before tagging and
releasing". So:

- No 0.6.0 tag and no release until the chough-repository dogfood above is complete.
- Every bug found in dogfood or testing is fixed with its regression case before the
  tag, and `chough bug list` shows zero open Chough bugs at tag time.
- A flake that never reproduces cannot be closed as fixed. If #132 or the lock or
  budget flakes are still open after their diagnostics and stress runs, that is
  brought to Leif before tagging, never closed to clear the gate.
- The release itself, the tag and crates.io publication, still waits for Leif's
  one-line go (LIFE-25).
- After 0.6.0 ships: refresh the shared skill copy the hc agents load with
  `chough skill update` (the audit found it still reads session.json), then other
  repositories, only at 95 percent confidence or better.

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
- **SpecSync's review after a committed Chough record.** Verification currency is a
  content check only, so the exclusion covers it; whether review also binds after a
  Chough commit is what TC-records-write-after-check-keeps-verification shows. If it
  does not, that part goes to spec-sync and the documented order stays for commits.
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
- **Gate decision.** Planned as Leif's recommended default (a pending decision
  holds only its own step); if he overrides at scope approval, REQ-cli-030 is
  rewritten before approval and nothing else changes.
- **The zero-open-bugs gate and flakes.** A flake that will not reproduce cannot be
  verified; the release gate sends that case to Leif rather than closing it.
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

Of 71 existing requirements, 35 are modified, 11 removed and 25 unchanged; the records module adds REQ-records-001 to 014 and history adds REQ-history-015. An unchanged requirement that says transition, observation or checkpoint reads, in 0.6.0, a timeline entry, an evidence entry and the session front matter.

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
| REQ-cli-029 | Agent-operated native workflow | modified |
| REQ-cli-030 | Open human gates hold forward moves | modified |
| REQ-cli-031 | New-project native setup hint | unchanged |
| REQ-cli-032 | Checkpoint capacity | modified |
| REQ-cli-033 | Post-archive summary | removed; carried by REQ-history-006, REQ-records-012 |
| REQ-cli-034 | Spec files have no size bound | modified |
| REQ-cli-036 | Backfilled stage moves | unchanged |
| REQ-cli-037 | Active session checkpoint | modified |
| REQ-cli-038 | Native verification that runs no project command | unchanged |
| REQ-cli-039 | Native check result in next | unchanged |
| REQ-cli-040 | Serialized checkpoint writers | modified |
| REQ-cli-041 | Trimming at done | removed; carried by REQ-history-006, REQ-records-011 |
| REQ-cli-042 | Native sources are read at full size | modified |
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

Leif approved retiring the old cleanup and vault-copy rules ("Yes, make it match", 2026-11-10). #212 marks them superseded; this change retires each with `hi retire <ID> "Superseded by KNOW-30 to KNOW-35: the vault links, never copies or moves; Chough 0.6.0 removes the behaviour"` in the commit that removes the requirements and cases behind it. The list below was checked against #212's merged text (88e53eba): it marks 18 superseded, and carries KNOW-10 and KNOW-11 forward as the upgrade's rules instead, so those two are not retired and now trace to REQ-records-009; KNOW-24 is reworded rather than retired: every kind of file has a stated rule, finished stories stay for good, working files go when the work finishes, and machine-only files are never committed (REQ-records-002).

| Criterion | Requirements tracing it | Cases bound to it or those requirements (each disposition is in the case map) | Safeguard kept |
|---|---|---|---|
| KNOW-2 | REQ-cli-015 (rewritten without it) | 7: TC-cli-0035, TC-cli-0036, TC-cli-0132, TC-cli-knowledge-archive-entries, TC-cli-knowledge-entries-bounded, TC-cli-librarian-default-on-knowledge, TC-cli-slug-snapshot-allocation | none; the vault links .specsync/archive/ |
| KNOW-7 | REQ-cli-033 (removed), REQ-cli-041 (removed) | 19: TC-cli-archive-summary-apply, TC-cli-archive-summary-changed-during-check, TC-cli-archive-summary-fails-closed, TC-cli-archive-summary-no-clobber-restore, TC-cli-archive-summary-preview, TC-cli-archive-summary-protection-before-each-removal, TC-cli-archive-summary-protection-during-check, TC-cli-archive-summary-stale-holds, TC-cli-archive-summary-unfiltered-head-compare, TC-cli-at-most-eight-validity-reads, TC-cli-done-trims-cold-proof-by-default, TC-cli-open-change-keeps-proof-at-done, TC-cli-recorded-retention-off-not-trimmed-at-done, TC-cli-reduce-attributed-by-cycle-time, TC-cli-reduce-parallel-validity, TC-cli-reduce-shared-artifact-released, TC-cli-reduce-superseded-runs, TC-cli-retention-manual-opts-out-at-done, TC-cli-validity-readers-on-any-host | only committed bytes ever leave the tree (REQ-records-012) |
| KNOW-8 | REQ-cli-033 (removed) | 15: TC-cli-archive-summary-apply, TC-cli-archive-summary-changed-during-check, TC-cli-archive-summary-fails-closed, TC-cli-archive-summary-no-clobber-restore, TC-cli-archive-summary-preview, TC-cli-archive-summary-protection-before-each-removal, TC-cli-archive-summary-protection-during-check, TC-cli-archive-summary-stale-holds, TC-cli-archive-summary-unfiltered-head-compare, TC-cli-at-most-eight-validity-reads, TC-cli-reduce-attributed-by-cycle-time, TC-cli-reduce-parallel-validity, TC-cli-reduce-shared-artifact-released, TC-cli-reduce-superseded-runs, TC-cli-validity-readers-on-any-host | none; no raw proof is committed (LIFE-19) |
| KNOW-9 | REQ-cli-033 (removed), REQ-cli-046 (removed) | 32: TC-cli-archive-summary-apply, TC-cli-archive-summary-changed-during-check, TC-cli-archive-summary-fails-closed, TC-cli-archive-summary-no-clobber-restore, TC-cli-archive-summary-preview, TC-cli-archive-summary-protection-before-each-removal, TC-cli-archive-summary-protection-during-check, TC-cli-archive-summary-stale-holds, TC-cli-archive-summary-unfiltered-head-compare, TC-cli-at-most-eight-validity-reads, TC-cli-failed-copy-keeps-other-writer, TC-cli-reduce-attributed-by-cycle-time, TC-cli-reduce-parallel-validity, TC-cli-reduce-shared-artifact-released, TC-cli-reduce-superseded-runs, TC-cli-rollback-keeps-changed-vault-copies, TC-cli-validity-readers-on-any-host, TC-cli-vault-attachment-path-bound, TC-cli-vault-attachment-written-mid-apply, TC-cli-vault-checkpoint-folder-reference, TC-cli-vault-closed-bug-releases-fixture, TC-cli-vault-identical-attachment-survives-restore, TC-cli-vault-key-items-move, TC-cli-vault-long-archive-short-folder, TC-cli-vault-loose-artifacts-trimmed, TC-cli-vault-loose-file-protected-mid-apply, TC-cli-vault-loose-group-gains-released-file, TC-cli-vault-mixed-age-folder-stays, TC-cli-vault-moved-key-item-mode, TC-cli-vault-occupied-attachment-kept, TC-cli-vault-open-bug-folder-fixture, TC-cli-vault-restore-keeps-mode | the migration receipt lists every removed file with its digest (REQ-records-009) |
| KNOW-10 (kept: #212 carries it forward) | REQ-cli-033 (removed), REQ-cli-046 (removed); now traced by REQ-records-009 | 32: TC-cli-archive-summary-apply, TC-cli-archive-summary-changed-during-check, TC-cli-archive-summary-fails-closed, TC-cli-archive-summary-no-clobber-restore, TC-cli-archive-summary-preview, TC-cli-archive-summary-protection-before-each-removal, TC-cli-archive-summary-protection-during-check, TC-cli-archive-summary-stale-holds, TC-cli-archive-summary-unfiltered-head-compare, TC-cli-at-most-eight-validity-reads, TC-cli-failed-copy-keeps-other-writer, TC-cli-reduce-attributed-by-cycle-time, TC-cli-reduce-parallel-validity, TC-cli-reduce-shared-artifact-released, TC-cli-reduce-superseded-runs, TC-cli-rollback-keeps-changed-vault-copies, TC-cli-validity-readers-on-any-host, TC-cli-vault-attachment-path-bound, TC-cli-vault-attachment-written-mid-apply, TC-cli-vault-checkpoint-folder-reference, TC-cli-vault-closed-bug-releases-fixture, TC-cli-vault-identical-attachment-survives-restore, TC-cli-vault-key-items-move, TC-cli-vault-long-archive-short-folder, TC-cli-vault-loose-artifacts-trimmed, TC-cli-vault-loose-file-protected-mid-apply, TC-cli-vault-loose-group-gains-released-file, TC-cli-vault-mixed-age-folder-stays, TC-cli-vault-moved-key-item-mode, TC-cli-vault-occupied-attachment-kept, TC-cli-vault-open-bug-folder-fixture, TC-cli-vault-restore-keeps-mode | migration never weakens a SpecSync archive (REQ-records-009) |
| KNOW-11 (kept: #212 carries it forward) | REQ-cli-033 (removed), REQ-cli-041 (removed); now traced by REQ-records-009 | 19: TC-cli-archive-summary-apply, TC-cli-archive-summary-changed-during-check, TC-cli-archive-summary-fails-closed, TC-cli-archive-summary-no-clobber-restore, TC-cli-archive-summary-preview, TC-cli-archive-summary-protection-before-each-removal, TC-cli-archive-summary-protection-during-check, TC-cli-archive-summary-stale-holds, TC-cli-archive-summary-unfiltered-head-compare, TC-cli-at-most-eight-validity-reads, TC-cli-done-trims-cold-proof-by-default, TC-cli-open-change-keeps-proof-at-done, TC-cli-recorded-retention-off-not-trimmed-at-done, TC-cli-reduce-attributed-by-cycle-time, TC-cli-reduce-parallel-validity, TC-cli-reduce-shared-artifact-released, TC-cli-reduce-superseded-runs, TC-cli-retention-manual-opts-out-at-done, TC-cli-validity-readers-on-any-host | migration previews first, applies only the previewed plan, only with consent and never while a change is open (REQ-records-009) |
| KNOW-13 | REQ-cli-041 (removed), REQ-cli-043 (rewritten without it), REQ-cli-045 (removed), REQ-cli-048 (removed), REQ-cli-049 (removed), REQ-history-011 (removed) | 27: TC-cli-archived-session-refs-protect-nothing, TC-cli-closed-bug-fixture-moves-to-vault, TC-cli-closed-bug-key-items-move-to-vault, TC-cli-done-drops-file-and-design-refs, TC-cli-done-refreshes-knowledge-after-trim, TC-cli-done-refreshes-knowledge-untrimmed, TC-cli-done-self-archive-vault-current, TC-cli-done-trims-cold-proof-by-default, TC-cli-empty-change-dir-not-open, TC-cli-init-builds-knowledge, TC-cli-init-preserves-knowledge, TC-cli-init-unbounded-knowledge-refused, TC-cli-ledger-keeps-newest-and-moves-the-rest, TC-cli-librarian-off-no-knowledge-at-done, TC-cli-librarian-off-references-skipped, TC-cli-moved-ledger-entries-are-read, TC-cli-no-reset-drops-no-observation, TC-cli-open-bug-keeps-links, TC-cli-open-change-keeps-proof-at-done, TC-cli-recorded-retention-off-not-trimmed-at-done, TC-cli-reduce-done-drops-closed-cycle-refs, TC-cli-reduce-unreadable-reference-kept, TC-cli-retention-manual-opts-out-at-done, TC-cli-rotation-leftover-read-once, TC-history-keep-five-archived-sessions, TC-history-roll-up-keeps-the-anchor-holder, TC-history-session-archives-itself-at-done | none; nothing is trimmed |
| KNOW-16 | REQ-cli-045 (removed) | 7: TC-cli-archived-session-refs-protect-nothing, TC-cli-done-drops-file-and-design-refs, TC-cli-librarian-off-references-skipped, TC-cli-no-reset-drops-no-observation, TC-cli-reduce-done-drops-closed-cycle-refs, TC-cli-reduce-unreadable-reference-kept, TC-cli-vault-checkpoint-folder-reference | none; references end with their session |
| KNOW-17 | REQ-cli-033 (removed) | 15: TC-cli-archive-summary-apply, TC-cli-archive-summary-changed-during-check, TC-cli-archive-summary-fails-closed, TC-cli-archive-summary-no-clobber-restore, TC-cli-archive-summary-preview, TC-cli-archive-summary-protection-before-each-removal, TC-cli-archive-summary-protection-during-check, TC-cli-archive-summary-stale-holds, TC-cli-archive-summary-unfiltered-head-compare, TC-cli-at-most-eight-validity-reads, TC-cli-reduce-attributed-by-cycle-time, TC-cli-reduce-parallel-validity, TC-cli-reduce-shared-artifact-released, TC-cli-reduce-superseded-runs, TC-cli-validity-readers-on-any-host | none |
| KNOW-18 | REQ-cli-046 (removed), REQ-cli-048 (removed) | 20: TC-cli-closed-bug-fixture-moves-to-vault, TC-cli-closed-bug-key-items-move-to-vault, TC-cli-failed-copy-keeps-other-writer, TC-cli-open-bug-keeps-links, TC-cli-rollback-keeps-changed-vault-copies, TC-cli-vault-attachment-path-bound, TC-cli-vault-attachment-written-mid-apply, TC-cli-vault-checkpoint-folder-reference, TC-cli-vault-closed-bug-releases-fixture, TC-cli-vault-identical-attachment-survives-restore, TC-cli-vault-key-items-move, TC-cli-vault-long-archive-short-folder, TC-cli-vault-loose-artifacts-trimmed, TC-cli-vault-loose-file-protected-mid-apply, TC-cli-vault-loose-group-gains-released-file, TC-cli-vault-mixed-age-folder-stays, TC-cli-vault-moved-key-item-mode, TC-cli-vault-occupied-attachment-kept, TC-cli-vault-open-bug-folder-fixture, TC-cli-vault-restore-keeps-mode | working files leave only once committed (REQ-records-012) |
| KNOW-19 | REQ-cli-046 (removed), REQ-cli-048 (removed) | 20: TC-cli-closed-bug-fixture-moves-to-vault, TC-cli-closed-bug-key-items-move-to-vault, TC-cli-failed-copy-keeps-other-writer, TC-cli-open-bug-keeps-links, TC-cli-rollback-keeps-changed-vault-copies, TC-cli-vault-attachment-path-bound, TC-cli-vault-attachment-written-mid-apply, TC-cli-vault-checkpoint-folder-reference, TC-cli-vault-closed-bug-releases-fixture, TC-cli-vault-identical-attachment-survives-restore, TC-cli-vault-key-items-move, TC-cli-vault-long-archive-short-folder, TC-cli-vault-loose-artifacts-trimmed, TC-cli-vault-loose-file-protected-mid-apply, TC-cli-vault-loose-group-gains-released-file, TC-cli-vault-mixed-age-folder-stays, TC-cli-vault-moved-key-item-mode, TC-cli-vault-occupied-attachment-kept, TC-cli-vault-open-bug-folder-fixture, TC-cli-vault-restore-keeps-mode | key screenshots beside the story (REQ-records-013) |
| KNOW-20 | REQ-cli-033 (removed), REQ-cli-046 (removed) | 32: TC-cli-archive-summary-apply, TC-cli-archive-summary-changed-during-check, TC-cli-archive-summary-fails-closed, TC-cli-archive-summary-no-clobber-restore, TC-cli-archive-summary-preview, TC-cli-archive-summary-protection-before-each-removal, TC-cli-archive-summary-protection-during-check, TC-cli-archive-summary-stale-holds, TC-cli-archive-summary-unfiltered-head-compare, TC-cli-at-most-eight-validity-reads, TC-cli-failed-copy-keeps-other-writer, TC-cli-reduce-attributed-by-cycle-time, TC-cli-reduce-parallel-validity, TC-cli-reduce-shared-artifact-released, TC-cli-reduce-superseded-runs, TC-cli-rollback-keeps-changed-vault-copies, TC-cli-validity-readers-on-any-host, TC-cli-vault-attachment-path-bound, TC-cli-vault-attachment-written-mid-apply, TC-cli-vault-checkpoint-folder-reference, TC-cli-vault-closed-bug-releases-fixture, TC-cli-vault-identical-attachment-survives-restore, TC-cli-vault-key-items-move, TC-cli-vault-long-archive-short-folder, TC-cli-vault-loose-artifacts-trimmed, TC-cli-vault-loose-file-protected-mid-apply, TC-cli-vault-loose-group-gains-released-file, TC-cli-vault-mixed-age-folder-stays, TC-cli-vault-moved-key-item-mode, TC-cli-vault-occupied-attachment-kept, TC-cli-vault-open-bug-folder-fixture, TC-cli-vault-restore-keeps-mode | none |
| KNOW-21 | REQ-cli-045 (removed) | 6: TC-cli-archived-session-refs-protect-nothing, TC-cli-done-drops-file-and-design-refs, TC-cli-librarian-off-references-skipped, TC-cli-no-reset-drops-no-observation, TC-cli-reduce-done-drops-closed-cycle-refs, TC-cli-reduce-unreadable-reference-kept | none |
| KNOW-22 | REQ-cli-048 (removed) | 3: TC-cli-closed-bug-fixture-moves-to-vault, TC-cli-closed-bug-key-items-move-to-vault, TC-cli-open-bug-keeps-links | a closed bug keeps its record and receipts (REQ-cli-012) |
| KNOW-23 | REQ-cli-049 (removed) | 4: TC-cli-ledger-keeps-newest-and-moves-the-rest, TC-cli-moved-ledger-entries-are-read, TC-cli-rotation-leftover-read-once, TC-history-session-archives-itself-at-done | none; no ledger |
| KNOW-25 | none | 0 | the index links every lesson (REQ-cli-043) |
| KNOW-26 | none | 0 | the archived story leads with goal, outcome and decisions (REQ-history-006) |
| KNOW-27 | none | 0 | the bug file is the write-up (REQ-cli-012) |
| KNOW-28 | none | 1: TC-cli-knowledge-index-bounded-timeline | the journal and feed are the timeline (REQ-records-011, REQ-cli-043) |
| KNOW-29 | REQ-cli-051 (removed), REQ-history-011 (removed) | 4: TC-cli-done-fresh-vault-session-links, TC-cli-session-notes-union, TC-history-keep-five-archived-sessions, TC-history-roll-up-keeps-the-anchor-holder | every archived story stays (KNOW-34, REQ-records-012) |

### Cases

304 of the repository's 426 cases are affected: 102 removed, 3 re-linked, 71 rewritten and 128 ported. A ported test keeps its function name, so its case binding does not change; a removed case goes in the same commit as the requirement or behaviour it proved, and the commit names it. Every other case links only unchanged requirements and keeps its definition.

#### Removed with the behaviour they prove (102)

| Case | File | Links | Why |
|---|---|---|---|
| TC-cli-rollback-keeps-changed-vault-copies | cli.attachment-rollback | cli-046, KNOW-19 | its requirement is removed |
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
| TC-cli-done-trims-cold-proof-by-default | cli | cli-041, KNOW-7 | its requirement is removed |
| TC-cli-retention-manual-opts-out-at-done | cli | cli-041 | its requirement is removed |
| TC-cli-open-change-keeps-proof-at-done | cli | cli-041 | its requirement is removed |
| TC-cli-recorded-retention-off-not-trimmed-at-done | cli | cli-041 | its requirement is removed |
| TC-cli-done-refreshes-knowledge-after-trim | cli | cli-043 | no trim at done; TC-cli-index-links-every-lesson covers the refresh |
| TC-cli-vault-archive-note-properties | cli | cli-044 | no archived-change notes (KNOW-2 retired) |
| TC-cli-vault-module-notes | cli | cli-044 | no module notes (KNOW-25 retired) |
| TC-cli-vault-intent-notes | cli | cli-044 | no intent notes (KNOW-31) |
| TC-cli-vault-sub-criterion-ids | cli | cli-044 | Trace reading served only intent and archive notes |
| TC-cli-vault-trace-and-family-forms | cli | cli-044 | Trace reading served only intent and archive notes |
| TC-cli-vault-inline-trace-crlf-comments | cli | cli-044 | Trace reading served only intent and archive notes |
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
| TC-cli-done-fresh-vault-session-links | cli.done-vault-current | cli-051, KNOW-29 | its requirement is removed |
| TC-cli-done-self-archive-vault-current | cli.done-vault-current | cli-043 | its file goes with the roll-up (audit); the check right after done is in TC-cli-index-links-every-lesson |
| TC-cli-knowledge-index-bounded-timeline | cli.knowledge-index-bound | cli-044, KNOW-28 | KNOW-28 retired; the journal is the timeline, TC-records-journal-and-feed |
| TC-cli-ledger-keeps-newest-and-moves-the-rest | cli.ledger-rotation | cli-049, KNOW-23 | its requirement is removed |
| TC-cli-moved-ledger-entries-are-read | cli.ledger-rotation | cli-049, KNOW-23 | its requirement is removed |
| TC-cli-rotation-leftover-read-once | cli.ledger-rotation | cli-049, KNOW-23 | its requirement is removed |
| TC-cli-librarian-off-references-skipped | cli.nine-criteria-assertions | cli-045 | its requirement is removed |
| TC-cli-no-reset-drops-no-observation | cli.nine-criteria-assertions | cli-045 | its requirement is removed |
| TC-cli-session-notes-union | cli.session-notes | cli-051, KNOW-29 | its requirement is removed |
| TC-cli-advance-to-single-checkpoint | cli.stage-advance-to | cli-052 | the single-checkpoint mode is removed |
| TC-cli-failed-copy-keeps-other-writer | cli.staged-vault-copies | cli-046, KNOW-19 | its requirement is removed |
| TC-cli-union-only-generated-notes | cli.union-generated-notes | cli-047 | generated notes and union merges go (audit); TC-cli-markdown-bug-merge covers the generated-file driver |
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
| TC-history-0061 | history | history-001 | retention labels go with REQ-history-001 (audit) |
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
| TC-history-prune-capture-byte-authority | history.pr197-p1 | history-004 | its requirement is removed |
| TC-history-prune-unsafe-restore-continuation | history.pr197-p1 | history-004 | its requirement is removed |
| TC-history-prune-protection-race | history.prune-race | history-004 | its requirement is removed |
| TC-history-prune-restores-partial-removal | history.prune-race | history-004 | its requirement is removed |
| TC-history-prune-restores-earlier-sets | history.prune-race | history-004 | its requirement is removed |
| TC-history-prune-restore-preserves-recreated-file | history.prune-race | history-004 | its requirement is removed |

#### Kept, re-linked (3)

| Case | File | Links | Why |
|---|---|---|---|
| TC-history-0054 | history | history-001 | to REQ-records-009: a removed record reads removed, never passed (LIFE-8); the audit lists it with REQ-history-001, but LIFE-8 still applies to migrated IDs |
| TC-history-session-use-waits-for-stage-advance | history | history-008 | to REQ-cli-040: session use waits for a running writer |
| TC-history-session-archives-itself-at-done | history.session-archives-at-done | cli-049, history-006 | drops REQ-cli-049; asserts the finish into one story |

#### Kept, rewritten to the 0.6.0 behaviour (71)

| Case | File | Links | Why |
|---|---|---|---|
| TC-cli-0015 | cli | cli-007 | init --objective writes the first session folder |
| TC-cli-0016 | cli | cli-007 | explicit migration now means 0.5.x to 0.6.0 (audit) |
| TC-cli-0035 | cli | cli-015 | drop the snapshot half; the index still lists open bugs (audit) |
| TC-cli-0063 | cli | cli-019 | init installs the skill and creates a session only with --objective |
| TC-cli-0073 | cli | cli-017 | migrated receipts without a content identity keep revision-bound freshness |
| TC-cli-0110 | cli | cli-019 | the SpecSync config comment joins the version agreement (BUG-0001) |
| TC-cli-0125 | cli | cli-012 | date and slug bug names; 0.5.x IDs as aliases |
| TC-cli-slug-observation-allocation | cli | cli-011, cli-017, cli-023 | date and slug evidence names |
| TC-cli-slug-validation-allocation | cli | cli-011, cli-023 | one entry per repeated validation run |
| TC-cli-checkpoint-byte-bound-is-131072 | cli | cli-032 | the session record bound of 1 MiB |
| TC-cli-delta-guard-refusals-name-the-agent-actor | cli | cli-022 | the remedy names the scope gate first, then the agent approval for standing authority |
| TC-cli-adopt-race-writers-save-or-refuse | cli | cli-040 | legacy adoption becomes part of migrate (audit) |
| TC-cli-folded-inbox-question-in-next | cli | cli-021 | the inbox lives in agent/ (audit) |
| TC-cli-archived-finalization-over-record-bound | cli | cli-042 | re-pointed at the finished-session reader (audit, REQ-cli-042) |
| TC-cli-vault-front-matter-on-every-note | cli | cli-044 | over index.md, journal posts and lessons (audit) |
| TC-cli-vault-rules-note | cli | cli-044 | its links move into index.md (audit) |
| TC-cli-vault-links-resolve | cli | cli-044 | over the new vault (audit) |
| TC-cli-vault-regenerates-identically | cli | cli-044 | only index.md and feed.xml (audit) |
| TC-cli-vault-readme-upgrade | cli | cli-044 | the old README folds into index.md (audit) |
| TC-cli-vault-crlf-readme-unmatched-backtick | cli | cli-044 | the same, for a CRLF checkout (audit) |
| TC-cli-change-ship-closes-gate-and-moves-to-done | cli.change-ship | cli-059 | its file is reworked for 0.6.0 (audit) |
| TC-cli-change-ship-refusal-passes-through | cli.change-ship | cli-059 | its file is reworked for 0.6.0 (audit) |
| TC-cli-change-ship-checks-before-running | cli.change-ship | cli-059 | its file is reworked for 0.6.0 (audit) |
| TC-cli-change-ship-names-remaining-step | cli.change-ship | cli-059 | its file is reworked for 0.6.0 (audit) |
| TC-cli-change-ship-outlives-read-bound | cli.change-ship | cli-059 | its file is reworked for 0.6.0 (audit) |
| TC-cli-change-walkthrough-runs-chough | cli.change-ship | cli-059 | review recorded through the human gate decision |
| TC-cli-checkpoint-at-the-cap | cli.checkpoint-cap | cli-032 | its file is reworked for 0.6.0 (audit) |
| TC-cli-full-stress-shape | cli.checkpoint-cap | cli-021 | its file is reworked for 0.6.0 (audit) |
| TC-cli-delivery-kinds-kept-dry-run-not | cli.delivery-kinds | cli-014 | drop moved into the vault; kinds stay as receipts (audit) |
| TC-cli-direct-write-recorded-once | cli.direct-writes |  | its file is reworked for 0.6.0 (audit) |
| TC-cli-chough-steps-not-flagged | cli.direct-writes |  | its file is reworked for 0.6.0 (audit) |
| TC-cli-direct-write-first-sight | cli.direct-writes |  | its file is reworked for 0.6.0 (audit) |
| TC-cli-direct-write-shown-not-written | cli.direct-writes |  | its file is reworked for 0.6.0 (audit) |
| TC-cli-direct-write-before-ship | cli.direct-writes |  | its file is reworked for 0.6.0 (audit) |
| TC-cli-documented-merge-drivers-match | cli.docs-accuracy | cli-047 | the session driver goes; bug and generated-file drivers change (audit) |
| TC-cli-doctor-report-changes-nothing | cli.doctor |  | its file is reworked for 0.6.0 (audit) |
| TC-cli-empty-change-dir-not-open | cli.empty-change-dirs | cli-043 | every reader of open changes skips an empty change directory (#148) |
| TC-cli-gate-list-summaries-and-digests | cli.gate-lists | cli-061 | its file is reworked for 0.6.0 (audit) |
| TC-cli-gate-record-listed-gates-only | cli.gate-lists | cli-061 | its file is reworked for 0.6.0 (audit) |
| TC-cli-gate-record-refusals-recover | cli.gate-lists | cli-061 | its file is reworked for 0.6.0 (audit) |
| TC-cli-gate-verifying-closes-through-ship | cli.gate-lists | cli-059, cli-061 | its file is reworked for 0.6.0 (audit) |
| TC-cli-gate-record-other-session-refused | cli.gate-lists | cli-061 | its file is reworked for 0.6.0 (audit) |
| TC-cli-one-checkpoint-store-no-aps | cli.nine-criteria-assertions | cli-007 | one store, the active session file |
| TC-cli-bug-id-upgrade-note | cli.nine-criteria-assertions | cli-012 | the note says 0.5.x cannot read 0.6.0 records |
| TC-cli-authority-before-approval | cli.nine-criteria-assertions | cli-029 | approval or review on authority given in advance names the agent as `<agent>-<terminal>` and quotes the human (Q2, FLOW-24, the standing rule of 2026-11-10) |
| TC-cli-manual-record-returns-observation-id | cli.observation-ids-60 | cli-011 | date and slug IDs (audit) |
| TC-cli-evidence-add-returns-observation-id | cli.observation-ids-60 | cli-017 | date and slug IDs (audit) |
| TC-cli-relay-closes-one-gate-with-four-values | cli.relayed-decisions | cli-050 | its file is reworked for 0.6.0 (audit) |
| TC-cli-relay-refusals-recover | cli.relayed-decisions | cli-050 | its file is reworked for 0.6.0 (audit) |
| TC-cli-relay-never-touches-native | cli.relayed-decisions | cli-050 | a gate that drives nothing leaves SpecSync unchanged |
| TC-cli-relay-clear-records-none | cli.relayed-decisions | cli-030, cli-050 | its file is reworked for 0.6.0 (audit) |
| TC-cli-advance-into-done-one-read | cli.stage-advance-to | cli-052 | no trim manifest to reuse (audit) |
| TC-cli-supported-platforms | cli.supported-platforms | cli-019 | Windows is best-effort through WSL (audit) |
| TC-history-bounds-pinned-exactly | history.bound-pins | history-003 | timeline entry and count bounds |
| TC-history-inbox-bounds-pinned-exactly | history.bound-pins | history-006 | its file is reworked for 0.6.0 (audit) |
| TC-history-bounds | history | history-003 | timeline bounds |
| TC-history-split-keeps-requirement-wording | history | history-001, history-002, history-003, history-004 | pins updated for the removed and modified bodies, naming this change |
| TC-history-sessions-local-holds-pointer-and-lock | history | history-007 | local/ holds the listed machine state |
| TC-history-shelve-and-robust-sessions | history.checkpoint-cap | history-006 | shelve a session folder; no --legacy |
| TC-history-direct-write-field-only-on-its-kind | history.direct-writes | history-013 | its file is reworked for 0.6.0 (audit) |
| TC-history-doctor-fixes-session-debt | history.doctor | history-012 | unmigrated replaces legacy_checkpoint |
| TC-history-finished-session-archived-without-native-read | history.finished-sessions | history-006 | listed as finished; only doctor --fix archives it (Q3) |
| TC-history-session-close-with-reason | history.finished-sessions | history-006 | its file is reworked for 0.6.0 (audit) |
| TC-history-switch-keeps-gated-finished-sessions-open | history.gated-switch-181 | history-006, history-012 | a switch never archives another session (Q3) |
| TC-history-missing-adopted-log-refuses | history.missing-log-113 | history-002, history-006 | a migration precondition: a missing adopted log refuses migrate, naming it (audit) |
| TC-history-never-adopted-log-remains-writable | history.missing-log-113 | history-002, history-006 | a migration precondition (audit) |
| TC-history-earlier-null-anchor-does-not-hide-adopter | history.missing-log-113 | history-002, history-006 | a migration precondition (audit) |
| TC-history-relay-invalid-payloads | history.relayed-decisions | history-010 | invalid relay fields on timeline entries |
| TC-history-switch-archives-sessions-left-open | history.session-archives-at-done | history-006 | a switch never archives another session (Q3) |
| TC-reporting-0086 | reporting | reporting-001 | bug fix times from entries, receipts and the bug Log; no ledger |
| TC-reporting-split-keeps-requirement-wording | reporting | reporting-001 | pin for REQ-reporting-001 updated naming this change |

#### Kept, binding ported (same assertions, new layout) (128)

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
| TC-cli-agent-native-workflow-upgrade | cli | cli-029 |
| TC-cli-native-workflow-bundle-consistency | cli | cli-029 |
| TC-cli-bug-add-accepts-values-that-start-with-two-hyphens | cli | cli-012 |
| TC-cli-native-next-action-is-not-repeated-in-the-next-line | cli | cli-021 |
| TC-cli-exercise-cleanup-handles-a-relative-tmpdir | cli | cli-011 |
| TC-cli-same-second-runs-in-two-git-roots-stay-distinct | cli | cli-023 |
| TC-cli-independently-added-records-merge-cleanly | cli | cli-023 |
| TC-cli-observation-pointer-bound-is-400 | cli | cli-032 |
| TC-cli-librarian-default-on-knowledge | cli | cli-015 |
| TC-cli-spec-files-read-at-any-size | cli | cli-034 |
| TC-cli-build-reference-from-main-known-outdated | cli | cli-034 |
| TC-cli-next-searches-large-context-md | cli | cli-034 |
| TC-cli-writers-share-the-project-lock-across-tmpdir | cli | cli-040 |
| TC-cli-checkpoint-lock-in-project-ignored | cli | cli-040 |
| TC-cli-next-shows-open-questions | cli | cli-021 |
| TC-cli-hi-export-over-16-kib-validates | cli | cli-042 |
| TC-cli-archived-change-md-over-record-bound | cli | cli-042 |
| TC-cli-sdd-policy-over-record-bound | cli | cli-042 |
| TC-cli-display-cut-points-to-source | cli | cli-042 |
| TC-cli-skill-bounds-never-justify-native-edits | cli | cli-042 |
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
| TC-cli-every-delivery-kind-recorded-as-itself | cli.delivery-kinds | cli-014, cli-017 |
| TC-cli-readme-helper-table-covers-help | cli.docs-accuracy | cli-019 |
| TC-cli-readme-verify-list-matches-fledge | cli.docs-accuracy | cli-019 |
| TC-cli-documented-flags-exist | cli.docs-accuracy | cli-019 |
| TC-cli-doc-links-resolve | cli.docs-accuracy | cli-019 |
| TC-cli-docs-name-pinned-versions | cli.docs-accuracy | cli-019 |
| TC-cli-closeout-closes-review-gate | cli.docs-accuracy | cli-050 |
| TC-cli-skill-615b587b-known-outdated | cli.docs-accuracy | cli-019 |
| TC-cli-cli-command-table-matches-help | cli.docs-accuracy | cli-019 |
| TC-cli-finished-session-not-counted-open | cli.finished-sessions | cli-037 |
| TC-cli-sigterm-stops-native-process | cli.native-process-liveness | cli-059 |
| TC-cli-claimed-approvals-open-no-gate | cli.nine-criteria-assertions | cli-029 |
| TC-cli-skipped-cold-start-not-a-pass | cli.nine-criteria-assertions | cli-029 |
| TC-cli-roadmap-claims-no-cold-start-pass | cli.nine-criteria-assertions | cli-029 |
| TC-cli-cases-split-per-change | cli.no-conflicting-files | cli-047 |
| TC-cli-knowledge-index-check | cli.no-conflicting-files | cli-047 |
| TC-cli-bug-merge-driver | cli.no-conflicting-files | cli-047 |
| TC-cli-prototype-hidden-panels-stay-hidden | cli.prototype-hidden-guard | cli-019 |
| TC-cli-prototype-styles-0-4-0-known-outdated | cli.prototype-hidden-guard | cli-019 |
| TC-cli-skill-names-advance-to | cli.skill-advance-to | cli-052 |
| TC-cli-advance-to-one-read-in-a-session | cli.stage-advance-to | cli-052 |
| TC-cli-advance-to-no-prerequisite | cli.stage-advance-to | cli-052 |
| TC-cli-advance-to-refusal-records-nothing | cli.stage-advance-to | cli-052 |
| TC-cli-plain-advance-unchanged | cli.stage-advance-to | cli-052 |
| TC-history-timing-skips-direct-writes | history.case-split-196 | history-013 |
| TC-history-inbox-add-never-stranded-by-archive | history | history-009 |
| TC-history-inbox-add-killed-leaves-no-partial-entry | history | history-009 |
| TC-history-archived-session-listed-archived | history | history-006 |
| TC-history-unrecorded-inbox-observation-warns | history | history-006 |
| TC-history-changelog-sessions-under-0-4-0 | history.changelog-release-placement | history-006 |
| TC-history-changelog-111-under-0-4-0 | history.changelog-release-placement | history-006 |
| TC-history-relay-summary-fields-both-or-neither | history.gate-lists | history-014 |
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
