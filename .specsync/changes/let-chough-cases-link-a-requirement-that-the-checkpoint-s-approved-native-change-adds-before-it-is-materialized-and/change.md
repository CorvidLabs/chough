---
id: let-chough-cases-link-a-requirement-that-the-checkpoint-s-approved-native-change-adds-before-it-is-materialized-and
state: approved
type: feature
base_commit: d673efc9fadd00953cf9e42dad96e08f3c76103c
---

# Let Chough cases link a requirement that the checkpoint's approved native change adds before it is materialized, and document how the test stage links failing-first evidence (DRIVE-6, Raven BUG-0004)

## Intent

Let Chough cases link a requirement that the checkpoint's approved native change adds before it is materialized, and document how the test stage links failing-first evidence (DRIVE-6, Raven BUG-0004)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Source: Leif's brief t811u item 6, "Want: either the test stage links failing-first evidence (the commit that adds failing tests, the receipt showing them fail on the base, and the passing run) and cases accept REQ ids from the active approved change, or test folds into build with clear guidance. Pick one and document it.", and Raven BUG-0004 ("chough cases check rejects a requirement that exists only in an approved but not yet materialized SpecSync delta"). The parent relayed the 6:03 PM MST decision to take the first option. Outcomes: (1) a case requirement absent from the module's requirements.md is accepted by cases check, run and record when the checkpoint's native change is approved, implementing or verifying with approval_valid true, correction_valid not false and its approved deltas unchanged (REQ-cli-022), and that change's delta for the case file's module adds or modifies the ID; (2) such a link is reported in cases check output under approved_delta_requirements with the change and module; (3) at most one native status read per command, and none when every link resolves in a companion; (4) a typo, another module's delta, an edited delta, an invalid approval or correction, a draft or archived change, no checkpoint change, no native configuration or a failed native read refuses as before, and the message names why the delta did not resolve it when a change was consulted; (5) evidence add and every stage gate are unchanged, Chough decides no approval and records no human approval or review; (6) the plan, testing, accept and lifecycle references and docs say cases may link approved delta requirements before change check and describe how the test stage links failing-first evidence: the commit that adds the failing tests, a failed observation on the base linked to a bug, and the passing run after the fix; (7) the new tests fail before the fix; fledge lanes run verify passes

## No-spec Rationale

Not applicable
