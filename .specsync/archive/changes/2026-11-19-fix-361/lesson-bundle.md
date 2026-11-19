# Lesson bundle: fix-361

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 361
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/skill.rs, src/doctor.rs, src/main.rs, tests/package.rs, docs/cli.md, specs/cli/requirements.md, specs/cli/cli.spec.md, .chough/cases/cli.skill-outdated-nudge.cases.toml, changelog.d/2026-11-18-fix-361.md
- **Acceptance**: While an installed skill file is byte-identical to a known earlier bundle, chough doctor reports it as skill_outdated, naming the file and chough skill update, not a decision, and chough next shows the line 'Skill files are outdated: run chough skill update' and skill_outdated as the count; a current, missing or edited file gives neither, and chough skill update clears both. chough status carries no_native_change once the session recorded it.

## Evidence

- Verification commit: `5c977807d92b638d515f8a896c5231f60a017bf2`
- Base commit: `66b82f22a2eb775ece9ab823b4f2698137474a68`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

GitHub #361, a follow-up from the tester's check of #359 (#351). After a project
upgrades the chough binary, nothing tells it that its installed skill files are
an earlier release's: `doctor`, `next` and `status` are silent, and the update
advice appears only after a migrate. So a skill improvement does not reach a
project unless someone remembers `chough skill update`.

Leif's decision, in #361 on 2026-11-18: "doctor and next". `chough doctor`
reports each known-outdated skill file with a finding that names
`chough skill update`, and does not report a file the user edited. `chough next`
shows one line while any are known-outdated. Also from the same check:
`chough status` JSON carries `no_native_change`.

Doctor does not run the update: it never writes the skill, and `chough skill
update` already keeps edited files. The check reads the installed copy's files,
which `chough skill status` already does.

## From the change's design.md

# Design

Nothing visual changes. `chough next` gains one line below the objective while any
installed skill file is an earlier release's:

```text
Skill files are outdated: run `chough skill update`
```

`chough doctor` lists each such file as a `skill_outdated` finding.

## From the change's testing.md

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

`a_symlinked_host_folder_hides_no_outdated_file` (added after the tester's finding on
#362): with the shared `.agents` copy outdated and `.codex/skills` a symlink to it,
`next` still counts 1, doctor still reports the file, and doctor names the symlinked
folder as the advisory finding `skill_uninspected`.

It fails on main, where doctor reports nothing. No new free-text input is added.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-016 | the test above and the case in `.chough/cases/cli.skill-outdated-nudge.cases.toml` |

## Where these lessons go

- `specs/cli/cli.3md`
