---
change: fix-295-the-init-case-runs-the-real-preservation-test
artifact: context
---

# Context

Pull request 295 clarified REQ-cli-001, then its case ran `echo preserves`. Review asked to bind that case to `init_preserves_user_files_and_installs_a_complete_skill` and to add the missing-bundle assertion, keeping the original receipts and correcting the story. The requirement text already says this. The archived clarification change cannot cover the new test file, so this change does.
