---
change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
artifact: context
---

# Context

A 2026-11-25 standing rule says an agent records a human's approval of a SpecSync
change through Chough, with who decided, the exact words, a citation of where they
were said and the recording agent; a relay without a citation is exactly what the
rule exists to prevent, and a record whose citation does not resolve is void. Gate
closings already relay approvals and reviews through Chough (`checkpoint
--close-gate`, `gate record`, `change ship`), but a change definition approval
relayed from a chat had no Chough path: the agent either called `specsync change
approve` directly or left the approval unrecorded.

The user approved building the passthrough (quest q-14-1, on council counsel from
Astra and Fable) before any direct approve call. It extends REQ-cli-057's pattern:
`chough change` runs a SpecSync step and passes its output, refusals and exit code
through unchanged.

Ruled out: reusing the gate relay, which records a decision at a gate rather than a
change definition approval; letting agents keep calling `specsync change approve`
directly.
