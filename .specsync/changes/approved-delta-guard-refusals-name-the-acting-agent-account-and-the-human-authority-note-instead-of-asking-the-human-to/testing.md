---
change: approved-delta-guard-refusals-name-the-acting-agent-account-and-the-human-authority-note-instead-of-asking-the-human-to
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-022, TC-cli-delta-guard-refusals-name-the-agent-actor | `tests/stages.rs::delta_guard_refusals_name_the_agent_actor_and_the_human_authority_note`: an edited approved delta (native-delta-changed) and a definition approval without digests (native-delta-unverifiable) each refuse `stage advance`; each refusal names `specsync change approve <id> --actor "<agent-account>" --note "Human authority: <source>; covered scope: <scope>; limits: <limits>"` and `specsync change check <id>`, contains neither "ask the human" nor "<their name>", and leaves records byte-identical. It fails at 8300b58 before the fix. |
| REQ-cli-022, TC-cli-0123, TC-cli-0124 | The existing route and unverifiable tests keep every assertion; only the quoted approval remedy changes to the agent-account form. The status-read remedy for other unverifiable comparisons is unchanged. |

## Focused and full checks

`cargo test --test stages`, the focused case run through `chough cases run`, and `fledge lanes run verify` with hi 0.8.0, fledge 1.8.0 and specsync 6.1.2.
