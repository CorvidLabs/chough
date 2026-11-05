# History context

## Decisions

2026-11-03: Split from the cli module so each requirements companion stays within
Chough's 65,536-byte structured record bound, on the parent agent's instruction at
9:46 AM MT (split rather than raise the limit, keep wording, weaken nothing).
SpecSync 6.1.2 requires `REQ-<module>-<number>` IDs in a module delta and keeps
removed IDs as permanent tombstones, so the moved requirements were renamed:

| Old ID | New ID | Subject |
|---|---|---|
| REQ-cli-018 | REQ-history-001 | Reference-aware retention |
| REQ-cli-025 | REQ-history-002 | Immutable transition history |
| REQ-cli-026 | REQ-history-003 | Bounded history lifecycle |
| REQ-cli-027 | REQ-history-004 | Conservative evidence cleanup |

The bodies are byte-identical to main 7bd1b62; tests/spec_split.rs pins their
SHA-256 digests. Archived native changes, observations, bug records and decisions
that cite the old IDs are historical records and are not rewritten. Earlier
decisions about retention and history are in the cli context and docs/DECISIONS.md
(D-017 and later).

## Working files

- src/history.rs: transition events, cursor, adoption and history prune.
- tests/history.rs and tests/evidence.rs: history and retention regressions.
- Evidence retention itself is implemented in src/evidence.rs, owned by cli.

## Lessons

Earlier retention and history lessons are in the Lessons section of
specs/cli/context.md.

2026-11-04, trimming at done is slow with many archives (verified): `stage advance` into
done with the librarian on took about 10 minutes here (78 archives), because the trim
re-reads native status for each cold archive whose files it removes (KNOW-10), and each
read costs tens of seconds. Source: #97's closeout, where accept to done was recorded at
23:51:07 and the closeout commit followed at 00:01:15, and the archived status timings
in the cli lessons. Future use: expect the move into done to be slow until SpecSync
offers a cheaper archived read (CorvidLabs/spec-sync#831), and add no further native
reads to that move. Limits: one repository; the cost grows with the cold archives the
trim touches. Related: #32.

2026-11-04, trimmed proof without a committed knowledge folder (verified): nothing built
knowledge/ except `library init` and `library index`. The trim at done wrote entries
only when it removed something, and before it removed and ledgered the files. This
repository's five trims left 85 ledger entries naming knowledge/archives/<change>.md
files that were never committed, and the one folder written, on #97's branch, was
deleted by the agent as a stray. `chough library index` regenerated all 79 entries in
about 2 seconds, with the trimmed observations as "not-run (pruned)". Source:
.chough/pruned.jsonl at main f7e5764 and a throwaway clone. Future use: REQ-cli-043
refreshes the folder after every move into done. Commit it with the checkpoint, and
treat an untracked knowledge/ after done as output to commit, never as a stray. Limits:
the entries regenerate from their sources, so nothing was lost.
