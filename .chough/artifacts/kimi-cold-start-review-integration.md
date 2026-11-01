# Kimi cold-start integration documentation review (TC-cli-0106)

Reviewer: Kimi Code CLI, agent-assisted; not a human review. Environment:
`repository-text`. Sources: `.chough/artifacts/kimi-cold-start.log`, the guidance at
base `5af3f78`, current canonical and installed guidance, affected documents, and
BUG-0025 with its linked evidence. This is a later documentation review, not another
Kimi execution.

1. Pass. The original log supports the one coached run's host, `.agents` discovery,
   cold-resume failure, commands, timings, bugs, and toolchain facts without enlarging
   them. The original verdict remains discovery passed and safe cold resume failed.
2. Pass. The base guidance lacked the Kimi installation mapping. The current canonical,
   `.agents`, and `.claude` guidance copies are byte-identical and tell Kimi Code CLI
   users to use `--host openai` for `.agents/skills/chough-workflow/`. They do not
   invent a `kimi` host value.
3. Pass. The affected documents consistently preserve BUG-0024 as the original Kimi
   archived-resume record, explain that its earlier native-routing fix was superseded
   during merge by main's newer REQ-cli-021 new-cycle behavior, and distinguish open
   BUG-0020. BUG-0025 is the distinct integration defect. I inspected its later linked
   pass `tc-cli-0105-bug-0025-post-fix` and the linked log, which reports one test
   passed and zero failed after the retained pre-fix failure, before accepting the
   verified status. The roadmap and walkthrough retain the content-identity freshness
   behavior, with the ancestry check and separate reuse label unapproved.
4. Pass. This review is distinct from the original run and records no human decision
   on anyone's behalf. The documents claim no fresh post-fix Kimi run. The original
   failed TC-cli-0106 observation remains unchanged.

Limits: one coached Kimi run on one host and OS; the supplied task named the
repository, objective, `AGENTS.md`, and canonical skill path; the original baseline
verify used HI 0.5.0 and Fledge 1.8.0 rather than the CI versions; no fresh post-fix
Kimi rerun occurred. This observation is a documentation review only, not an automated
product-test execution, hosted check, human acceptance, merge, release, or deployment result.
