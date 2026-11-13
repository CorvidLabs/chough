---
change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
artifact: testing
---

# Testing

A synthetic short host name through `Local::new`, never this machine's: a record using
it as a word is written and `chough doctor` finds nothing; the same name inside a path,
a URL or as `<user>@<host>` is refused; a long or hyphenated host name is refused
anywhere. Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-006 | `tests/leaks.rs`: a short all-letter host name is a leak only inside a path, a URL or `<user>@<host>` |
