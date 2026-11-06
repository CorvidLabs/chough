---
id: chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20
state: draft
type: feature
base_commit: 852605831643d975411015c58cc0f9a2919d0568
---

# Chough change ship: ship after the review, close the review gate with the relay, and move to done (179, DRIVE-20)

## Intent

chough change ship: ship after the review, close the review gate with the relay, and move to done (179, DRIVE-20)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- REQ-cli-059 (new): chough change ship <id> with the review gate's relay checks the session, stage, open gates and relay before anything runs (each refusal names a recovery that a test runs), runs specsync change ship <id> unbounded with its output and refusals passed through, then closes the gate as a relayed claim and moves the session to done; a refused step after a successful ship exits 1, says shipped and names the remaining command, which a test runs. The walkthrough and the skill's closeout run chough change new, check and ship, with review still specsync change review. Six tests in tests/change.rs fail first, each mutant in testing.md fails one of them, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
