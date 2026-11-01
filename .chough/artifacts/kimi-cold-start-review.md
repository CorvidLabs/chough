# Kimi cold-start review (TC-cli-0106)

Reviewer: Kimi Code CLI, agent-assisted; not a human review. Original run base:
`5af3f78`. Source: `.chough/artifacts/kimi-cold-start.log`. This is the original
retrospective review before guidance and documentation fixes.

1. Pass. The original log supports the one run's discovery, cold-resume, command,
   timing, bug, and toolchain facts without enlarging them. It records Kimi Code CLI
   0.41.0 on macOS arm64, supplied task guidance, successful `.agents` discovery,
   archived native state with stale resume guidance, BUG-0020 through BUG-0023,
   and the initial and later tool versions.
2. Fail. The guidance available at base `5af3f78` did not give the `openai` to
   `.agents` installation mapping for Kimi. The existing mapping was undocumented,
   which is BUG-0021. This absence does not justify inventing a `kimi` host.
3. Fail. Consistent corrected guidance and affected documentation were absent at
   base. The documents did not explain the observed mapping or contain the required
   corrected roadmap and observation-reuse statements, so cross-document consistency
   cannot pass for the original review.
4. Pass. This note distinguishes the original run from any later documentation
   review. The log records no agent-recorded human decision, and this review records
   no human decision on anyone's behalf.

Limits: one run on one host and OS; the supplied task named the repository,
objective, `AGENTS.md`, and canonical skill path; the initial baseline verify used
HI 0.5.0 and Fledge 1.8.0 rather than the CI versions; no fresh post-fix Kimi rerun
occurred.
