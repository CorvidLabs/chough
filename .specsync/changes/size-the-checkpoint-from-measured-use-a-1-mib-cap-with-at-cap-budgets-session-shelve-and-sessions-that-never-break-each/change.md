---
id: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
state: implementing
type: feature
base_commit: ec830cfb001cc5f71f4f83809ca3294d685f42a4
---

# Size the checkpoint from measured use: a 1 MiB cap with at-cap budgets, session shelve, and sessions that never break each other (DRIVE-14, 125)

## Intent

Size the checkpoint from measured use: a 1 MiB cap with at-cap budgets, session shelve, and sessions that never break each other (DRIVE-14, 125)

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- Every checkpoint is bounded at 1,048,576 bytes; at the cap next, status and context answer within 2 s in the debug test build (250 ms release) with the focused view within 16,384 bytes; one byte over is refused naming the cap and chough session shelve; chough session shelve <key> and --legacy move the oversized checkpoint byte for byte into .chough/sessions/shelved/ so session new and next work; one unreadable session never breaks list, new, use or another session next; retention reads shelved sessions; REQ-cli-021 AC1 holds 40 gates, 40 blockers and 50 decisions of 4 KiB within the cap. The new tests fail first and fledge lanes run verify passes on Linux.

## No-spec Rationale

DRIVE-14 (Leif: 1 MiB from the sizing study, Add shelve, 2026-11-05): REQ-cli-032 cap and budgets, REQ-cli-021 stress shape (C3, #125), REQ-history-006 shelve and per-session robustness.
