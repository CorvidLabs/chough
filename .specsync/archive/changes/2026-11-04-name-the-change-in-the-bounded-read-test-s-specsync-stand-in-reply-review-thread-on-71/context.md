---
change: name-the-change-in-the-bounded-read-test-s-specsync-stand-in-reply-review-thread-on-71
artifact: context
---

# Context

The review-thread fix on #71 (change refuse-a-native-change-status-reply-that-names-no-change-id-issue-63-review-thread-on-71) makes `checkpoint --native-change` refuse a SpecSync reply that names no change ID. The shared SpecSync stand-in in tests/cli.rs replies `{"next_action":...,"state":"draft"}`, which names none, so `native_reads_end_by_their_deadline_and_retain_bounded_output` failed at its setup checkpoint. Only that test's setup reply changes; no assertion changes.
