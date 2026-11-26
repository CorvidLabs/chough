---
change: release-prep-0-6-4
artifact: context
---

# Context

Three structurally important changes are unreleased on main since 0.6.3 (2026-11-22):
the six overnight record/gate fixes (#448), `chough change approve` relay passthrough
(#450, REQ-cli-071), and chough-first skill routing (#451, REQ-cli-072). CI is green at
merge commit 2577da8e; tests verified on a fresh clone 2026-11-26 (14/14 unit + 24/24
gates). Per LIFE-25 a weekly 0.x release ships from main when CI is green; the agent
prepares changelog + version bump + green checks, Leif gives a one-line go ("Yes let's
do that", pi chat t14z 2026-11-26), then it publishes. The tag/publish/GitHub-release/
install steps in docs/RELEASING.md are the human's and follow after this merges.
