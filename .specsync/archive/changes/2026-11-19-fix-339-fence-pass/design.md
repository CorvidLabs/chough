---
change: fix-339-fence-pass
artifact: design
---

# Design

`fenced` keeps its rule and changes how it finds each opener's close. One right-to-left pass answers, for every opener line, which later line closes it: the first later line that closes that fence character with a run at least the opener's length, before the next heading of Chough's own sections. For each fence character it keeps a stack of candidate closes, nearest first, whose run lengths grow with depth: a close is dropped once a nearer one is at least as long, a section heading of Chough's empties the stacks, and an opener's close is found by a binary search on the stack. Each line is pushed and popped at most once, so the pass is O(n log n). The forward pass then walks the lines as before, and each opener looks its close up instead of searching for it. An opener with no close still hides nothing.
