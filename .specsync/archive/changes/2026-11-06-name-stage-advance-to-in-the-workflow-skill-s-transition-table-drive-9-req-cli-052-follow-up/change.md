---
id: name-stage-advance-to-in-the-workflow-skill-s-transition-table-drive-9-req-cli-052-follow-up
state: archived
type: documentation
base_commit: 01f1d023072da58e11feffd7408c98b9e341c3a9
---

# Name stage advance --to in the workflow skill's transition table (DRIVE-9, REQ-cli-052 follow-up)

## Intent

Name stage advance --to in the workflow skill's transition table (DRIVE-9, REQ-cli-052 follow-up)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The bundled and installed chough-workflow references/stages.md transition tables have a byte-identical row for 'chough stage advance --to <stage>' saying every step is checked first on one native read, a refused step records nothing, and into done it reads native status once. The stages.md that main shipped from 0.1.0 to 01f1d023 (sha256 658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f) is reported known-outdated by 'chough skill status' and becomes current with 'chough skill update', while an edited copy stays edited. The new test fails first, and fledge lanes run verify passes.

## No-spec Rationale

Skill guidance for the existing REQ-cli-052 command, which #147 shipped without a row in the skill's transition table; the earlier stages.md digest joins the known previous copies so installed 0.4.0 copies update; no requirement text changes.
