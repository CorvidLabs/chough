---
change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
artifact: testing
---

# Testing

Each failing on main first, in `tests/leaks.rs`:

- With the user `tob` and the host label `Tob`, a record naming a `tob` path component
  as in the tob repository's source paths is written, and `chough doctor` over it
  reports no `host-name` finding.
- The same name in a URL, as `<user>@<host>`, and as the `<host>` of an scp-style
  remote path is still refused.
- A host label that is not the user name is refused as a path component as before,
  and a label that equals the user only inside a longer name, such as a dotted or
  hyphenated host, is still refused.

Then the full test suite and the verify lane; finally a run of the built binary's
`doctor` on the tob repository, which must report no host-name finding for the case
definition's source paths.
