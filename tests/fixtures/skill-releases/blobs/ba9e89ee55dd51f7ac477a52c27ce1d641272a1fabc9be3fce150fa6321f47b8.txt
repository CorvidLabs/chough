# Project librarian

The librarian is a project-local knowledge folder, on by default in `knowledge/` at
the project root. Opt out when the human asks: `chough pref set librarian off --source
"<their words>"`. The folder name can be changed with `chough pref set library-dir
<name> --source …`. It is not a shared cross-project vault, not a ledger, and no gate
reads it. `chough next` names its index.

`chough init` creates the folder, and every `chough stage advance` into done refreshes
it after the trim (`applied.librarian_index`), so each archived change has its entry.
Commit it with the checkpoint. It is generated: on a merge conflict inside it, take
either side and run `chough library index` to regenerate it.

It is an Obsidian-style vault (REQ-cli-044). Every note starts with YAML front matter
(`id`, `title`, `type`, `tags`, ISO dates) and links with standard Markdown links:
`archives/<change>.md` per archived change, with its modules, requirements and traced
HI criteria; `modules/<module>.md` with the module's sources, lesson count and the
changes that touched it; `intent/<file>.md` with each criterion ID and the changes that
trace it; and `rules.md` with the rules, skills and decisions. Notes point to sources
and never copy lesson or criterion text. Open the folder in Obsidian to follow the
links and backlinks.

Old proof leaves `.chough/` as changes go cold (REQ-cli-033, REQ-cli-045):
- At done, the checkpoint drops references to proof from earlier cycles; the archives
  and the vault keep them.
- Of the routine runs of one case and environment, only the newest run and the newest
  failure stay protected. Older runs of a cold change are summarized and trimmed.
- Proof that no archive lists belongs to the change whose cycle it was observed in, and
  follows the same rules once that change is cold.
- Loose artifacts that no record references, such as old pull request folders, follow
  the same rules once their change is cold. An open bug keeps its fixture.
- Key items move into the vault instead: manual and agent-assisted observations,
  failed runs' logs, screenshots and failure-first logs go to
  `knowledge/attachments/<change>/`, and the change's note links them.
- Every removal is ledgered first in `.chough/pruned.jsonl`, and the bytes stay in Git.

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
- `library index` also writes one `archives/<change-id>.md` entry per archived
  native change (newest 3 hot), a lessons index of pointers to each module's lesson
  source and the archived lesson bundles, and lists an earlier `library/` folder as is.
- `library archive` previews the post-archive summary: proof only cold archives list,
  committed identically at HEAD, outside every retention protection. `--apply` needs
  the human's retention consent and no open native change; never run it unasked.
- Briefs in `briefs/` are focused next-session summaries written from
  [the brief template](../templates/brief.md), linking to originals.

## Closeout

Closeout is one quiet skill pass inside the open native change, before `specsync
change check <id> --commit` and finalize. It is a skill step, not a command or a gate,
and it works whether the librarian is on or off (REQ-cli-015):

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
