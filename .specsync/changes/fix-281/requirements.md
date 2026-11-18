---
change: fix-281
artifact: requirements
---

# Requirements

- REQ-cli-007 (`deltas/cli.md`): gains the `--no-native-change <reason>` flag, its two
  refusals, that recording a native change removes it, the
  `ungated:no-native-change-required` label for a move that would otherwise refuse as
  `refused:no-native-change`, and that an open human gate still refuses the move.
- The records spec's Invariants (`deltas/records.md`): one new invariant for the
  `no_native_change` front matter key.
