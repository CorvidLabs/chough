# Plan — fix-439

## Approach
In `src/native_check.rs`, before emitting the "SpecSync keeps no
reason" note, read the change's `verification-attempts.json`, take the
latest attempt, and if it has `"passed": false` with a string
`"refusal"`, build the note from that refusal text plus the existing
`run specsync change check <id>` pointer. Keep every other arm and the
fix-437 state machine untouched. Reader is pure (read-only, no writes
to the attempts file).

## Out of scope
- Writing attempts or changing their schema (fix-437 territory).
- Any other note in `next`/`status`.
- `.gitattributes`, `specs/**` contents.

## Steps
1. Add a small read-only helper to load the latest refusal from
   `verification-attempts.json` (tolerant of missing/invalid JSON).
2. Use it in the `(Some("approved" | "implementing"), _)` note branch.
3. Update the refused-check row in `docs/cli.md`.
4. Add tests: refusal surfaced; fallback note when no refusal recorded.

## Risks
- A malformed attempts file must never panic the CLI — the helper
  returns `None` on any parse error.
- The note is user-facing text; keep the existing wording for the
  fallback so no other test pins break.

## Constraints consulted
- Read-only reader; `change check` remains the writer.
- `cargo test --locked`; docs tests pin `.gitattributes` exactly.