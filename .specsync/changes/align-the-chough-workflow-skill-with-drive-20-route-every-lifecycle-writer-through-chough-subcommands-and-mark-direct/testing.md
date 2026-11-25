## Testing

- The check script runs in CI (docs lane) and locally; its failure output names the
  offending file + line.
- Human pass over the four reference files: every `specsync change` fenced command is
  either one of the four labeled exceptions or a chough form.
- No product code changes; `cargo test` unaffected but run once as regression.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-072 | scripts/check-skill-chough-first.sh passes (OK on the changed bundle), verified to FAIL on a planted violation (testing.md fixture, exit 1, then reverted); SKILL.md Drive-the-workflow now states the chough-first order citing DRIVE-20; lifecycle.md/plan.md/accept.md fenced commands labeled unwrapped-step or swapped to chough forms; fledge lanes ci+verify gained the skill-chough-first step; docs/lifecycle.md states the rule. |
