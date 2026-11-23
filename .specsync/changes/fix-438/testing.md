---
change: fix-438
artifact: testing
---

# Testing

The regression: a session record padded so gate 1's close fits but gate
2's cumulative close exceeds SESSION_LIMIT; `gate record --gates 1,2`
refuses with nothing written and both gates still open. Also: the dry
run writes nothing and runs no native step; a normal two-gate record
still closes both. Gate and checkpoint test files, plus the change's
check lane, fmt and lint.