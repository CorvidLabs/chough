---
change: rebuild-the-linux-test-environment-from-the-repository-with-one-command-using-the-project-s-pinned-tool-versions-177
artifact: plan
---

# Plan

## Approach

Commit the image recipe under `tools/linux/` and take every tool version from the CI workflow's pins (REQ-cli-058, DRIVE-22). Add `fledge run linux-image` to build the images and `fledge run linux-verify` to run the verify lane in them, refusing when a tool is not pinned.

## Out of scope

- CI itself, which keeps its own setup and is unchanged.
- The agents' `lx` runner, which stays in their scratch directories.
- Publishing images to a registry.
- Windows (FLOW-17).

## Steps

1. Capture DRIVE-22. Write five failing tests in `tests/linux_image.rs`, which need no Docker, and run them on Linux.
2. Get scope approval through the orc.
3. Add `tools/linux/`: `Dockerfile`, `pins.sh`, `image.sh`, `check-versions.sh`, `verify.sh` and a README. Add the two fledge tasks.
4. Add the test file to `cli.spec.md`. Update `docs/cli.md`, `AGENTS.md` (how to rebuild after a Docker reset) and `CHANGELOG.md`.
5. Run `fledge run linux-image`, then `fledge run linux-verify`, plus the verify lane on Linux and the native check last. Open the pull request.

## Risks

See design.md.

## Constraints consulted

DRIVE-22, FLOW-17, REQ-cli-058, `.github/workflows/ci.yml`.
