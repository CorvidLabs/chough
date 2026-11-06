---
change: rebuild-the-linux-test-environment-from-the-repository-with-one-command-using-the-project-s-pinned-tool-versions-177
artifact: context
---

# Context

Issue #177. Agents run the Linux verify in local Docker images `chough-linux:1` to `:3`, and nothing in the repository described them. On 2026-11-06 a Docker Desktop reset deleted every image and volume. Two agents then rebuilt the same tags at once, from Dockerfiles kept in one agent's scratch directory. The rebuilt images pinned specsync 6.1.2, while the project pins 6.2.0. Leif captured DRIVE-22 ("Capture as written").

ch-c456 asked for its `linux-verify.sh` to become `fledge run linux-verify` here. That script clones the branch inside `chough-linux:3`, refuses unless specsync, hi and fledge are the pinned versions, and runs the verify lane. REQ-cli-058 was agreed with ch-c456, whose #174 uses REQ-cli-057.
