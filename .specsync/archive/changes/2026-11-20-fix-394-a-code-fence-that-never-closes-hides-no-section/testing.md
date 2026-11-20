---
change: fix-394-a-code-fence-that-never-closes-hides-no-section
artifact: testing
---

# Testing

Each failing on main first:

- `tests/records.rs`: a session whose Goal is three backticks, saved three times with a
  decision and a question each, keeps one of each section, and `chough next` names the
  decisions and the questions; with `~~~` the same; a closed fence quoting `## Log` still
  hides it (the existing unit test).
- `tests/bugs.rs`: two sides each adding a Log entry to a bug whose write-up holds an
  unclosed fence merge with both entries and exit 0.

Then every verify step, the cases on macOS, and the tester's run on the real
repositories, which must show: every committed session, story and bug reads with the
same sections before and after (a count per record kind, and the list of any record
that differs, each one holding a fence that never closes); a save of a copy of each
record that holds an unclosed fence adds no section; and `chough bug merge` on each
such bug keeps both sides' Log entries.
