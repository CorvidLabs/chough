---
change: build-the-knowledge-directory-by-default-chough-init-creates-it-and-every-move-into-done-refreshes-it-after-the-trim
artifact: testing
---

# Testing

New tests, each failing before the change:
- tests/knowledge_default.rs: init builds the directory; init preserves an existing one;
  init still succeeds and writes no knowledge when the index cannot be bounded.
- tests/librarian_trim.rs: done refreshes the entries after the trim (the trimmed proof
  reads as pruned); done refreshes the index when nothing is trimmed; a librarian-off
  project gets no knowledge directory at done.

Two existing tests change with the requirement. A refused `library init` test in
tests/evidence.rs removes the folder that init now creates first, and its assertions are
unchanged. tests/optional.rs expects the index right after init, where it expected none. Cases bind the 6 new
tests. `fledge lanes run verify` runs on Linux.
