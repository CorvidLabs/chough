---
change: clarify-req-cli-001-init-preserves-bytes-acceptance
artifact: context
---

# Context

REQ-cli-001 ("Preserve user files") already states that init must not replace
existing files, and its acceptance line covers repeated init and adding missing
bundle files. The walk-through of `chough change ready` (DRIVE-29) showed the
readiness command reports a missing delta whenever a change touches a spec, so
this documentation change needs its own `deltas/cli.md` before approval. No code
or behavior changes; only the acceptance line is clarified to name both
preservation behaviors explicitly.
