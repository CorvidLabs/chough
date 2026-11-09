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
chough stage return --note "Snapshot 2026-10-31-1 taken"
```

- `library init` creates `README.md`, `INDEX.md` and `briefs/`.
- `library snapshot` copies originals into `snapshots/<date>-<n>/` with a manifest of
  source path, Git revision and SHA-256. Sources are never moved or edited.
- `library index` regenerates `INDEX.md`: living sources and their owners, active
  and archived native changes, the open bug backlog, recent observations, briefs and
  snapshots.
- Briefs in `briefs/` are focused next-session summaries written from
  [the brief template](../templates/brief.md), linking to originals.

## Boundaries

Living HI, prototypes, contracts and native archives stay in their owning locations;
a snapshot is history, not a second current copy. Summaries complement originals.
Unfixed bugs remain in the live backlog even when an old report is snapshotted. The
mandatory checkpoint does not depend on the librarian.
