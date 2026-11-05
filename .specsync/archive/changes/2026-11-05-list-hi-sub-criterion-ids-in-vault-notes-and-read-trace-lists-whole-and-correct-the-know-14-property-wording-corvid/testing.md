---
change: list-hi-sub-criterion-ids-in-vault-notes-and-read-trace-lists-whole-and-correct-the-know-14-property-wording-corvid
artifact: testing
---

# Testing

tests/vault.rs `sub_criterion_ids_are_listed_and_traced_in_whole_trace_lists` adds
`hi/ux.md` with `UX-1`, `UX-1.a` and `UX-2`, and a requirement traced to
`UX-1.a, UX-2.`. The archive note must list both IDs, and the intent note must credit
both and never `UX-1`. It fails before the fix. `hi check` passes after the KNOW-14
wording fix, and `fledge lanes run verify` runs on Linux.
