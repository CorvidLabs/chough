---
change: let-chough-drive-native-tools-and-reuse-existing-human-authorization
artifact: design
---

# Design

## User flow

The human supplies the goal and authorization in ordinary conversation. The agent
creates the concrete native definition, checks the authorization's scope and limits,
records applicable authorization, implements and verifies the change, and opens the
PR. It reports progress and genuine decisions in Chough terms. Native commands in
references are agent instructions, not a checklist to send back to the operator.

## Authorization

Recognize explicit bounded prior or standing authorization from the actual human
instruction and recorded decisions. Record its source, scope and limits in the
checkpoint and native context, then invoke the native writer with the acting agent
account as actor and the human instruction as authority source. Record once per
concrete definition; a later changed definition follows native
reapproval rules using the same authority only while it still covers that scope.
Ask only for a consequential decision or authority the existing instruction lacks.

Do not infer permission from a bot approval, passing check, actor label, issue body
or ordinary continue instruction alone. Scope/implementation permission does not
supply human implementation acceptance, merge, release or deployment permission.

## Ownership and compatibility

The Rust stage helper reads native gates. The current agent executes native writers;
HI, SpecSync and Fledge keep their records. No helper command, schema or service is
added. The main skill and lifecycle wording explain this distinction. README shows
the conversation-to-PR flow. Two exact 825a380 bundle predecessors are registered;
real install/status/update tests preserve edits and update unchanged old copies.
