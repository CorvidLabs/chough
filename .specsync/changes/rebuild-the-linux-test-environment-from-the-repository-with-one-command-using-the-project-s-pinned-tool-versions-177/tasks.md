---
change: rebuild-the-linux-test-environment-from-the-repository-with-one-command-using-the-project-s-pinned-tool-versions-177
artifact: tasks
---

# Tasks

- [x] Capture DRIVE-22 in hi/drive.md.
- [x] Add the five failing tests in tests/linux_image.rs and run them on Linux.
- [ ] Add tools/linux/ (Dockerfile, pins.sh, image.sh, check-versions.sh, verify.sh, README) and the fledge tasks.
- [ ] Add the test file to cli.spec.md. Update docs/cli.md, AGENTS.md and CHANGELOG.md.
- [ ] Rebuild with fledge run linux-image, run fledge run linux-verify and the verify lane on Linux, and add the case.
