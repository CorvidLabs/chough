---
change: approved-delta-guard-refusals-name-the-acting-agent-account-and-the-human-authority-note-instead-of-asking-the-human-to
artifact: context
---

# Context

Found while working Leif's 2026-11-03 backlog item on contradictory recovery guidance.
Route reproduced at main 8300b58: an approved change whose delta is edited after
approval, or whose definition approval has no `approved_delta_digests`, refuses
`chough stage advance` with a remedy that says "ask the human for a new scope
approval (specsync change approve <id> --actor \"<their name>\")".

That contradicts the current lifecycle rule (skills/chough-workflow/references/lifecycle.md,
Approval; plan.md, One approval) and docs/cli.md's refusal table, which say the acting
agent account records the approval and the human instruction supplies the authority in
the note. Duplicate search: no open or closed GitHub issue and no local bug covers it;
docs, skill references and package fixtures were already corrected, only these two
`src/stage.rs` strings remain. Old-skill fixtures under tests/fixtures/ and the
KNOWN_PREVIOUS texts in tests/workflow_authority.rs and tests/package.rs are frozen
history and stay unchanged.

Human authority: Leif (0xLeif), standing instruction in chat 2026-11-03 7:49 AM MT ("I, 0xLeif, authorize bounded fixes to existing Chough bug tickets through implementation, testing, independent review, native closeout and merge after all checks pass"), which names this investigation: "investigate contradictory recovery guidance that still asks the human to operate native tools or uses the wrong acting-account wording". Covered scope: the two approved-delta guard refusal messages in src/stage.rs, their tests, REQ-cli-022 wording, one case and one local bug. Limits: no new product scope, no breaking change, no release or deployment, refusal labels and gate behavior unchanged.
