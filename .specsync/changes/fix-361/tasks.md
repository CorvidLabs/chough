---
change: fix-361
artifact: tasks
---

# Tasks

- [x] Add a test that fails on main: doctor and next say when skill files are known-outdated.
- [x] `chough doctor` reports each known-outdated skill file as `skill_outdated`, naming `chough skill update`.
- [x] `chough next` shows one line and gives the count as `skill_outdated`.
- [x] `chough status` carries `no_native_change`.
- [x] Document the finding in `docs/cli.md`; add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
