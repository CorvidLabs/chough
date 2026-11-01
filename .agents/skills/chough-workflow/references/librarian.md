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

## Closeout

Closeout is one quiet skill pass inside the open native change, before `specsync
change check <id> --commit` and finalize. It is a skill step, not a command or a gate,
and it works with the librarian preference off (REQ-cli-015):

1. Write the supported lessons from this change into the module lesson source, each
   with its certainty label (`observation`, `hypothesis` or `verified`), source and
   limits (see [accept.md](accept.md)).
2. Check that new checkpoint decisions carry a reason and that verified bugs link
   their failed and passed observations.
3. Add one `completed` checkpoint entry for the closeout.
4. End with one closeout summary of at most 12 lines.

Closeout never edits specs after archive: a lesson found later goes into a new native
change. A library brief is written only when the librarian is on.

## Boundaries

Living HI, prototypes, contracts and native archives stay in their owning locations;
a snapshot is history, not a second current copy. Summaries complement originals.
Unfixed bugs remain in the live backlog even when an old report is snapshotted. The
mandatory checkpoint does not depend on the librarian.
