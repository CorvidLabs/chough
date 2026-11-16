- `chough init` still adds a missing skill file without touching its neighbors, and a
  session whose files were edited since init stays byte-identical on a later init.
  Init never removes or rewrites a file the user or a previous run created
  (REQ-cli-001).
