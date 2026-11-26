# Requirements — fix-439

## Acceptance criteria
1. When `verification-attempts.json` for a change records a latest
   attempt with `"passed": false` and a `"refusal"` string, `chough
   next` and `chough status` surface that refusal text in the readiness
   note instead of the "SpecSync keeps no reason" sentence.
2. When the latest attempt passed, or no attempts file exists, or the
   refusal field is absent or not a string, the existing notes are
   unchanged (no regression for the fix-437 states).
3. The refused-check row in `docs/cli.md` no longer says the reason is
   "not shown yet"; it names `verification-attempts.json` as the source
   the CLI reads.
4. `cargo test --locked` stays green; new tests pin (a) refusal shown
   from a recorded attempt, (b) fallback note when no refusal is
   recorded.