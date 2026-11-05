---
change: read-the-last-trace-list-of-a-requirement-line-and-accept-quoted-and-block-hi-family-declarations-in-vault-notes
artifact: testing
---

# Testing

tests/vault.rs `trace_lists_and_family_declarations_are_read_in_every_valid_form` adds a
requirement that mentions `Trace:` inline before `Trace: UX-1, SEND-2.`, plus
`hi/ux.md` declaring `families: ["UX"]` and `hi/send.md` declaring a block list. The
archive note must credit both IDs and link both intent notes, and each intent note must
give its family and link the archive. The test fails first. `fledge lanes run verify`
runs on Linux, and this repository's vault then credits KNOW-14 and KNOW-15.
