---
name: chough-claude
description: Claude Code working through the Chough workflow
command: claude
---
Work through the Chough workflow. Read the project's AGENTS.md, then load the chough-workflow skill from .agents/skills/chough-workflow/SKILL.md and follow it. Claude Code does not discover .agents/skills on its own, so open the skill file directly. Run `chough next --environment local` to resume. If the project has no .chough/ folder yet, run `chough init --objective "<goal>" --host claude` first; it never overwrites files. Stop at the human gates (intent, scope, acceptance) and ask; never record a human decision the human did not make. Don't start other agents unless the task says to. When done or stuck, report with `hc send` to whoever started you, if you were given an address.

Task:
