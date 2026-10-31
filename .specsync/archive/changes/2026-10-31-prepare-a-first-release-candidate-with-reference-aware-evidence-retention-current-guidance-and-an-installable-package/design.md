---
change: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
artifact: design
---

# Design

## Retention (REQ-cli-018)

An observation set is one evidence record plus the artifacts no other record
references. `chough evidence prune [--apply]` classifies every set:

| Protected because | Reason shown |
|---|---|
| Listed in the checkpoint | `checkpoint` |
| Linked by any bug (observation or baseline), any status | `BUG-NNNN <status>` |
| Listed by an active native change manifest | `native active <id>` |
| Listed by the most recent archived native change manifest | `native archive <id>` |
| Not a routine run (manual, agent-assisted, human, delivery, publication, installation, deployment, live-health) | `<kind>` |
| Among the newest 3 automated runs for its case and environment | `newest` |
| Newest failure for its case and environment | `last failure` |
| Failure, first or last pass of a validation run | `validation <run>` |
| Younger than 14 days | `recent` |

Everything else is eligible. The preview prints eligible sets (case, environment,
age, outcome, bytes), protected sets with reasons, totals and a warning above 50 MiB.
Native manifests are read with `specsync change status <id> --json`; if that read is
unavailable, every set it could protect stays protected.

`--apply` needs a run consent `retention` or the standing preference `retention auto`
(values `off`, the default, and `auto`). With `auto`, `cases run` and `validate run`
apply the same rules after recording. Each pruned set appends a line to
`.chough/pruned.jsonl`; files are removed from the working tree only.

Reading references: present and current → `current`; present but stale → `historical`;
absent and in the ledger → `pruned`; absent and not in it → `unavailable`. Pruned and
unavailable never count as current, passed or verifying evidence.

## Package and install (REQ-cli-019)

`Cargo.toml` gains `include = ["/src", "/skills/chough-workflow", "/Cargo.toml",
"/Cargo.lock", "/README.md", "/LICENSE"]`. `KNOWN_PREVIOUS` gains the main-build digests
of bundle files changed by PRs #7 and #8 and this change.

## Guidance

README (public repository, install, platforms, limits), ROADMAP (current states) and
the `docs/architecture.md` records table (one row group per folder: what it is, who
owns it, committed, local or CI artifact, prunable or frozen).
