---
module: history
version: 21
status: active
files:
  - src/history.rs
  - tests/history.rs
  - tests/evidence.rs
  - tests/spec_split.rs
  - src/sessions.rs
  - tests/sessions.rs

db_tables: []
depends_on:
  - cli
---

# Chough history and retention

## Purpose

Own Chough's transition history and the retention of its records: immutable,
bounded transition events behind one checkpoint cursor, and previewed, authorized,
reference-aware removal of observation sets and closed history cycles. This module
was split from cli on 2026-11-03 so each requirements companion stays within the
65,536-byte record bound; REQ-history-001 to REQ-history-004 keep the exact wording
of REQ-cli-018 and REQ-cli-025 to REQ-cli-027 (mapping in context.md and
docs/FRD.md). See requirements.md.

## Public API

The binary exposes CLI commands; it does not export a Rust library API. This module
covers the `history` command group (`history adopt`, `history prune`) and the
retention behavior of `evidence prune`, as documented in docs/cli.md. The cli module
owns the command parser and every other command group.

## Invariants

1. Adoption preview and prune preview write nothing; only an explicit `--apply`
   with existing human retention authority removes anything.
2. The legacy transitions log, migration backup, native archives, ignore rules and
   Git history are never rewritten by retention or adoption.
3. Event publication precedes atomic checkpoint and cursor publication, and the
   cursor stays in the single schema-2 checkpoint's current-cycle merge group.
4. Pruned and unavailable observations or history are reported as such and never
   treated as current, passed, zero or verified.
5. Every read and enumeration is bounded, and an incomplete scan refuses apply.

## Behavioral Examples

