---
change: case-recording-order-the-shipped-skill-keeps-running-and-recording-chough-cases-out-of-tasks-md-checkboxes-and-runs-the
artifact: requirements
---

# Requirements

One requirement is modified; none is added or removed. See `deltas/cli.md`.

- REQ-cli-009 (validated test definitions) gains one sentence and one acceptance
  criterion: the shipped skill tells agents that a requirement an approved native
  change adds is linkable only after the scoped `specsync change check` materializes
  it, to run that check before adding, running or recording cases that link it, and
  not to list running or recording cases as a `tasks.md` checkbox. Trace adds LIFE-6
  ("reuse HI, SpecSync, Fledge ... instead of maintaining duplicate lifecycle
  records") to FLOW-7 and LIFE-2.

Unchanged and relied on: REQ-cli-005 (the refusal and its check-first hint, which
SHALL NOT read the checkpoint or native deltas), REQ-cli-016 and REQ-cli-019 (host
copies, known earlier digests, no dashes). No HI criterion is added or reworded; HI
owns wording.
