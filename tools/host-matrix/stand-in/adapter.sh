# Adapter for the scripted stand-in host (REQ-cli-066): no model, used by the
# self-test. Sourced by tools/host-matrix/run.sh; every adapter defines the same names
# (see tools/host-matrix/README.md).
# shellcheck disable=SC2034 # run.sh, which sources this file, reads these settings

HOST_LABEL=stand-in
HOST_INIT_ARGS="--host openai"
HOST_SKILL=.agents/skills/chough-workflow/SKILL.md
HOST_SKILL_READ='chough-workflow/SKILL\.md'
HOST_TURN_TIMEOUT=${HOST_TURN_TIMEOUT:-600}

host_check() {
    command -v jq >/dev/null 2>&1 || { echo "jq is not installed"; return 1; }
}

host_version() {
    printf 'stand-in (scripted, no model)\n'
}

# host_turn <prompt-file> <log-file> <reply-file>: one fresh run in the current directory.
host_turn() {
    HOST_INIT_ARGS=$HOST_INIT_ARGS sh "$HM_ROOT/tools/host-matrix/stand-in/agent.sh" <"$1" >"$2" 2>&1 || return $?
    sed -n '/^--- reply ---$/,$p' "$2" | sed '1d' >"$3"
}
