---
change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
artifact: requirements
---

# Requirements

REQ-records-006 is modified (see `deltas/records.md`): a host label equal to the local
user name, case-insensitive, does not count as a host-name leak as a path component,
while the same name still counts in a URL, as `<user>@<host>` and as the host of an
scp-style remote path. No other requirement changes; the user-name rules are untouched.
Trace: LIFE-26.
