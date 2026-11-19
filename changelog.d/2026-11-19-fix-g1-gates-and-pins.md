- A session file edited by hand to hold a blank, placeholder or doubled
  `no_native_change` is refused by every reader and save, naming the field and the
  edit, so it can no longer allow the ungated move (#401).
- `chough change ship` checks its whole history, however many commits; in a shallow
  clone without the base commit it refuses, naming `git fetch --unshallow`, and where a
  squash merge removed the base it says so and goes on (#400); its gate close and move
  into done act on the session `--session` or `CHOUGH_SESSION` names (#369).
- `chough inbox list` lists the pinned session (#393).
- `chough session use` and `session new` say on standard error when a pin still names
  another session, and `chough session shelve` takes an alias or bare slug (#372).
