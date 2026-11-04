---
change: warn-when-the-native-check-runs-none-of-the-project-commands-judged-from-the-effective-commands-specsync-reports-and
artifact: requirements
---

# Requirements

ADDED REQ-cli-038 (see `deltas/cli.md`): `next` and `status` warn when the effective
commands SpecSync reports for the native check are only SpecSync's own, naming
configured commands it does not run; at test, `next` points to recorded test evidence.
No gate changes and no native read is added. Trace: FLOW-15, FLOW-6, LIFE-7.