```gherkin
Given observation sets referenced by the checkpoint, a bug or a recent archived change
When evidence retention previews
Then those sets are listed as protected with reasons and no file changes

Given a project with a legacy transitions log
When history adopt --apply runs twice
Then the original log and migration bytes are preserved and no event is duplicated

Given closed history cycles and a human retention preference
When history prune --apply runs
Then a bounded receipt is written before deletion and protected cycles remain
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Missing retention authority | Apply runs without a run consent or standing retention preference | Refuse without removing anything |
| Changed candidate | A candidate file changed, vanished or became referenced after preview | Retain or refuse it with an explicit outcome |
| Bound exceeded | An event, cycle, traversal or scan exceeds its bound | Refuse before checkpoint mutation, or report partial history |
| Anchor mismatch | The adopted legacy anchor no longer matches | Refuse new writes |
| Captured artifact identity changed | Bytes captured after eligibility validation do not match the eligible artifact digest | Refuse before this set's ledger write/removal; retain the replacement and restore any earlier removals |
| Unsafe restoration destination | A removed path or ancestor is recreated as a symlink | Keep its identity and target untouched; preserve original bytes in a safe named hold, continue remaining held-file restores and record honest restored accounting for every ledgered set |

## Dependencies

Depends on the cli module for the command parser, checkpoint, records, bounded
reads and atomic writes. Git supplies revisions; no external service is used.

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-11-03 | Split from cli with REQ-cli-018 and REQ-cli-025 to REQ-cli-027 renamed REQ-history-001 to REQ-history-004, wording unchanged |
| 2 | 2026-11-03 | split-the-cli-spec-move-the-retention-and-transition-history-requirements-req-cli-018-and-req-cli-025-to-req-cli-027-to: Split the cli spec: move the retention and transition-history requirements REQ-cli-018 and REQ-cli-025 to REQ-cli-027 to a new history module as REQ-history-001 to REQ-history-004 with wording unchanged and a documented ID mapping, so the cli requirements companion has headroom under the 65536-byte record bound |
| 3 | 2026-11-04 | record-a-stage-gate-that-already-happened-with-its-original-time-quoted-evidence-and-a-backfilled-flag-that-timing: Record a stage gate that already happened with its original time, quoted evidence and a backfilled flag that timing reports separately (DRIVE-5) |
| 4 | 2026-11-04 | keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer: Keep one session per objective or native change under .chough/sessions with append-only transitions, an active pointer, automatic archiving and an append-only subagent inbox (DRIVE-3) |
| 5 | 2026-11-04 | show-open-questions-in-next-and-status-report-an-archived-session-as-archived-and-warn-when-an-inbox-observation-has-no: Show open questions in next and status, report an archived session as archived, and warn when an inbox observation has no record (issue 84) |
| 6 | 2026-11-04 | keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy: Keep the checkpoint writer lock in the project so every temp directory shares it, name the file being written when busy, and specify serialized writers and the local state directory (issue 83) |
| 7 | 2026-11-04 | remove-the-history-events-a-command-published-when-its-save-is-refused-because-the-session-file-vanished-and-test-that: Remove the history events a command published when its save is refused because the session file vanished, and test that session use waits for stage advance (issue 82) |
| 8 | 2026-11-04 | lock-inbox-add-against-a-concurrent-archive-and-protect-observations-named-by-unfolded-inbox-entries-from-retention: Lock inbox add against a concurrent archive and protect observations named by unfolded inbox entries from retention (issue 98) |
| 9 | 2026-11-06 | size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each: Size the checkpoint from measured use: a 1 MiB cap with at-cap budgets, session shelve, and sessions that never break each other (DRIVE-14, 125) |
| 10 | 2026-11-06 | keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29: Keep the five most recent archived sessions and roll older ones into the knowledge vault (KNOW-29) |
| 11 | 2026-11-06 | record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7: Record relayed human decisions at gates as claims, not authentication (123, D7) |
| 12 | 2026-11-06 | keep-the-archived-session-that-holds-the-legacy-history-anchor-out-of-the-roll-up-143: Keep the archived session that holds the legacy history anchor out of the roll-up (143) |
| 13 | 2026-11-06 | a-session-archives-itself-at-its-own-move-into-done-155: A session archives itself at its own move into done (155) |
| 13 | 2026-11-06 | a-session-whose-native-change-is-archived-is-finished-and-abandoned-work-closes-with-a-reason-163-drive-16: A session whose native change is archived is finished, and abandoned work closes with a reason (163, DRIVE-16) |
| 14 | 2026-11-06 | chough-doctor-finds-and-fixes-the-chough-debt-that-stops-a-fresh-worktree-from-entering-chough-drive-17: Chough doctor finds and fixes the Chough debt that stops a fresh worktree from entering Chough (DRIVE-17) |
| 15 | 2026-11-07 | flag-direct-specsync-lifecycle-writes-in-history-183-drive-21: Flag direct SpecSync lifecycle writes in history (183, DRIVE-21) |
| 16 | 2026-11-07 | chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20: Chough gate list: one-screen gate summaries with digests, and one reply for listed gates only (184, FLOW-19, FLOW-20) |
| 17 | 2026-11-07 | chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21: chough timing skips direct-native-write events, and chough change new passes --no-spec-change (189, DRIVE-21, DRIVE-20) |
| 18 | 2026-11-07 | make-evidence-prune-apply-recheck-protections-and-restore-partial-removals: Make evidence prune --apply recheck protections and restore partial removals (65) |
| 19 | 2026-11-07 | correct-prune-capture-authority-and-unsafe-rollback-for-pr-197: Correct prune capture authority and unsafe rollback for PR 197 |
| 20 | 2026-11-10 | keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181: Keep finished sessions with open human gates open on session use/new, naming doctor's decision fix (#181) |
| 21 | 2026-11-12 | chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an: Chough 0.6.0 human-first records: one Markdown file with YAML front matter per record, date+slug session folders with an append-only timeline that fold into one archived story at finish, a knowledge vault that links and never copies, no local facts or credentials in committed records, a human's gate approval drives the SpecSync approval as that human, and chough migrate converts 0.5.x projects with a digest receipt |
