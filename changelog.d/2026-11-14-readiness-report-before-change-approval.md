- `chough change ready <id>` lists everything still missing before a change is
  approved and exits 1 when anything is missing. Each finding names the command
  that fixes it, and the command changes no tracked file and no SpecSync hash
  cache (REQ-cli-067, DRIVE-29).
