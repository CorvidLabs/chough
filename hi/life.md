---
hi: 1
families: [LIFE]
---

# Life

## Intent

I want the lifecycle to keep my intent intact and make it easy to resume with
honest evidence. I should not need a new agent runtime or a hosted service to use
it. This captures the desired product; the first implementation covers a focused
slice, with the remaining work in the roadmap.

## Criteria

- **LIFE-1**  I can carry a natural-language request from human intent through requirements, implementation, verification, review, delivery, and lessons without losing why it matters.
- **LIFE-2**  My acceptance criterion IDs remain permanent while evidence states what is actually achieved for a code revision and environment.
- **LIFE-3**  I can resume work with focused context, decisions, open questions, and the next action rather than loading the whole history.
- **LIFE-4**  I can use skills and the same workflow from supported agent hosts or a small optional command-line helper.
- **LIFE-5**  I can reproduce, classify, deduplicate, and report bugs and connect them to criteria and verification evidence.
- **LIFE-6**  I can reuse HI, SpecSync, Fledge, and other appropriate CorvidLabs tools instead of maintaining duplicate lifecycle records.
- **LIFE-7**  I can see missing or stale evidence and distinguish structural checks, product tests, human acceptance, and delivery verification.
- **LIFE-8**  Routine reruns do not keep raw test logs forever, while the proof behind active work, open and fixed bugs, and accepted or released baselines stays, and anything removed is shown as removed rather than as passed.
- **LIFE-9**  I can install Chough into another project and get the helper and skills without Chough's own backlog or run history.
- **LIFE-10**  I can see how long each stage, test, bug fix and human decision actually took, so I can estimate future work from real history.
- **LIFE-11**  I can keep working as a project grows with a larger bounded progress checkpoint and observation reference list, without dropping evidence I still need. Source: Leif in chat, 2026-11-02 9:40 PM MT, "Do your agent's cleanup and raise the progress file and test-result limits." This is an agent-normalized capture of that qualitative direction. Root selects 131,072 checkpoint bytes and 400 observation pointers as implementation values under the standing through-PR authority and the later direct instruction to finish these PRs and merge. The cleanup remains in #50.
- **LIFE-12**  I keep one full copy of the Chough skill per project, in the shared `.agents/skills` directory, and asking for the Claude host installs or updates that same shared copy instead of a second `.claude/skills` copy. Source: Leif in chat, 2026-11-03, interview answer "Only the shared .agents copy" and 8:27 AM MT answer "Remove .claude entirely", chosen knowing Claude Code 2.1.316 did not discover `.agents/skills` alone.
- **LIFE-13**  An existing `.claude/skills` copy in a project I already use is never deleted silently by init or update; it is reported as a legacy copy and removed only when I run an explicit command, which keeps edited files in place. Source: Leif in chat, 2026-11-03 8:27 AM MT, "Remove .claude entirely"; the no-silent-deletion limit is an agent-normalized safeguard from his standing rule to preserve existing files.
- **LIFE-14**  Chough's own repository carries no `.claude` skill copy, and its documentation says honestly that Claude Code 2.1.316 was observed not to discover `.agents/skills` alone, so Claude Code reaches the workflow through `CLAUDE.md` and `AGENTS.md` until a newer run shows otherwise. Source: Leif in chat, 2026-11-03 8:27 AM MT, "Remove .claude entirely".
