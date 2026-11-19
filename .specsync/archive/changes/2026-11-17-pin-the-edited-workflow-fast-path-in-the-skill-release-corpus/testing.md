---
change: pin-the-edited-workflow-fast-path-in-the-skill-release-corpus
artifact: testing
---

# Testing

`cargo test --locked --test skill_releases` passed locally: 7 passed. That is the check that hashes `skills/chough-workflow/references/stages.md` against `current_bundle_sha256` and hashes the manifest against `MANIFEST_SHA256`.
