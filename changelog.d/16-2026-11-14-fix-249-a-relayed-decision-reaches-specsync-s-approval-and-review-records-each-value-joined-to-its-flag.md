- A relayed gate close's native approval or review, including the review `chough change
  ship` runs, passes the relay to SpecSync 6.3.0 as `--words`, `--where` and
  `--recorded-by`, so SpecSync's approval and review records hold the relayed decision
  (#249, REQ-records-008). Each value is passed joined to its flag (`--words=<words>`), so
  relay text that starts with `-`, which Chough's own flags accept, is not read as a flag.
  SpecSync 6.3.0 then records such a change at workflow version 3, which SpecSync
  6.2.x refuses, so every checkout and CI job of a project that uses it needs 6.3.0.
