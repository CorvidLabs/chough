---
change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
artifact: requirements
---

# Requirements

Four modules change. The deltas are the contract; this page is the map.

## records (new module)

Adds REQ-records-001 to 013: record files and tolerant reading (001), the tree and
`.chough/README.md` (002), date and slug names with aliases (003), the session
record (004), proof files, logs and external evidence (005), the leak guard and scan
(006), the acting agent (007), a human's gate decision driving approval or review as
that human (008), migration from 0.5.x (009), failing closed on unmigrated trees
(010), the journal and feed (011), removal only of committed bytes (012) and key
screenshots beside the story (013). Owns `src/identity.rs` (moved from cli),
`src/markdown.rs`, `src/leaks.rs`, `src/local.rs`, `src/humans.rs`,
`src/migrate.rs`, `src/journal.rs` and their tests.

## history

Adds REQ-history-015 (append-only corrections). Modifies 002 (append-only timeline),
003 (timeline bounds), 005 (backfill), 006 (sessions in folders, one story at finish,
the #165 regression), 007 (local state), 009 (inbox in `agent/`), 010 (relay fields),
012 (doctor findings), 013 (direct-write entries), 014 (relay digests). Removes 001
and 004 (evidence prune and the ledger), 008 (orphan events) and 011 (the roll-up).

## cli

Modifies 007 (checkpoint), 011 (case runs), 012 (bugs), 014 (delivery kinds), 015
(librarian), 017 (evidence entries), 019 (version agreement and the toolchain check,
BUG-0001, BUG-0022), 021 (next guidance and the status summary, #76, BUG-0023), 022
(the delta guard's effective approval and the human-gate remedy, BUG-0020), 023 (names
and draws), 030 (gates
hold their stage, #237), 032 (capacity), 037 (active session), 040 (one writer
lock, with the holder named for #132), 043 (knowledge index; empty change
directories are not open changes, #148), 044 (vault), 047 (no conflicting files, bug merge),
050 (close-gate drives the native step), 052 (one read into done), 059 (ship reviews
as the human), 061 (gate record, and summaries that count only the change's own files, #190). Removes 033, 041, 045, 046, 048, 049 and 051 (the
post-archive trim, closed-cycle references, attachments, closed-bug moves, ledger
rotation and session notes).

## reporting

Modifies 001 (timing reads timelines and receipts).

## Criteria traced

DRIVE-24 to 28, KNOW-30 to 35, FLOW-22 and 23, LIFE-8, LIFE-17 to 20 and LIFE-26,
with the existing criteria each modified requirement already traced. KNOW-34 and
KNOW-35 are captured in this branch from Leif's 2026-11-10 answers.

## Unchanged

Every other requirement of cli, history and reporting keeps its wording: where it
says transition, observation or checkpoint, 0.6.0 reads a timeline entry, an
evidence entry and the session front matter (REQ-history-002, REQ-cli-017,
REQ-cli-007).

## Bugs

Every open Chough bug is mapped to a requirement and a regression case in
testing.md. Most fixes land in the requirements above; three need no requirement
change: #12 (a real native closeout test under REQ-cli-004 and REQ-cli-059) and the
two timing flakes (REQ-reporting-002 already bounds only time spent on native reads).

## Requirement map

plan.md lists every existing requirement with what 0.6.0 does to it, every retired
criterion with the requirements and cases behind it, and every affected case.
