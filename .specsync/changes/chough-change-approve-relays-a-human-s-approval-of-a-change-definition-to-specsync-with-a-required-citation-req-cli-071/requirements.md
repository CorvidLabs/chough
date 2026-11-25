---
change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
artifact: requirements
---

# Requirements

REQ-cli-071 (added): `chough change approve <id>` records a relayed human approval
of the change definition by running `specsync change approve <id> --actor <actor>
--note <note>`, where the note carries the relay's five fields, and passes
SpecSync's standard output, standard error and exit code through unchanged. Without
the approver, their exact words or the citation, or with a blank one of them, it
refuses before SpecSync runs; without `--recorded-by` it names the running Chough
version. The full text is in `deltas/cli.md`.
