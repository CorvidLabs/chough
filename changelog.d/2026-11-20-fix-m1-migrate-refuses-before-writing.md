- `chough migrate` reads every target, and every entry of the receipt folder, without
  following a link or waiting, and refuses anything in the way before it writes,
  naming the path relative to the project (#337).
- A 0.5.x file that comes back after a migration is judged by the committed receipt
  that lists it: with those bytes it is removed as already migrated; with others the
  preview and the apply refuse, naming it, the receipt and the way on (#370).
- An apply takes the writer lock before it plans, so of several applies of one digest
  only one applies (#378).
- No 0.5.x file is removed before every planned file is on disk; `knowledge/index.md`
  beside `knowledge/INDEX.md` is written by rename, and a regular generated index or
  feed is replaced as the plan says under `replaces` (#338).
- A folder the migration empties, left holding only macOS metadata, goes with it and is
  named (#375).
- `chough doctor --fix` keeps a pointer that names an open session by alias or bare
  slug (#376).
