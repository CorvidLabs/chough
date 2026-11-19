---
change: fix-m1-migrate-refuses-before-writing
artifact: context
---

# Context

Four reports from the 0.6.2 candidate swarm about `chough migrate` and the doctor run
after it (#376, #370, #337, #378). #375 (folders left holding only Finder files) is in as its option A, a folder left
holding only macOS metadata is removed with it after the apply and named in the
output, not in the plan or the receipt; it comes out again if Leif picks another
option. #338 (a Linux chough on a Docker Desktop bind mount) is in: tester2 reproduced it on
main in 6 of 6 fresh clones on that mount and 0 of 4 on overlayfs. The case-only
change `INDEX.md` to `index.md` is written by create-new as the last step, after the
receipt and the removals, and fails there on a stale name; sources are gone and the
index is missing. tester2 also confirmed #337 on Linux: an empty folder at
`knowledge/index.md` ends with every source removed.

Reproduced on main (9d260310) with the built binary on scratch 0.5.x fixtures:

- #376: with a kept 0.5.x pointer (`open-one`), `chough doctor --fix` after the
  migration reports `stale_pointer` fixed, removes the pointer, and `chough next` then
  refuses with several open sessions. The after-fix check in `src/doctor.rs` reads the
  raw pointer; the read-only check already resolves aliases. The issue's other half,
  "migration writes the 0.6 id", is not done: migration never writes the pointer, and
  REQ-history-006 says an in-place migration keeps it.
- #370: a 0.5.x bug that comes back with an edited title is planned as a second bug
  holding the same 0.5.x ID; the apply refuses on `knowledge/index.md`, and with the
  index deleted by hand two bugs hold one alias. A new 0.5.x bug on a second migration
  is refused on the index too.
- #337: a directory at `knowledge/index.md` or at the receipt name gives a partial
  write and an absolute path in the error; a FIFO at the receipt name makes the
  preview wait for ever; a file that cannot be read or is not UTF-8 at the receipt
  name is taken as free. Not reproduced on macOS: sources being removed (that needs a
  case-sensitive file system, where the index write comes after the removals).
- #378: of three applies of one digest, two or three exit 0 in half the rounds; the
  losers print `applied: true` with nothing written. The plan and every check run
  before the writer lock is taken.

On real projects (tester2, four 0.5.x projects, macOS and Linux) #370 is wider than
the alias: a returning edited `session.json` became a second open session, and a
returning edited history event was planned as removed for a reason that was not the
true one. The first reproduces on the scratch fixture: an edited `open-one.json`
becomes `2026-11-19-open-one` beside `2026-11-11-open-one`, and `next` refuses with
several open sessions. The receipt already lists every file an earlier migration read
with its SHA-256 (its Before section), so the rule is stated on the receipt.

Receipts as input, checked on real ones: this repository's own receipt of
2026-11-12 (1,577,630 bytes, under the 4,194,304-byte receipt bound) and jackdaw's of
2026-11-13 and 2026-11-16 all carry `## Before` with one line per file read, its path
and SHA-256, the format this build writes. Not tried on the binary, and to be pinned
by the first failing tests: a returning file whose session has since been archived,
and a returning fixed-path `.chough/session.json` that holds a different session.

Telling an earlier migration's receipt from this migration's own: the receipt is
written before any 0.5.x file is removed, so an interrupted apply leaves every source
present with the bytes its receipt lists. The two are told apart by Git, not by the
plan: a finished migration is committed (a file can only come back through Git after
that), while an interrupted apply's receipt is not committed. A receipt Git has never
held judges nothing and is used again as today. A receipt Git holds but that differs
on disk from its committed bytes is neither kind: treating it as the migration's own
would let an uncommitted edit switch the guard off, so it refuses by name. Walked through for a stop before the
receipt (no receipt, the plan as before), after the receipt and partway through the
removals (the receipt is uncommitted, so it judges nothing and the apply goes on as
today). Tried on the binary by putting sources back after a full apply, which is not
a faithful stop: the next preview had another digest and the apply wrote a second
receipt, with no duplicate session. So the criterion promises what the existing text
does, that the next apply completes with the same result, and not a same digest. If someone commits a half-applied tree, its sources
are then removed as already migrated, which is right because every 0.6.0 file is
written before the receipt.

Not in this scope: a hand-made file under `.chough/` that is no 0.5.x record kind,
such as a `.chough/proposals/` note, is neither carried, removed nor mentioned.

Also seen and covered by the same sentences: the preview never says that the apply
would refuse on an existing target.
