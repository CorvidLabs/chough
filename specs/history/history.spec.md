---
module: history
version: 10
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
