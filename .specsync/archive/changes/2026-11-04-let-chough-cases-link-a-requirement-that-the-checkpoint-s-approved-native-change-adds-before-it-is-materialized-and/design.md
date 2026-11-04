---
change: let-chough-cases-link-a-requirement-that-the-checkpoint-s-approved-native-change-adds-before-it-is-materialized-and
artifact: design
---

# Design

`cases::check_links` keeps the canonical companion lookup. On a miss it asks a per-command resolver, created once per `cases check`, `run` or `record`. The resolver reads the checkpoint's `native_change` and, only when one is recorded and SpecSync is configured, makes one `specsync change status <id> --json` read, then reuses that result for every case in the command. It accepts the ID only when the state is approved, implementing or verifying, `approval_valid` is true, `correction_valid` is not false, `stage::approved_deltas` reports the deltas unchanged, and `.specsync/changes/<id>/deltas/<module>.md` holds `### REQUIREMENT <ID>` under `## ADDED` or `## MODIFIED`. Any other outcome keeps the existing refusal and appends the reason. `evidence add` keeps its canonical-only lookup.
