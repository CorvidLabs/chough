---
change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
artifact: design
---

# Design

No product screen changes, so there is no HTML prototype. The visible result is
the command line: `chough change approve` prints nothing of its own on success
but SpecSync's output, and its refusals are plain messages that name the missing
relay field and the command to run again.
