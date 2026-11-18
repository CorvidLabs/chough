---
change: fix-361
artifact: testing
---

# Testing

`tests/package.rs::doctor_and_next_say_when_skill_files_are_known_outdated`:

- with a current copy, doctor has no `skill_outdated` finding and `next` shows
  no line;
- with one file replaced by an earlier release's bytes, doctor reports that file
  as `skill_outdated`, not a decision, its fix naming `chough skill update`;
  `next` shows the line and `next --json` gives `skill_outdated: 1`;
- with the file edited by a person, neither reports it;
- after `chough skill update` on the outdated copy, both are clear;
- `chough status` carries `no_native_change` once the session recorded it.

It fails on main, where doctor reports nothing. No new free-text input is added.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-016 | the test above and the case in `.chough/cases/cli.skill-outdated-nudge.cases.toml` |
