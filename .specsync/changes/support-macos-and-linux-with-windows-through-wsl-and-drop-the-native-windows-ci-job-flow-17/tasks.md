---
change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
artifact: tasks
---

# Tasks

- [ ] Add the failing test `supported_platforms_are_macos_and_linux_with_windows_through_wsl` in tests/package.rs and see it fail on the base.
- [ ] Remove `windows-latest` from the product matrix in `.github/workflows/ci.yml`.
- [ ] Update README.md, docs/ci.md, docs/integrations.md, docs/cli.md, docs/ROADMAP.md, docs/DECISIONS.md and CHANGELOG.md.
- [ ] Update specs/cli/testing.md so it no longer claims Windows CI.
- [ ] Run `fledge lanes run verify` locally.

Not tasks (human gates and Chough records): orc's scope approval relay, recording and running case `TC-cli-supported-platforms`, the human implementation review, and merge on orc's relay.
