---
change: fix-m1-migrate-refuses-before-writing
artifact: requirements
---

# Requirements

Two requirements gain sentences and criteria, and one sentence of a third changes (the deltas hold the
text):

- REQ-records-009: anything at a target or receipt name counts as taken and is never
  opened in a way that can wait; the preview says the apply would refuse; the index
  and feed are rebuilt in place; a returning record whose ID is held is refused by
  name; the writer lock is held from before the plan is read (#337, #370, #378).
- REQ-records-015: the one edit of existing text. A folder the migration empties
  that holds only macOS metadata is removed with it and named in the apply's output;
  the plan, digest and receipt still do not depend on such files (#375, option A).
- REQ-cli-055: `doctor --fix` removes the pointer only when it names no open session
  by ID, alias or bare slug (#376).
