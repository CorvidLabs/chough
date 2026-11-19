---
change: readiness-report-before-change-approval
artifact: context
---

# Context

On 2026-11-14, local-model agents wrote correct code fixes and then skipped the
session, the change, the delta or the commit. The rules they missed are ones
SpecSync states only when a later command refuses: a delta for every affected
spec, a delta still written against the canonical text it was approved on, every
export in a declared file documented in the spec, a case that links only its own
module's requirements, scope frozen at approval, and an interview or artifact
left unfinished. Change ids are long and must be typed exactly, so a second
attempt often repeats the same miss.

DRIVE-29, captured from Leif's choice "Capture as written" in chat on
2026-11-14, asks for one read-only command before approval. `chough change ready
<id>` prints every miss in one report and names the exact command that fixes
each one. It does not approve, check or rewrite anything.

The command is new. `chough change new`, `answer`, `check`, `status` and `ship`
stay SpecSync passthroughs (REQ-cli-057, REQ-cli-059). The cli spec's `files:`
list gains `src/ready.rs` and `tests/ready.rs`, which do not exist until the
build. README's helper table and `docs/cli.md`'s command table must name the
new subcommand, because those tables are checked against `chough --help`.
