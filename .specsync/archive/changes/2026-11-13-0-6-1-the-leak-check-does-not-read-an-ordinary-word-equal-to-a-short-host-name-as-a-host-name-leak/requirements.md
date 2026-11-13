---
change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
artifact: requirements
---

# Requirements

REQ-records-006 (modified): the local host name is a leak anywhere when it is at least
8 characters long or holds a digit, a hyphen or a dot; a shorter all-letter host name,
which may be an ordinary word, is a leak only inside a path or a URL or as
`<user>@<host>`. The local user name keeps its rule.
