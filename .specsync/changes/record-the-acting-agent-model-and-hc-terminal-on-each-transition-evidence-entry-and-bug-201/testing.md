---
change: record-the-acting-agent-model-and-hc-terminal-on-each-transition-evidence-entry-and-bug-201
artifact: testing
---

# Testing

| Requirement | Coverage |
|---|---|
| REQ-cli-063 | `TC-cli-actor-identity-recorded` binds `transitions_evidence_and_bugs_record_the_acting_agent` in tests/evidence.rs: override variables on a transition and an evidence add, Claude Code and hc markers on a bug add, `last_actor` in next and status, and no `actor` when nothing is known |

After scope approval, `TC-cli-actor-identity-recorded` is added in `.chough/cases/cli.actor-identity-201.cases.toml` (before then `chough cases check` cannot resolve REQ-cli-063) and run on the materialized requirement. The full suite must stay green with the fixture clearing the variables, wherever it runs (Claude Code, hc, CI).
