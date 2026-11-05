# Lesson bundle: read-the-last-trace-list-of-a-requirement-line-and-accept-quoted-and-block-hi-family-declarations-in-vault-notes

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Read the last Trace list of a requirement line and accept quoted and block HI family declarations in vault notes (reviews of 107)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/vault.rs, tests/vault.rs
- **Acceptance**: A requirement that mentions Trace: inline before its real Trace list is credited from the real list, so this repository REQ-cli-044 credits KNOW-14 and KNOW-15; HI files declaring families quoted or as a YAML block list link to their intent notes; the new test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `804e5cbec9033ad15d7cb3eb2904eaaa9e3c4a3d`
- Base commit: `94ebefc04e6910f066800cc34698a6c2b3a101bd`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Two more reading bugs were found in #107's vault at 94ebefc:
- corvid-agent (review 5779563953): `vault::traces` used the first `Trace:` on a line.
  REQ-cli-044 mentions `Trace:` inline before its real list, so this repository's vault
  credited no criteria to the vault change itself (KNOW-14 and KNOW-15 showed "none yet").
- Leif's review agent (0xLeif): HI 0.8.0 accepts `families: ["UX"]` and a YAML block
  list (`families:` then `  - UX`). The vault read the quotes into the family name, and
  read the block list as empty, so archive notes lost their intent links.

REQ-cli-044 already requires both behaviours, so no requirement text changes.

## From the change's testing.md

# Testing

tests/vault.rs `trace_lists_and_family_declarations_are_read_in_every_valid_form` adds a
requirement that mentions `Trace:` inline before `Trace: UX-1, SEND-2.`, plus
`hi/ux.md` declaring `families: ["UX"]` and `hi/send.md` declaring a block list. The
archive note must credit both IDs and link both intent notes, and each intent note must
give its family and link the archive. The test fails first. `fledge lanes run verify`
runs on Linux, and this repository's vault then credits KNOW-14 and KNOW-15.

## Where these lessons go

- `specs/cli/context.md`
