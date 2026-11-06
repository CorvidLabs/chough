#!/bin/sh
# Print the tool versions the CI workflow pins, one `tool=version` line each, for the
# Linux test images (REQ-cli-058, DRIVE-22). Reads .github/workflows/ci.yml, or
# $CHOUGH_CI_FILE. Refuses, naming the tool, when a pin is missing or the specsync pins
# disagree, so an image is never built from a guess.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
ci=${CHOUGH_CI_FILE:-$here/../../.github/workflows/ci.yml}
[ -f "$ci" ] || { echo "pins: no CI workflow at $ci" >&2; exit 1; }

# The `version:` under each `uses: <action>@` step, without quotes or a leading v.
action_versions() {
    awk -v action="$1" '
        index($0, "uses: " action "@") { want = 1; next }
        want && /^[[:space:]]*-[[:space:]]/ { want = 0 }
        want && /^[[:space:]]*version:/ {
            value = $0
            sub(/^[[:space:]]*version:[[:space:]]*/, "", value)
            gsub(/["\047]/, "", value)
            sub(/^v/, "", value)
            print value
            want = 0
        }
    ' "$ci" | sort -u
}

specsync=$(action_versions CorvidLabs/spec-sync)
fledge=$(action_versions CorvidLabs/fledge)
human_intent=$(sed -n 's/.*cargo install human-intent --version \([^ ]*\).*/\1/p' "$ci" | sort -u)

check() {
    count=$(printf '%s\n' "$2" | grep -c . || true)
    if [ "$count" -eq 0 ]; then
        echo "pins: $ci pins no $1 version" >&2
        exit 1
    fi
    if [ "$count" -gt 1 ]; then
        echo "pins: $ci pins $1 at different versions: $(printf '%s' "$2" | tr '\n' ' ')" >&2
        exit 1
    fi
}
check specsync "$specsync"
check fledge "$fledge"
check human-intent "$human_intent"

printf 'specsync=%s\nfledge=%s\nhuman-intent=%s\n' "$specsync" "$fledge" "$human_intent"
