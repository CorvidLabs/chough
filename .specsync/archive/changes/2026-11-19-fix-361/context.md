---
change: fix-361
artifact: context
---

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
