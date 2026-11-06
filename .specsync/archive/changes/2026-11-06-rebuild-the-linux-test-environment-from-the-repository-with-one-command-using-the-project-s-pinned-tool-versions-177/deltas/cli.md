## ADDED

### REQUIREMENT REQ-cli-058

Linux test environment. The repository SHALL hold the recipe of the Linux images that verification runs in, under `tools/linux/`.
- `fledge run linux-image` SHALL build `chough-linux:1` (the toolchain, as root), `chough-linux:2` (adds a uid-501 `dev` user) and `chough-linux:3` (adds rsync).
- The image SHALL take its specsync, fledge and human-intent versions from the pins in `.github/workflows/ci.yml`. The Dockerfile SHALL name no tool version itself.
- The pin reader SHALL refuse, before anything is built, when a pin is missing or the workflow's specsync pins disagree, naming the tool.
- `fledge run linux-verify` SHALL run `fledge lanes run verify` on the current commit inside `chough-linux:3`. Before verifying, it SHALL refuse with exit 3 when specsync, fledge or `hi` in the image is not the pinned version, naming the tool and `fledge run linux-image`.

Trace: DRIVE-22, FLOW-17.

Acceptance Criteria
- `fledge.toml` declares `linux-image` and `linux-verify`, running `tools/linux/image.sh` and `tools/linux/verify.sh`.
- The pin reader prints exactly the specsync, fledge and human-intent versions the CI workflow pins. A workflow whose specsync pins disagree, or that lacks the human-intent pin, is refused with a message naming the tool.
- The Dockerfile contains none of the pinned versions and takes each one from a build argument.
- The version check passes with the pinned tools, and exits 3 for a stale specsync, naming it and `fledge run linux-image`.
