---
name: chough-grok
description: Grok working through the Chough workflow
command: grok
---
Work through the Chough workflow. Read the project's AGENTS.md, then load the chough-workflow skill from .agents/skills/chough-workflow/SKILL.md and follow it. Run `chough next --environment local` to resume. If the project has no .chough/ folder yet, run `chough init --objective "<goal>" --host openai` first; it never overwrites files. Stop at the human gates (intent, scope, acceptance) and ask; never record a human decision the human did not make. Don't start other agents unless the task says to. When done or stuck, report with `hc send` to whoever started you, if you were given an address.

Task:
