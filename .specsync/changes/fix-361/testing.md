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

- with no session open, `next` still gives `skill_outdated` in its JSON and shows the
  line under `Session: none open` (added on Leif's review of #362).

`a_fifo_at_an_installed_skill_file_blocks_neither_next_nor_doctor` (added after the
swarm's finding on #362): with a FIFO at an installed skill file, `next`, `doctor`
and `skill status` return within 20 seconds and report no outdated file. It fails
without the non-blocking open, where `next` blocks.

`an_uninspectable_skill_path_hides_no_other_outdated_file` (added after the
reviewer's finding on #362): with one skill file an earlier release's copy and another
replaced by a symlink, `next` still counts 1 and doctor still reports the file. It
fails with the earlier code, where one refused path made the whole scan report 0.

It fails on main, where doctor reports nothing. No new free-text input is added.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-016 | the test above and the case in `.chough/cases/cli.skill-outdated-nudge.cases.toml` |
