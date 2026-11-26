---
change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
artifact: docs
---

# Docs

`docs/cli.md` lists `approve` in the `change` row of the Commands table and adds
its row to the SpecSync change steps table, with the flags, the note's fields in
order, the refusals and the passthrough.

`README.md` names `approve` in the helper table's `change` row and in the
paragraph that says which `chough change` steps pass SpecSync's output through.

`changelog.d/2026-11-25-approve-passthrough.md` records the change for the next
release. No HI file changes: DRIVE-20 already owns the criterion.
