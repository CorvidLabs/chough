---
change: align-native-actor-examples-in-user-documentation
artifact: docs
---

# Documentation

In the CLI recovery table, assign native approval recording to the current agent,
use its account in --actor, and carry the actual human source, covered scope and
limits in the note. Apply existing authority when it covers the change and ask
only for missing or exceeded authority. Preserve all native refusal/check rules.

In the walkthrough, make native commands agent instructions and show the same
actor/source roles without asking the operator to dispatch an approval command.
Keep actual implementation review as a separate human decision.

In the deferred NEXT_CHANGE proposal, update only the conditional native approval
example and its authority explanation. Retain the historical unapproved status,
proposed scope and observation limits; no historical approval or product result is
created by changing the example.

The primary-scope consistency test now scans these three files as well as README,
the canonical bundle and the installed copies. Its pre-fix failure is retained
with baseline head 9c41e95949adcb1009c986f97db50faa593d81b2, source diff and hashes.
After the documentation edits, rerun the two workflow tests and formatting. Root
owns Chough observations/bugs, full Fledge and MSRV qualification, all committed
native verification and pushing the existing PR. No native review or ship occurs
in this documentation task.
