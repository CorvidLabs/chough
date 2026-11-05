---
change: read-the-last-trace-list-of-a-requirement-line-and-accept-quoted-and-block-hi-family-declarations-in-vault-notes
artifact: context
---

# Context

Two more reading bugs were found in #107's vault at 94ebefc:
- corvid-agent (review 5779563953): `vault::traces` used the first `Trace:` on a line.
  REQ-cli-044 mentions `Trace:` inline before its real list, so this repository's vault
  credited no criteria to the vault change itself (KNOW-14 and KNOW-15 showed "none yet").
- Leif's review agent (0xLeif): HI 0.8.0 accepts `families: ["UX"]` and a YAML block
  list (`families:` then `  - UX`). The vault read the quotes into the family name, and
  read the block list as empty, so archive notes lost their intent links.

REQ-cli-044 already requires both behaviours, so no requirement text changes.
