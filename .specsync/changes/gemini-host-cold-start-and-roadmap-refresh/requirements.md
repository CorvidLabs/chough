---
change: gemini-host-cold-start-and-roadmap-refresh
artifact: requirements
---

# Requirements

This change delivers guidance, cold-start evidence, and roadmap currency without altering canonical contract behavior. Canonical requirements in specs/cli/ remain authoritative and unchanged:

- REQ-cli-007 (Canonical checkpoint and explicit migration): cold resumption follows the single checkpoint, checks native SpecSync authority, and identifies next valid actions when stage is done.
- REQ-cli-016 (Host skill installation): skill bundle copies remain current across supported host locations (.agents, .claude, and .gemini), and updates preserve user edits while replacing only known earlier bundle files.
- REQ-cli-019 (Installable package): the shipped bundle contains no em or en dash, and previous bundle digests are tracked in KNOWN_PREVIOUS.

Trace: FLOW-1, FLOW-13, LIFE-4, LIFE-9, LIFE-10. These durable criteria exist in hi/ and are linked without modification.
