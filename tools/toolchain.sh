#!/bin/sh
# The `toolchain` task (REQ-cli-019, BUG-0022): print the installed HI, SpecSync and
# Fledge versions beside the versions the CI workflow pins (read by linux/pins.sh, from
# .github/workflows/ci.yml or $CHOUGH_CI_FILE), and Rust beside the minimum Cargo.toml
# declares (or $CHOUGH_CARGO_FILE), then fail naming each one that differs. CI builds
# with stable Rust and tests the minimum, so a newer local Rust is not a difference and
# an older one is. The verify lane runs this first.
#
# The verdict, `matches` or `differs: <each difference>`, then the UTC time and the
# report, goes to .chough/local/logs/toolchain.log (or $CHOUGH_TOOLCHAIN_LOG) when the
# project has the Git-ignored local folder (REQ-history-007), so evidence recorded on a
# toolchain that differs can say so as a limitation.
set -u
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/.." && pwd)
pins=$(sh "$here/linux/pins.sh") || exit 1
pin() { printf '%s\n' "$pins" | sed -n "s/^$1=//p"; }

# The first word of `<command> --version` that starts with a digit, or nothing.
installed() {
    command -v "$1" >/dev/null 2>&1 || return 0
    "$1" --version 2>/dev/null | head -n 1 | tr ' ' '\n' | grep '^[0-9]' | head -n 1
}

# Whether dotted version $1 is at least $2.
at_least() {
    awk -v have="$1" -v want="$2" 'BEGIN {
        n = split(have, h, "."); m = split(want, w, ".")
        for (i = 1; i <= (n > m ? n : m); i++) {
            if (h[i] + 0 > w[i] + 0) exit 0
            if (h[i] + 0 < w[i] + 0) exit 1
        }
        exit 0
    }'
}

report=""
differs=""
note() { report="$report$1
"; }
differ() { differs="${differs:+$differs; }$1"; }

for tool in "HI hi human-intent" "SpecSync specsync specsync" "Fledge fledge fledge"; do
    set -- $tool
    want=$(pin "$3")
    have=$(installed "$2")
    note "$1: installed ${have:-not installed}, CI pins $want"
    [ "$have" = "$want" ] || differ "$1 ${have:-not installed} (CI $want)"
done

minimum=$(sed -n 's/^rust-version = "\(.*\)"/\1/p' "${CHOUGH_CARGO_FILE:-$root/Cargo.toml}")
for command in rustc cargo; do
    have=$(installed "$command")
    note "Rust ($command): installed ${have:-not installed}, CI stable and minimum $minimum"
    if [ -z "$have" ] || ! at_least "$have" "$minimum"; then
        differ "Rust ($command) ${have:-not installed} (minimum $minimum)"
    fi
done

printf '%s' "$report"
if [ -n "$differs" ]; then
    verdict="differs: $differs"
    echo "toolchain: differs from CI: $differs" >&2
else
    verdict="matches"
    echo "toolchain: matches CI"
fi

log=${CHOUGH_TOOLCHAIN_LOG:-}
if [ -z "$log" ] && [ -f "$root/.chough/local/.gitignore" ]; then
    log="$root/.chough/local/logs/toolchain.log"
fi
if [ -n "$log" ]; then
    mkdir -p "$(dirname "$log")"
    printf '%s\n%s\n%s' "$verdict" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$report" > "$log.$$" && mv -f "$log.$$" "$log"
fi

[ -z "$differs" ]
