---
change: fix-m1-migrate-refuses-before-writing
artifact: context
---

# Context

Four reports from the 0.6.2 candidate swarm about `chough migrate` and the doctor run
after it (#376, #370, #337, #378). Two more wait for Leif's choice and are not in this
scope yet: #375 (folders left holding only Finder files) and #338 (a Linux chough on a
Docker Desktop bind mount).

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

Also seen and covered by the same sentences: the preview never says that the apply
would refuse on an existing target.
