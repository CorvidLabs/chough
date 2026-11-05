---
change: reduce-old-observation-proof-drop-closed-cycle-checkpoint-references-at-done-trim-superseded-runs-of-cold-changes
artifact: design
---

# Design

- **`evidence.rs`:**
  - `Classification` gains `newest_run` (the newest routine run per case and
    environment).
  - `ListedArchive` gains `finalized`.
  - `summary_protections`:
    - attributes unlisted sets with `owner_by_cycle` (first archive whose
      `finalized >= observed_at`);
    - removes `newest` when the set is not in `newest_run`;
    - keeps candidate `archives` (owner or listers) for the ledger and knowledge
      pointer, and `listed_by` for validity.
  - `validity` runs on scoped threads, with at most 8 at once.
  - `attributed_sets(root, archives)` lets library.rs list attributed records per
    archive.
- **`library.rs`:** an archive entry's Observations table adds its attributed records,
  marked "by cycle time", with ledger outcomes for pruned ones.
- **`stage.rs`:** on the move into done, with the librarian on,
  `drop_closed_cycle_references` loads each referenced record's `observed_at`, keeps
  those at or after `latest_reset_at`, saves, and reports
  `applied.checkpoint_references`. Then the existing trim and index refresh run.
