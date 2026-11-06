---
change: rebuild-the-linux-test-environment-from-the-repository-with-one-command-using-the-project-s-pinned-tool-versions-177
artifact: design
---

# Design

## Approach

- **`tools/linux/pins.sh`** reads `.github/workflows/ci.yml`, or `$CHOUGH_CI_FILE`, and prints `specsync=`, `fledge=` and `human-intent=` lines:
  - specsync and fledge from the `version:` under each `uses: CorvidLabs/spec-sync@` and `uses: CorvidLabs/fledge@` step;
  - human-intent from `cargo install human-intent --version`.

  It exits non-zero naming the tool when a pin is missing or the specsync pins disagree.
- **`tools/linux/Dockerfile`** has three stages:
  - `toolchain`, from `rust:1.95-bookworm` with git, jq, python3, procps, the three tools via `cargo install --locked` from `ARG`s, rustfmt and clippy;
  - `dev`, which adds the uid-501 user with git config;
  - `runner`, which adds rsync.
- **`tools/linux/image.sh`** reads the pins and builds `--target toolchain -t chough-linux:1`, `--target dev -t chough-linux:2` and `-t chough-linux:3`, with `--build-arg` for each version.
- **`tools/linux/check-versions.sh`** compares `specsync --version`, `fledge --version` and `hi --version` with the pins, and exits 3 naming the tool and `fledge run linux-image`.
- **`tools/linux/verify.sh`**, on the host:
  - bundles the current branch;
  - runs `chough-linux:3`, which clones the bundle, runs `check-versions.sh` and then `fledge lanes run verify`;
  - returns the lane's exit code.

  It refuses before anything else when the image is missing, naming `fledge run linux-image`.

## Risks

- **Two kinds of pin.** The Rust toolchain stays `rust:1.95-bookworm`, as today: CI uses stable plus a 1.88 MSRV job, so there is no single Rust pin to read. Only the tool pins come from CI.
- **Workflow layout.** If the CI workflow's layout changes, the pin reader refuses rather than guessing, and its test fails.
- **`verify.sh` needs Docker,** so its run is shown on Linux, not in `cargo test`. Its version gate is the tested `check-versions.sh`.

## Constraints consulted

DRIVE-22, FLOW-17, REQ-cli-058.
