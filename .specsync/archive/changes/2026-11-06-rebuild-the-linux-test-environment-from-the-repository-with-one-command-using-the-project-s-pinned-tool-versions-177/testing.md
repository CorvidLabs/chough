---
change: rebuild-the-linux-test-environment-from-the-repository-with-one-command-using-the-project-s-pinned-tool-versions-177
artifact: testing
---

# Testing

New tests in tests/linux_image.rs. None needs Docker.
- `fledge_declares_the_linux_image_and_verify_tasks`.
- `the_image_pins_are_the_ci_pins`: `pins.sh` prints the versions the test reads from ci.yml itself.
- `missing_or_disagreeing_pins_refuse`: a copy of ci.yml with one specsync pin changed, and one without the human-intent install, are both refused, naming the tool.
- `the_dockerfile_takes_every_tool_version_from_a_build_argument`: no pinned version appears in the Dockerfile, each is an `ARG` used as `${...}`, and `image.sh` builds all three tags.
- `the_version_check_refuses_a_tool_that_is_not_pinned`: with stub tools on PATH, the pinned versions pass, and a stale specsync exits 3 naming specsync and `fledge run linux-image`.

All five fail first on Linux, because the files are missing. A case binds them. The Docker parts are shown by running `fledge run linux-image` and `fledge run linux-verify` on the branch.
