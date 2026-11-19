- A session file edited by hand to hold a blank, placeholder or doubled
  `no_native_change` is refused by every reader and save, naming the field and the
  edit, so it can no longer allow the ungated move (#401).
- `chough change ship` checks its whole history, however many commits, and refuses as
  `refused:history-unchecked` when Git cannot read it (#400); its gate close and move
  into done act on the session `--session` or `CHOUGH_SESSION` names (#369).
- `chough inbox list` lists the pinned session (#393).
- `chough session use` and `session new` say on standard error when a pin still names
  another session, and `chough session shelve` takes an alias or bare slug (#372).
