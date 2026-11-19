---
change: fix-263
artifact: context
---

# Context

GitHub #263, found by a swarm stress test on Debian and reproduced by a second
tester. A 0.5.x project has `knowledge/INDEX.md`; the migration plan removes it
and writes `knowledge/index.md`. On a case-sensitive file system a separate
`knowledge/index.md` with other bytes existed before the apply. The apply did
not refuse. It wrote the other planned files and the receipt, removed every
0.5.x file, and only then failed on `knowledge/index.md` with "appeared while
migrating". The receipt claims bytes that were never written, and a second
apply reports nothing to do.

The cause is the pre-write check. It skips a target that differs only in case
from a planned removal, because on a case-insensitive file system reading
`knowledge/index.md` returns the old `INDEX.md`. On a case-sensitive file system
the two are separate files, and the skip let the junk file through.

REQ-records-009 already requires the refusal: the apply "SHALL refuse, writing
nothing more, when a file it would write already exists with other bytes". No
requirement text changes.
