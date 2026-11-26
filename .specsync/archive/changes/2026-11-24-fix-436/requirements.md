# Requirements — fix-436

## Acceptance criteria
1. When a gate closes with a relayed human decision and `--decided-by`
   resolves to a configured human through a name or alias, the rendered
   decision text shows that human's configured `name` (the display
   name), not the raw typed text.
2. When the typed `--decided-by` already equals the display name, the
   rendered text is unchanged.
3. The relay record (`decided_by` field) keeps the raw typed value —
   the audit trail is untouched; only the rendered decision sentence
   changes.
4. New tests pin: alias close renders the display name; exact-name
   close is unchanged. Full `cargo test --locked` stays green.
5. docs/cli.md's decision-format wording names the configured display
   name.