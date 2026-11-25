---
change: fix-435
artifact: testing
---

# Testing

The exFAT path: tests/no_hard_links.rs (ignored by default, mounts an
exFAT image with hdiutil; run by hand with `cargo test --locked --test
no_hard_links -- --ignored` when safe) gains `chough inbox add` and
asserts the entry is written whole. The regular lane: the session and
inbox tests that cover publish_entry's exists-and-false contract, plus
fmt and lint. Verification commands: the change's own specsync check
lane plus those tests.