---
change: fix-320
artifact: testing
---

# Testing

`tests/leaks.rs`:
- `a_leaking_id_reference_names_the_recovery`: a session whose committed `cases` reference holds a token refuses `checkpoint --decision`, naming `chough session shelve <id>`. Fails on main.
- `doctor_redact_never_rewrites_an_id`: the same committed session, with an email in a body line, gives two `leak` findings, and the ID's names `chough session shelve <id>`. `doctor --fix --redact` redacts the email, leaves the `cases` line byte for byte with its finding `skipped`, and the session still reads (`chough status` succeeds). Fails on main.

Case `TC-cli-doctor-redact-never-rewrites-an-id` binds the second.
