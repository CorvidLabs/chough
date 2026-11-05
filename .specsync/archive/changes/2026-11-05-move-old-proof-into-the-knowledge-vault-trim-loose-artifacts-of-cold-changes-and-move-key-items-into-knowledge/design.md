---
change: move-old-proof-into-the-knowledge-vault-trim-loose-artifacts-of-cold-changes-and-move-key-items-into-knowledge
artifact: design
---

# Design

- **`attachments::is_key_item(path)`:** screenshot extensions, or a lowercased name
  containing `failure-first`, `failing-first`, `fails-first`, `before-fix`,
  `pre-fix` or `reproduction`.
- **`attachments::attachment(dir, archive, file)`:** gives
  `<dir>/attachments/<archive>/<file minus .chough/>`.
- **`attachments::first_commits(root)`:** one `git log --diff-filter=A
  --format=@%ct --name-only -- .chough/artifacts`, keeping the oldest time per file.
- **`attachments::loose_groups`:** groups the unreferenced files by top-level entry,
  skipping hidden ones. It applies the checkpoint and open-bug protections and the
  hot-archive protection, then picks the owner: a cold lister, or by cycle time from the
  earliest first commit.
- **`SummaryCandidate`:** gains `loose` and `moves`.
  - An observation set whose only reasons are non-routine moves whole.
  - Otherwise the artifacts of a failed set, and its key-item artifacts, move.
  - A loose group moves its key items.
- **Apply:** loads evidence only for observation sets. Loose groups are ledgered with
  `kind: loose-artifact`, `removed`, `moved` and `artifacts` digests. Moved files
  are copied before removal, and a restore deletes the copies. The entries are written
  again after a successful apply.
- **Archive notes:** gain "Loose artifacts" (from the ledger) and "Attachments" (the
  files under `attachments/<archive>/`) sections, with links.
