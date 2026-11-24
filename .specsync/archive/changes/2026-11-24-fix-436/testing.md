# Testing — fix-436

Planned coverage (to be recorded after verification):

1. Test: closing a gate with an alias renders the configured display
   name in the decision text.
2. Test: closing with the exact display name renders it unchanged.
3. Existing decision-text tests updated where they pinned the raw
   typed text.
4. Full suite: `cargo test --locked` green (all suites).

Verification environment: local-macos-arm64.