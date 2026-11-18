## MODIFIED

### SPEC SECTION Invariants

1. A record file starts with front matter between two `---` lines; front matter
   outside the subset, a duplicate key or a missing closing line is refused with the
   file, the line and the rule, and nothing is written.
2. A key Chough does not know, a blank line and a comment line are kept byte for byte
   when Chough rewrites the front matter; every body byte outside the sections Chough
   appends to is kept.
3. A record is never larger than its bound: 65,536 bytes, or 1,048,576 for a session
   and an archived session story, or 4,194,304 for a migration receipt.
4. A new record's name is a UTC date and a slug of at most 80 bytes with any clash
   suffix, never a count; a clash draws one suffix, and a second clash is refused
   without another draw.
5. A refusal for a leak names the field and the kind of match, never the text.
6. A session's front matter holds `no_native_change`, the reason its work needs no
   native change, only once the session recorded it (REQ-cli-007), and never together
   with a native change; an archived story keeps it.
