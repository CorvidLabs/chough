# Lesson bundle: list-hi-sub-criterion-ids-in-vault-notes-and-read-trace-lists-whole-and-correct-the-know-14-property-wording-corvid

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: List HI sub-criterion IDs in vault notes and read Trace lists whole, and correct the KNOW-14 property wording (corvid-agent review of 107)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/vault.rs, src/records.rs, tests/vault.rs, hi/knowledge.md
- **Acceptance**: Vault intent notes list HI sub-criterion IDs such as UX-1.a, a Trace list such as UX-1.a, UX-2. credits both IDs and never UX-1, the archive note front matter lists both, and KNOW-14 says every note has at least id, title, type and tags while an archived-change note also gives status, dates, modules, criteria, requirements and commits. The new test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `22178540f6497c0491d38df8430c4743dd65aed2`
- Base commit: `16dc3294fc8e91a3f6a116914412fe8714a28ddd`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

corvid-agent's review of #107 (5779445909) found two bugs in the vault code:
- `vault::is_criterion` accepted only digits and dots after the family, so HI
  sub-criterion IDs such as `UX-1.a` and `CHAT-15.a`, which Chough accepts everywhere
  else (DRIVE-1), never reached `intent/<file>.md`.
- `vault::traces` cut a `Trace:` list at its first `.`, so `Trace: UX-1.a, UX-2.`
  credited `UX-1` and dropped `UX-2`.

REQ-cli-044 already requires every criterion ID to be listed, so no requirement text
changes. The review also noted that KNOW-14 listed status, dates and commits for every
note, while only archived-change notes carry them. Leif chose "Fix the wording", and
KNOW-14 is corrected before #107 merges.

## From the change's testing.md

# Testing

tests/vault.rs `sub_criterion_ids_are_listed_and_traced_in_whole_trace_lists` adds
`hi/ux.md` with `UX-1`, `UX-1.a` and `UX-2`, and a requirement traced to
`UX-1.a, UX-2.`. The archive note must list both IDs, and the intent note must credit
both and never `UX-1`. It fails before the fix. `hi check` passes after the KNOW-14
wording fix, and `fledge lanes run verify` runs on Linux.

## Where these lessons go

- `specs/cli/context.md`
