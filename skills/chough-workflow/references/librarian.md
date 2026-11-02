# Optional project librarian

The librarian is a project-local knowledge folder, off by default. Turn it on only
when the human wants it: `chough pref set librarian on --source "<their words>"`.
The folder name defaults to `library` and can be changed with `chough pref set
library-dir <name> --source …`. It is not a shared cross-project vault.

## From any stage

Enter it as a side stage and return to the caller:

```sh
chough stage enter librarian --reason "Snapshot the approved prototype before redesign"
chough library snapshot design/prototype/index.html design/prototype/COMPANION.md --reason "Approved version 1"
chough library index
chough stage return --note "Snapshot SNAP-approved-version-1-<24-lowercase-hex> taken"
```

- `library init` creates `README.md`, `INDEX.md` and `briefs/`.
- `library snapshot` copies originals into
  `snapshots/SNAP-<slug>-<24-lowercase-hex>/` with a random 96-bit suffix and a
  manifest of source path, Git revision, SHA-256 and creation time. Sources are
  never moved or edited, and their bytes and digests stay unchanged. Repeated
  snapshots of identical inputs remain distinct history.
- `library index` regenerates `INDEX.md`: living sources and their owners, active
  and archived native changes, the open bug backlog, recent observations, briefs and
  snapshots. Legacy `snapshots/<date>-<n>/` directories remain listed beside new
  SNAP directories without renaming or migration. Manifest timestamps determine
  chronology, never the snapshot ID or random suffix.
- Briefs in `briefs/` are focused next-session summaries written from
  [the brief template](../templates/brief.md), linking to originals.

## Boundaries

Living HI, prototypes, contracts and native archives stay in their owning locations;
a snapshot is history, not a second current copy. Summaries complement originals.
Unfixed bugs remain in the live backlog even when an old report is snapshotted. The
mandatory checkpoint does not depend on the librarian.
