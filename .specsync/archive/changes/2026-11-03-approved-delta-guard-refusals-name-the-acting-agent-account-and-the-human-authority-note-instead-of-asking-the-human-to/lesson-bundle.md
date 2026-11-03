# Lesson bundle: approved-delta-guard-refusals-name-the-acting-agent-account-and-the-human-authority-note-instead-of-asking-the-human-to

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Approved-delta guard refusals name the acting agent account and the human authority note instead of asking the human to approve under their name
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/stage.rs, tests/, specs/cli/, .chough/
- **Acceptance**: Each approved-delta guard refusal that names a native approval says to record it as the acting agent account with the human authority source, covered scope and limits in the note, matching docs/cli.md, and no longer says to ask the human or to use their name as the actor
- **Acceptance**: The refusal labels, refused moves leaving records byte-identical, and the status-read guidance for other unverifiable comparisons are unchanged

## Evidence

- Verification commit: `31119bebbff17573128e4c81920917d74eab4b11`
- Base commit: `8300b584e6e76c676d3da2f21aab9eb7e89b311a`
- Verified by: `specsync check --spec cli`

## From the change's context.md

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

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-022, TC-cli-delta-guard-refusals-name-the-agent-actor | `tests/stages.rs::delta_guard_refusals_name_the_agent_actor_and_the_human_authority_note`: an edited approved delta (native-delta-changed) and a definition approval without digests (native-delta-unverifiable) each refuse `stage advance`; each refusal names `specsync change approve <id> --actor "<agent-account>" --note "Human authority: <source>; covered scope: <scope>; limits: <limits>"` and `specsync change check <id>`, contains neither "ask the human" nor "<their name>", and leaves records byte-identical. It fails at 8300b58 before the fix. |
| REQ-cli-022, TC-cli-0123, TC-cli-0124 | The existing route and unverifiable tests keep every assertion; only the quoted approval remedy changes to the agent-account form. The status-read remedy for other unverifiable comparisons is unchanged. |

## Focused and full checks

`cargo test --test stages`, the focused case run through `chough cases run`, and `fledge lanes run verify` with hi 0.8.0, fledge 1.8.0 and specsync 6.1.2.

## Where these lessons go

- `specs/cli/context.md`
