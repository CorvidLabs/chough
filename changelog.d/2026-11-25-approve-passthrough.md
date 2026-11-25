- `chough change approve <id> --actor <name> --words "<their words>" --where
  "<citation>" [--recorded-by <agent>] [--note <extra>]` records a relayed human
  approval of a change definition through SpecSync, with the relay's fields —
  `relayed=pi chat`, `decided-by`, `words`, `where` and `recorded-by`, defaulting
  to the running Chough version — in SpecSync's note, and passes SpecSync's output,
  refusals and exit code through unchanged (REQ-cli-071, DRIVE-20). Without the
  approver, their words or the citation, or with one of them blank, it refuses
  before SpecSync runs, so a relay without a citation has no path through Chough.
