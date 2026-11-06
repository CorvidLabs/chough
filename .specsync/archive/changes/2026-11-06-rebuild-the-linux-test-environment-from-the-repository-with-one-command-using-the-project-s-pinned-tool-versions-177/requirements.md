---
change: rebuild-the-linux-test-environment-from-the-repository-with-one-command-using-the-project-s-pinned-tool-versions-177
artifact: requirements
---

# Requirements

REQ-cli-058 (ADDED, in `deltas/cli.md`): the Linux test environment.
- The recipe lives under `tools/linux/`.
- `fledge run linux-image` builds the three tags, with tool versions taken from the CI workflow's pins.
- The pin reader refuses when a pin is missing or the specsync pins disagree.
- `fledge run linux-verify` runs the verify lane in `chough-linux:3`, and refuses with exit 3 when a tool is not pinned.

No existing requirement text changes.
