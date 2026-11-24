---
change: fix-437
artifact: testing
---

# Testing

A direct-native-write entry with an absent state reads back with
before_state: none and after_state: none, and its text says `state stays
none`. The session/history tests covering direct-write entries cover it;
the change's own check lane plus fmt and lint.