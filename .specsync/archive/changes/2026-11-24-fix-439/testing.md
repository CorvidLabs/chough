# Testing — fix-439

Planned coverage (to be recorded after verification):

1. Unit/integration test: a change directory with
   `verification-attempts.json` whose latest attempt has
   `"passed": false` and `"refusal": "..."` — `next`/`status` output
   contains the refusal text and no longer contains "SpecSync keeps no
   reason".
2. Test: no attempts file (or latest attempt passed) — output keeps the
   existing fallback note.
3. Full suite: `cargo test --locked` green (all suites).

Verification environment: local-macos-arm64.