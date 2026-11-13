---
change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
artifact: design
---

# Design

Every write that adds an observation reference (`cases run`, `cases record`, `evidence
add`, `checkpoint --observation`) checks the cap before it runs a case or writes an
entry, and refuses naming the bound and `chough session shelve <id>`. `next` takes a
case's latest run only from the observations the checkpoint references. No contract
text changes: REQ-cli-032 and REQ-cli-021 already say this.
