---
change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
artifact: docs
---

# Docs

The changelog fragment records that `chough doctor` no longer reports a host-name leak
for a path component equal to the local user name, and that the name still counts in a
URL, as `<user>@<host>` and as the host of an scp-style remote path. `docs/cli.md`
mentions the host rule without naming this exception, so it needs no change.
