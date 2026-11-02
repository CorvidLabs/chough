# Lesson bundle: agents-state-the-recorded-handoff-before-their-first-edit-the-skill-s-resume-steps-ask-a-fresh-agent-to-say-the-goal

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Agents state the recorded handoff before their first edit: the skill's resume steps ask a fresh agent to say the goal, binding decisions with reasons, the last failure with its bug status, the applicable lessons and the next task in its own words
- **Kind**: Feature
- **Specs**: cli
- **Paths**: skills/chough-workflow/SKILL.md, .agents/skills/chough-workflow/SKILL.md, .claude/skills/chough-workflow/SKILL.md, src/skill.rs, tests/package.rs, docs/, CHANGELOG.md, specs/cli/
- **Acceptance**: See the Acceptance Criteria section of change.md: skill step, installed copies, contract and docs, cold-start proof, verification.

## Evidence

- Verification commit: `c2218e9b1c3ea7cc7f0d18b3784d3e892f8a064e`
- Base commit: `118a895e6db179b1fbf7abeee9a9e7130cd7e399`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The learning handoff change (PR #35, merged as 118a895) added the decisions, last
failure and lessons blocks to `chough next`. Its cold-start proof (criterion 9)
failed. On the frozen seeded repository, Kimi Code CLI 0.41.0 ran `chough next`, saw
the decisions, the failure with BUG-0001's status and the lesson pointer, and read
`context.md` and `session.json` before its first edit, but none of its own messages
before that edit stated those facts. 0xLeif graded it Fail (P1 to P3 not met, P4 to
P6 met; observation handoff-cold-start-change-kimi-1-grade). The baseline on main
fb6e487 failed P2 and P3 the same way.

The lesson recorded in specs/cli/context.md names the untested hypothesis this
change tests: the helper puts the facts in front of the agent, but the skill never
asks it to state them before acting. 0xLeif accepted PR #35 as is, with criterion 9
failed, and asked for this follow-up as its own change.

For a session picking this up:

- The seed and rubric in docs/exercises/handoff-cold-start/ are frozen and hashed
  (observation handoff-cold-start-frozen-pair-1). Do not edit them; a change voids
  every run.
- Kimi needs rune to drive it, shows a folder trust dialog in a new directory, and
  has a 5-hour usage quota that stopped the earlier real-repository run after 8 tool
  calls. 0xLeif starts Kimi runs himself; agents prepare the scripts.
- Two sessions closed out PR #35 in parallel on one branch, and the second archive
  made SpecSync report corrupt history until the branch was repaired. Only one
  session closes out this change.

## From the change's design.md

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

## From the change's testing.md

# Testing

## Cases

| Case | Requirement | Criteria | Method | What it shows |
|---|---|---|---|---|
| TC-cli-skill-asks-for-the-handoff-before-the-first-edit | REQ-cli-002 | LIFE-3, FLOW-13, LIFE-1, FLOW-12 | agent-assisted, reviewer-note | At the implementation tip SKILL.md's resume list has the step between "Check what is still true" and the stage contract, naming the goal, binding decisions with reasons and rejected approaches, the last failure with its bug status, the lessons with their source, the next task, what counts as an edit and saying what the records lack; at most 8 lines, no dash, SKILL.md under 6144 bytes; `chough skill status` reports both host copies current |
| TC-cli-0133 (existing) | REQ-cli-021 | FLOW-13, LIFE-3, LIFE-1 | manual, transcript | New observations for the baseline at 118a895 and the change run at the tip, graded by 0xLeif against the frozen rubric |

The package test (automated, in the full suite) covers the known-outdated update of
the 118a895 copy and the step's presence. The new case is added after the scoped
check materializes the delta.

## Cold-start protocol

Unchanged from the learning change: frozen seed and rubric, Kimi Code CLI 0.41.0,
model kimi-code/k3.1-pro-highspeed, prompt `Resume this project.`, driven through
rune, trust dialog accepted as recorded host setup. The real-repository run uses
expected facts for this repository written and committed before it starts, in the
style of the earlier real-repo-kimi-1/expected-facts.md. Grades are 0xLeif's
statements, recorded as such.

## Regression

The full suite, `chough cases check` and the verify lane; `chough next` output on this
repository is compared before and after to show it did not change.

## Where these lessons go

- `specs/cli/context.md`
