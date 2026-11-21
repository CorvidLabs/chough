---
change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
artifact: context
---

# Context

corvid-agent reviewed pull request 443 at `727d3bfd` and requested one code change. The label script removes every `chough-stage:*` label, including the one it is about to apply, then adds that label again. REQ-cli-070 already says to apply the one main stage and remove every other stage label.

The fork guard, the symlink skip, and the label-definition refresh are already archived. This change does not reopen that archive.

Ruled out: leaving the remove-and-add in place and only filing a follow-up issue. The review asked for the script change.
