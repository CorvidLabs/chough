---
change: split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new
artifact: research
---

# Research

- SpecSync 6.1.2 rules as recorded for #59: a non-removed requirement in a module
  delta must start with `REQ-<module>-`; added IDs may not reuse a tombstone; a source
  file may be owned by one spec only, and every spec needs a `files:` list.
- Chough validates a case's requirement links against its own module contract's
  companion only (src/cases.rs check_links, src/evidence.rs validate_requirements),
  so a case cannot link REQ-cli and REQ-reporting IDs at once.
- Case co-linking at 70009d6: REQ-cli-028 shares no case; REQ-cli-020 is linked by
  12 cases and REQ-cli-024 by 2 (both also link 020); 6 of them also link REQ-cli-003,
  004, 006, 007 or 008.
- Sizes at 70009d6: specs/cli/requirements.md 65,698 bytes; the three moved blocks
  1,740 + 3,789 + 2,075 = 7,604 bytes.
- Only #69 among open PRs edits src/timing.rs; none edits tests/timing.rs.
