---
id: rebuild-the-linux-test-environment-from-the-repository-with-one-command-using-the-project-s-pinned-tool-versions-177
state: draft
type: feature
base_commit: dd5bbb764169d8e38f1de65a8cb5a5b70eb6483d
---

# Rebuild the Linux test environment from the repository with one command, using the project's pinned tool versions (177, DRIVE-22)

## Intent

Rebuild the Linux test environment from the repository with one command, using the project's pinned tool versions (177, DRIVE-22)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- DRIVE-22 (Leif, relayed by orc 2026-11-06, 'Capture as written'; chough#177), in the new REQ-cli-058. tools/linux/ holds the Dockerfile (three stages for chough-linux:1/:2/:3, with tool versions as build arguments), pins.sh (reads the specsync, fledge and human-intent pins from .github/workflows/ci.yml and refuses missing or disagreeing pins), image.sh, verify.sh and check-versions.sh. fledge run linux-image builds the images; fledge run linux-verify runs fledge lanes run verify on the current commit in chough-linux:3 and refuses with exit 3 when a tool is not pinned. The five new tests in tests/linux_image.rs need no Docker and fail first. The images are rebuilt with fledge run linux-image and linux-verify passes on Linux. Docs, AGENTS.md and CHANGELOG say so. linux-verify comes from ch-c456's linux-verify.sh at its request.

## No-spec Rationale

Not applicable
