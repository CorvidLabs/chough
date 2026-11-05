---
change: move-old-proof-into-the-knowledge-vault-trim-loose-artifacts-of-cold-changes-and-move-key-items-into-knowledge
artifact: testing
---

# Testing

New tests in tests/reduce.rs, each failing first:
- Loose artifacts of a cold change are trimmed (listed by a cold archive, or committed in
  its cycle), and current or bug-held ones stay.
- A closed bug releases its loose fixture.
- Key items move into the vault (a manual observation with a screenshot, and an older
  failed run's log), and a routine record is removed.

The existing reduce, evidence, librarian and vault tests keep passing. The trim is
measured on this repository, and `fledge lanes run verify` runs on Linux.
