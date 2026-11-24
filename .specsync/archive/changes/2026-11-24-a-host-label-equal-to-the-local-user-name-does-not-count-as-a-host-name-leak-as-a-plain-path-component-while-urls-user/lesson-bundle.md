# Lesson bundle: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: A host label equal to the local user name does not count as a host-name leak as a plain path component, while URLs, user@host and scp-style hosts keep matching (REQ-records-006)
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/leaks.rs, tests/leaks.rs, changelog.d/2026-11-23-leak-host-equals-user.md
- **Acceptance**: chough doctor on this Mac reports no host-name finding for a case path component equal to the user name; a record holding the same name in a URL, as <user>@<host> or as an scp-style host is still refused; cargo test passes with new leak tests for each of these.

## Evidence

- Verification commit: `80791c9a6296773575af708c74af96f04a551ac3`
- Base commit: `6e00b813a0d52be20a361b625cb96db03be171d6`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

On this Mac the host name is `Tob.hsd1.co.comcast.net` with LocalHostName `Tob`, and the
local user is `tob`. `chough doctor` on the tob repository reported six `host-name`
findings, all on the same lines: the case-definition path component
`Sources/tob/` in `.chough/cases/image.comfy-backend.cases.toml`. The host rule counts
a short all-letter label as a host name inside a path, and the folder `tob` there is
the product's source directory, not a machine. The user-name rule deliberately does not
count such a folder ("a relative path … holds no user name"), so the host rule re-flags
exactly what the user rule let through. The same name still leaks meaningfully in a URL,
as `<user>@<host>`, or as the host of an scp-style remote path, and the full dotted and
`.local` names keep matching as they do now.

## From the change's testing.md

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

## Where these lessons go

- `specs/records/context.md`
