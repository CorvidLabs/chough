# Lesson bundle: rebuild-the-linux-test-environment-from-the-repository-with-one-command-using-the-project-s-pinned-tool-versions-177

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Rebuild the Linux test environment from the repository with one command, using the project's pinned tool versions (177, DRIVE-22)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: tools/linux/Dockerfile, tools/linux/pins.sh, tools/linux/image.sh, tools/linux/verify.sh, tools/linux/check-versions.sh, tools/linux/README.md, fledge.toml, tests/linux_image.rs, specs/cli/cli.spec.md, hi/drive.md, docs/cli.md, AGENTS.md, CHANGELOG.md
- **Acceptance**: DRIVE-22 (Leif, relayed by orc 2026-11-06, 'Capture as written'; chough#177), in the new REQ-cli-058. tools/linux/ holds the Dockerfile (three stages for chough-linux:1/:2/:3, with tool versions as build arguments), pins.sh (reads the specsync, fledge and human-intent pins from .github/workflows/ci.yml and refuses missing or disagreeing pins), image.sh, verify.sh and check-versions.sh. fledge run linux-image builds the images; fledge run linux-verify runs fledge lanes run verify on the current commit in chough-linux:3 and refuses with exit 3 when a tool is not pinned. The five new tests in tests/linux_image.rs need no Docker and fail first. The images are rebuilt with fledge run linux-image and linux-verify passes on Linux. Docs, AGENTS.md and CHANGELOG say so. linux-verify comes from ch-c456's linux-verify.sh at its request.

## Evidence

- Verification commit: `a530d96674a853372c42ec2daa69184adfdc6436`
- Base commit: `dd5bbb764169d8e38f1de65a8cb5a5b70eb6483d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #177. Agents run the Linux verify in local Docker images `chough-linux:1` to `:3`, and nothing in the repository described them. On 2026-11-06 a Docker Desktop reset deleted every image and volume. Two agents then rebuilt the same tags at once, from Dockerfiles kept in one agent's scratch directory. The rebuilt images pinned specsync 6.1.2, while the project pins 6.2.0. Leif captured DRIVE-22 ("Capture as written").

ch-c456 asked for its `linux-verify.sh` to become `fledge run linux-verify` here. That script clones the branch inside `chough-linux:3`, refuses unless specsync, hi and fledge are the pinned versions, and runs the verify lane. REQ-cli-058 was agreed with ch-c456, whose #174 uses REQ-cli-057.

## From the change's design.md

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

## From the change's testing.md

# Testing

New tests in tests/linux_image.rs. None needs Docker.
- `fledge_declares_the_linux_image_and_verify_tasks`.
- `the_image_pins_are_the_ci_pins`: `pins.sh` prints the versions the test reads from ci.yml itself.
- `missing_or_disagreeing_pins_refuse`: a copy of ci.yml with one specsync pin changed, and one without the human-intent install, are both refused, naming the tool.
- `the_dockerfile_takes_every_tool_version_from_a_build_argument`: no pinned version appears in the Dockerfile, each is an `ARG` used as `${...}`, and `image.sh` builds all three tags.
- `the_version_check_refuses_a_tool_that_is_not_pinned`: with stub tools on PATH, the pinned versions pass, and a stale specsync exits 3 naming specsync and `fledge run linux-image`.

All five fail first on Linux, because the files are missing. A case binds them. The Docker parts are shown by running `fledge run linux-image` and `fledge run linux-verify` on the branch.

## Where these lessons go

- `specs/cli/context.md`
