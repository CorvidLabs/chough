---
change: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
artifact: testing
---

# Testing

Product tests (fail first on main):
- tests/relay.rs:
  - closing one of two gates records the relay, and the other gate still holds advance;
  - a fresh `next` shows the decision;
  - each refusal leaves the checkpoint and history unchanged, and its named recovery
    then succeeds;
  - with a draft native change, SpecSync files are byte-identical and the build gate
    still refuses;
  - `--clear-awaiting-human` writes no relay;
  - closing the only gate lets advance apply;
  - the legacy log and history events round-trip the relay;
  - invalid payloads are refused.
- tests/stages.rs: the refusal and waiting action name `--close-gate`. The existing
  assertions stay, and the expected text gains the new command.

Cases: `.chough/cases/cli.relayed-decisions.cases.toml` binds the main tests, run locally
and on Linux (Docker, local-linux-aarch64).

Gates: `fledge lanes run verify` (fmt, clippy -D warnings, tests, hi, specsync strict,
audit, cases, knowledge, exercise, build). Hosted CI is observed only after finalize.
