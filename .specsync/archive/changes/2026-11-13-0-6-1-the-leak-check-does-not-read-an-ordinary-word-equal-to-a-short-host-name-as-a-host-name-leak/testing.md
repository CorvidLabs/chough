---
change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
artifact: testing
---

# Testing

A unit test in `src/leaks.rs` with a synthetic short host name through `Local::new`, never this machine's: a record using
it as a word is written and `chough doctor` finds nothing; the same name inside a path,
a URL or as `<user>@<host>` is refused; a long or hyphenated host name is refused
anywhere. Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-006 | `src/leaks.rs` (`leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host`): a short all-letter host name is a leak only inside a path, a URL or `<user>@<host>` |
