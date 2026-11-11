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
| 12 | Dogfood: migrate this repository, continue this session to done in 0.6.0 | M | first real migration |

Order: 1, then 2, then 3 to 9 (3 first; 4 to 9 can run in parallel after it), then
10, 11 and 12. Step 10 depends on every format being final.

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
4. Apply on the branch as its own commit (step 12), so the diff is the migration.

## Dogfood plan (chough repository only)

1. Keep a 0.5.0 build of this branch's base outside the working tree to drive this
   session until the migration commit, so half-built code never writes the records.
2. When steps 1 to 11 pass, build 0.6.0 and run `chough migrate`, then
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
