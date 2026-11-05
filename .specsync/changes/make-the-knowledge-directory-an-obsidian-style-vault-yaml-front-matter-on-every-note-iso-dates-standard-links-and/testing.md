---
change: make-the-knowledge-directory-an-obsidian-style-vault-yaml-front-matter-on-every-note-iso-dates-standard-links-and
artifact: testing
---

# Testing

tests/vault.rs, with a fixture of two archives over cli and history, requirements with
`Trace:` lines, two HI files, a lesson source, AGENTS.md and docs/DECISIONS.md:
- Front matter on every note.
- Archived-change properties and links.
- Module notes.
- Intent notes, with no criterion text copied.
- The rules note and the index links.
- Every relative link resolves.
- Byte-identical regeneration. This one already held before the change and guards it.

The other six fail first. Cases bind all seven, and `fledge lanes run verify` runs on
Linux.
