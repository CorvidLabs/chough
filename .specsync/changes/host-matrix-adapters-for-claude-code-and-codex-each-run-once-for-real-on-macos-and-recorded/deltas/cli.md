## ADDED

### REQUIREMENT REQ-cli-067

Host-matrix adapters for Claude Code and Codex (LIFE-21 item 4, LIFE-22, LIFE-27).
- `integrations/claude-code/adapter.sh` SHALL run one Claude Code turn with `claude -p` and stream-json output. `integrations/codex/adapter.sh` SHALL run one Codex turn with `codex exec --json`, keeping no session files. Both SHALL run without permission prompts, with Codex's sandbox off because it refuses writes to `.git`, and only inside the throwaway sample, with the user's own configuration.
- A `host_turn` that cannot run the host now, for want of a login or because of a usage limit, SHALL exit 75. `run.sh` SHALL then end the leg as UNAVAILABLE, naming the turn and the reason; any other failure stays a failed check.
- Every harness turn prompt SHALL tell the host not to start other agents, terminals or hc.
- Each adapter's first real macOS leg SHALL be recorded as an agent-assisted case run, TC-cli-host-matrix-claude-code or TC-cli-host-matrix-codex, with its verdict (PASS, FAIL or UNAVAILABLE) and receipt. `docs/integrations.md` SHALL report the observed result per host.

Acceptance Criteria
- `sh tools/host-matrix/selftest.sh` prints `ok: usage-limit -> UNAVAILABLE` and `HOST MATRIX SELFTEST PASSED` under dash and bash.
- Both adapters exist, and each host's first macOS leg is recorded with its verdict and receipt.
