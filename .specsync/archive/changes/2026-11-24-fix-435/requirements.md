---
change: fix-435
artifact: requirements
---

# Requirements

Covers REQ-records-001 (a record is written whole or not at all, on file
systems without hard links) and FLOW-8 (records kept locally with their
evidence, whole). Observable outcome: on an exFAT volume, `chough inbox
add` succeeds and the entry file exists whole, where the hard-link
publish refused; the existing writers' behavior is unchanged.