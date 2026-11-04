---
id: serialize-checkpoint-writers-with-an-inter-process-lock-and-remove-session-temps-left-by-killed-writers-issues-62-and
state: archived
type: bug_fix
base_commit: 4daf861509b70c14282ac16b5d0547d701e385b2
---

# Serialize checkpoint writers with an inter-process lock and remove session temps left by killed writers (issues 62 and 64)

## Intent

Serialize checkpoint writers with an inter-process lock and remove session temps left by killed writers (issues 62 and 64)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Concurrent chough checkpoint writers in one clone keep every update: in 8 rounds of three parallel writers every process succeeds and every decision observation and bug link is in the final checkpoint; a writer that cannot take the checkpoint lock within 30 seconds refuses with refused:checkpoint-busy and saves nothing; a session temp left by a writer killed before its rename is removed by the next checkpoint writer with a stderr notice naming it and the session keeps only saved updates while a writer killed after its rename keeps its update and nothing is removed; only regular files named .session.json.<digits>.tmp directly in .chough are removed and read-only commands remove nothing; init still leaves only .chough/session.json and refused writers still change nothing under .chough; the new tests fail with src at main 4daf861 and docs/cli.md describes the lock and cleanup and fledge lanes run verify passes; the test suite leaves no chough-checkpoint lock files in the system temporary directory

## No-spec Rationale

Checkpoint integrity within REQ-cli canonical checkpoint as written: one canonical checkpoint that writers replace atomically; serializing writers and removing temps left by dead writers restores that guarantee across processes without changing requirement or criterion text; docs/cli.md describes the lock and cleanup
