---
change: trim-old-proof-automatically-when-a-change-reaches-done-with-the-librarian-on-and-let-retention-manual-opt-out-issue-88
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/88 (field test 2026-11-03): with the
librarian on, nothing trims after archive; `chough library archive` only previews and
`--apply` needs `chough consent grant retention` first. Leif prefers trimming to be
automatic by default with an opt-out.

Leif chose on 2026-11-04 ("Auto-trim at done"), captured as KNOW-13: with the librarian
on (the default), `stage advance` into done, after the archive, trims cold archived
proof, keeping the ledger-first and KNOW-10 restore checks, and `chough pref set
retention manual` opts out; the librarian default counts as the standing preference
KNOW-11 names. A preview with a one-step apply, and an opt-in standing preference, were
offered and not chosen. hi keeps KNOW-11's wording (criteria are added, not rewritten).

Trimming reads native status for the cold archives whose files it would remove (KNOW-10),
which can take time on projects with many archives; it runs only on the move into done,
never inside `next` or `status`.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes, his brief to
fix issues 81 to 88 one PR each, and his #88 choice.
