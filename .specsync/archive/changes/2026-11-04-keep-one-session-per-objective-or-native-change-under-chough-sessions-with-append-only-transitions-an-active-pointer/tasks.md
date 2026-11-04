---
change: keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer
artifact: tasks
---

# Tasks

- [x] hi/drive.md intent wording brought up to #68's
- [x] Failing-first tests in tests/sessions.rs
- [x] Sessions, archive, pointer and inbox in src/sessions.rs; path resolution in src/checkpoint.rs
- [x] Commands and init refusal in src/main.rs; protection sources in src/history.rs
- [x] .gitattributes, docs/cli.md, CHANGELOG.md, checkpoint reference and its installed copy
- [x] Known-previous digest for main edbc3c1 (v0.2.0) checkpoint reference in src/skill.rs, with a fixture and a failing-first test in tests/workflow_authority.rs
- [x] #77 review (corvid-agent review items 1 to 3): a done session with unfolded entries stays open; fold checks targets and entries before saving (d5bd3d8 tests first, 57b321a)
- [x] Leif's agent's P1 on src/checkpoint.rs:362: each command pins the session it resolved (ada8b93 test first, 8e0152f)
- [x] Leif's agent's P1 on src/checkpoint.rs:335 and src/evidence.rs: retention protects every open and archived session's references and fails closed (ada8b93 test first, 8e0152f)
- [x] Review item 4: MODIFIED REQ-cli-007 and REQ-history-002 traced to DRIVE-3; REQ-history-006 states the keep-open rule and the retention sources; the REQ-history-002 digest in tests/spec_split.rs follows the modified wording
