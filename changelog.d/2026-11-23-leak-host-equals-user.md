- `chough doctor` no longer reports a `host-name` leak for a path component equal to
  the local user name, as a source directory on a machine named after its owner; the
  same name still counts in a URL, as `<user>@<host>` and as the host of an scp-style
  remote path (REQ-records-006).
