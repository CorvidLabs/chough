---
id: never-overwrite-an-attachment-let-folder-references-protect-their-files-and-keep-a-loose-folder-with-any-current-file
state: implementing
type: bug_fix
base_commit: d165b0a20d05f29c884e6970a30fb50b88c950fd
---

# Never overwrite an attachment, let folder references protect their files, and keep a loose folder with any current file (109 review P1s)

## Intent

Never overwrite an attachment, let folder references protect their files, and keep a loose folder with any current file (109 review P1s)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Moves into the vault never overwrite: an identical attachment is kept and survives a refused apply, a different one keeps both files and protects the set, and one written during the apply refuses it. A folder named by the checkpoint or an open bug keeps the loose files and observation sets inside it. A loose folder with any current file stays whole. The new tests fail first, fledge lanes run verify passes on Linux, and the trim reruns on this repository at closeout.

## No-spec Rationale

Not applicable
