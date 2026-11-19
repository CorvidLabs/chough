---
change: fix-339-fence-pass
artifact: testing
---

# Testing

`tests/records.rs::many_unclosed_fence_openers_are_scanned_in_bounded_time`, red on fix-339 before this change: a session whose Goal holds 40,000 unclosed fence openers saves within a generous time bound, and with a conflict after them the next write is still refused, naming the conflict's line. The existing fence, section and advisory tests still pass unchanged.
