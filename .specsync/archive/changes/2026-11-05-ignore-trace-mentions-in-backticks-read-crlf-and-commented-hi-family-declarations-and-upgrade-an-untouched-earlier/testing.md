---
change: ignore-trace-mentions-in-backticks-read-crlf-and-commented-hi-family-declarations-and-upgrade-an-untouched-earlier
artifact: testing
---

# Testing

Two new tests in tests/vault.rs, both failing first:
- `inline_trace_mentions_crlf_and_comments_do_not_hide_criteria_or_families`: a
  backticked `Trace:` after the real list, a CRLF HI file, and a commented families list.
- `an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept`: the README from main
  8fbbbe7 (tests/fixtures/knowledge-readme-at-8fbbbe7.md) is upgraded by `library
  index`; an edited copy is left byte-identical by `index` and `init`.

The earlier inline-mention-before-list test still passes. `fledge lanes run verify` runs
on Linux.
