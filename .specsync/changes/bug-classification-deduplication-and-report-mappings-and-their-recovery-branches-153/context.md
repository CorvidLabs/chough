---
change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
artifact: context
---

# Context

Issue #153. The audit (main `700dad96`) rated "Chough: bugs" at 90%: "the entire classification/dedup/reporting surface is not independently exercised". Its 95% list asks to "complete classification, deduplication and report mappings; then exercise their negative/recovery branches with retained failure/fix proof".

Probing those branches on main `64aa77e0` (macOS) found two defects and two refusals that name no recovery:
1. After `chough bug update B --duplicate-of A`, `chough bug update B --status open` is refused: "duplicate_of is only set for duplicate bugs". No flag clears `duplicate_of`, so a mistaken duplicate cannot be reopened except by hand-editing the record.
2. `chough bug update A --related A` is accepted, while `--duplicate-of A` on A is refused.
3. "unknown bug: BUG-0099" (from update and report) and "a bug cannot duplicate itself" name no recovery command.
4. "verified needs a linked failed observation …" names no command either.

## Assignment and IDs

From the orc, deciding overnight for Leif (2026-11-06): own issue, a worktree from origin/main, failing tests first, a mutation check, and the approval queued for Leif. REQ-cli-054 is taken from the Chough ID registry: main's highest is 052, 053 is on a branch, and 054 was found nowhere.
