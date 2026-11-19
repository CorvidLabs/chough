---
change: fix-281
artifact: testing
---

# Testing

`tests/stages.rs::a_session_that_needs_no_native_change_reaches_done_and_archives_as_done`,
with SpecSync configured and no native change recorded:

- a stage advance refuses as `refused:no-native-change`;
- a blank reason is refused;
- after `chough checkpoint --no-native-change "<reason>"` the session holds the
  reason, in its JSON and in its front matter;
- an open human gate still refuses the advance as `refused:awaiting-human`;
- with the gate withdrawn, four advances, each labelled
  `ungated:no-native-change-required`, reach done; the session is archived with
  `outcome: done` and its story keeps the reason;
- recording a native change removes the reason, and the flag is then refused,
  naming the change.

- a reason holding a credential or an email address is refused as `refused:leak`
  with the session unchanged, and `--redact` stores it redacted (added after the
  tester's finding on #348; it fails without the leak guard).

It fails on main, which has no such flag.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-007 | the test above and the case in `.chough/cases/cli.no-native-change-required.cases.toml` |
