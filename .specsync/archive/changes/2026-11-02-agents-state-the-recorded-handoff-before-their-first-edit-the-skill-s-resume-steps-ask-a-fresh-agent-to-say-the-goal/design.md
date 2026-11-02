---
change: agents-state-the-recorded-handoff-before-their-first-edit-the-skill-s-resume-steps-ask-a-fresh-agent-to-say-the-goal
artifact: design
---

# Design

## The skill step

skills/chough-workflow/SKILL.md, "Start or resume", gains a new step 4; today's step
4 (read the stage contract) becomes step 5. Proposed text:

```markdown
4. Before your first edit, state the handoff in your own message. Reading it in tool
   output is not the same as stating it. Say the goal; each recorded decision that
   binds the next task, with its reason, naming a rejected approach as rejected; the
   last failure, its bug and that bug's status; the lessons that apply and where they
   are recorded; and the next task. An edit is any file change, including a Chough
   command that writes a record. Where the records do not say one of these, say so
   instead of guessing.
```

Seven lines, no dash, about 560 bytes; SKILL.md goes from 4874 to about 5440 bytes,
under the 6144-byte limit in the criteria. It lives in SKILL.md, not a reference,
because it must run before the agent knows which stage reference to open, and
SKILL.md is the one file every host loads.

## Why skill-only

The hypothesis under test is about the skill, so this change moves only that. `chough
next` already prints every fact the rubric asks for (the earlier change run showed
it). Adding a "state this before editing" line to the next view would change
REQ-cli-021 output and mix two causes in one proof. If the change run still fails,
that line is the next candidate, as its own change.

## What a pass can and cannot show

The step mirrors rubric items P1 to P3, so a pass shows that the agent follows the
step, not that it understood more. Two checks reach beyond that: P4 to P6 (it starts
the right task without repeating the failed experiment or the rejected approach),
and the real-repository run, whose expected facts are not the rubric's. One run per
arm on one host and model is a single sample; the pull request will say so.

## Installed copies and earlier digests

`chough skill update` refreshes .agents/ and .claude/. KNOWN_PREVIOUS gains the
118a895 SKILL.md digest so an unedited copy from main updates. The test keeps the
118a895 bytes inline as a const in tests/package.rs, as the learning change did for
checkpoint.md, because a fixture directory would sit outside the affected paths.
