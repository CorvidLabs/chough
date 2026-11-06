---
change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
artifact: tasks
---

# Tasks

- [x] Add the failing FLOW-18 checks in tests/docs.rs and see them fail on the base.
- [x] Correct README.md, docs/ and the diagrams, and re-render stages, lifecycle and ownership SVGs.
- [x] Correct SKILL.md, references/checkpoint.md and references/lifecycle.md and their .agents copies, and register their 615b587b digests at the end of KNOWN_PREVIOUS in src/skill.rs.
- [x] Add the Unreleased CHANGELOG entry and the docs/verification.md render record.
- [x] Run fledge lanes run verify and specsync change audit --strict locally.

Not tasks (human gates and Chough records): orc's scope approval relay, running and recording the FLOW-18 cases, the failing-first bug link, the human implementation review, and merge on orc's relay.
