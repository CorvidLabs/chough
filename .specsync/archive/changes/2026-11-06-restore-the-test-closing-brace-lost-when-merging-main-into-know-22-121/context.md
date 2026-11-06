---
change: restore-the-test-closing-brace-lost-when-merging-main-into-know-22-121
artifact: context
---

# Context

Leif chose "Record and merge" for #121 (KNOW-22) on 2026-11-05 at 17:07 MST. He asked for `main` to be merged in after #120 (KNOW-21) landed, with no force push, and for a fresh review if the merge changed the reviewed change.

In merge `e08266c1`, both branches had appended tests to the end of `tests/reduce.rs`, after a shared closing brace. The resolution kept only one brace, so `main`'s last test lost its `}` and the test target stopped compiling. Commit `b3868b84` restores that one brace. That commit changed a meaningful path after the KNOW-22 archive, so this change covers it.
