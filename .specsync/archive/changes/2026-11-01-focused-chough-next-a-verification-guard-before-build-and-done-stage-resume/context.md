---
change: focused-chough-next-a-verification-guard-before-build-and-done-stage-resume
artifact: context
---

# Context

0xLeif named three decisions as blocking for 0.1.0, after independent reviewers
measured them:

1. `chough next` and `chough status` share one code path (`src/main.rs`, the
   `Commands::Status { .. } | Commands::Next { .. }` arm) and print the same report;
   `--json` is ignored by both. On this repository the report is about 410 KB, almost
   all of it observation records and raw SpecSync output, which a resuming agent
   must load to find one next action.
2. BUG-0002 (open, out of scope for Chough's code until now): SpecSync 6.1.1 keeps
   `summary.approval_valid` true after an approved delta is edited, and Chough's build
   gate trusts it. Chough should guard this itself before admitting build, test or
   accept.
3. Closed PR #20 (a Gemini or Antigravity cold start) found two real things and got
   them partly wrong: from stage done, `chough next` echoed the old review action;
   its suggested `chough stage reset --to plan` omitted the required `--reason`; and
   its host wording claimed that Gemini CLI and Antigravity read both `.agents/skills`
   and `.gemini/skills`, which no record shows.

Constraints a resuming session needs: draft only until the human approves the scope;
`.chough/` is deliberately not an affected path; no new integration, model client or
storage; open PR #22 (0.1.0 release prep) modifies REQ-cli-008, REQ-cli-016 and
REQ-cli-019 and edits SKILL.md, references/checkpoint.md and KNOWN_PREVIOUS, so this
change adds REQ-cli-021 and REQ-cli-022 instead of modifying those requirements, and
whichever pull request merges second rebases its KNOWN_PREVIOUS lines and bundle
edits. Case IDs TC-cli-0120 to TC-cli-0124 are reserved here; they are free on
origin/main (highest 0104) and on PR #22 (0110 to 0114).
