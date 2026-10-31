# Wording review (TC-cli-0103)

Reviewer: Claude (Opus 5.5), agent-assisted; not a human review. Revision: the
product tip after the scoped native check on feat/resumable-stage-workflow.

Method: case-insensitive searches of skills/, src/, README.md and docs/*.md for
soak, live health, proof or proves, deploy, and consent next to preference, then
reading every remaining match in context.

1. Pass. "Proof" appears only as "never treated as behavioral proof" (README) and in
   the unrelated phrase "proof of concept" (roadmap).
2. Pass. Run consent and standing preference are kept apart wherever both appear
   (validate.md, stages.md, prefs.rs messages).
3. Pass. Soak and live health appear only as negations or as the definition of the
   live-health-observation kind.
4. Pass. Deployment appears as a distinct observation kind or in negations;
   simulated deploy scripts are recorded as dry-run.

Limits: keyword search plus reading the matches, not every sentence; English only;
no localization in scope.
