---
change: fix-361
artifact: requirements
---

# Requirements

REQ-cli-016 is modified, in `deltas/cli.md`: one sentence saying that, while an
installed skill file is byte-identical to a known earlier bundle, `chough doctor`
reports it as `skill_outdated` naming `chough skill update`, with no fix of its own,
and `chough next` shows one line and the count; neither reports a current, missing or
edited file. `chough status` carrying `no_native_change` needs no requirement text.
