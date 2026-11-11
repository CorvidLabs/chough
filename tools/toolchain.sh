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

# The first word of `<program> --version` that starts with a digit, or nothing.
version_of() {
    "$1" --version 2>/dev/null | head -n 1 | tr ' ' '\n' | grep '^[0-9]' | head -n 1
}

# The version of the copy of `$1` that PATH finds first, or nothing.
installed() {
    command -v "$1" >/dev/null 2>&1 || return 0
    version_of "$1"
}

# Every copy of `$1` on PATH, in the order PATH finds them: `<path> <version>` lines.
copies() {
    seen=":"
    old_ifs=$IFS
    IFS=:
    set -f
    for dir in $PATH; do
        [ -n "$dir" ] || dir=.
        case "$seen" in *":$dir:"*) continue ;; esac
        seen="$seen$dir:"
        if [ -f "$dir/$1" ] && [ -x "$dir/$1" ]; then
            printf '%s %s\n' "$dir/$1" "$(version_of "$dir/$1")"
        fi
    done
    set +f
    IFS=$old_ifs
}

# Where `$2` (shown as `$1`) is on PATH, for a failure: every copy with its version,
# the one in use first, and which to fix. `$3` is the pin; `$4` is `minimum` when any
# version at or above it is fine. Paths stay out of the verdict that evidence quotes.
locate() {
    list="" first="" better=""
    found=$(copies "$2")
    while read -r path version; do
        [ -n "$path" ] || continue
        if [ -z "$first" ]; then
            first=$path
            list="$path (${version:-unknown version}, used)"
        else
            list="$list, $path (${version:-unknown version})"
            if [ -n "$version" ]; then
                if [ "$4" = minimum ]; then
                    if at_least "$version" "$3"; then better=yes; fi
                elif [ "$version" = "$3" ]; then
                    better=yes
                fi
            fi
        fi
    done <<COPIES
$found
COPIES
    wanted=$3
    if [ "$4" = minimum ]; then wanted="$3 or later"; fi
    if [ -z "$first" ]; then
        where="$where
  $1: no copy on PATH; install $wanted"
    elif [ -n "$better" ]; then
        where="$where
  $1 on PATH, first found first: $list; a copy at $wanted is on PATH after the one used: put it first, or upgrade or remove $first"
    else
        where="$where
  $1 on PATH, first found first: $list; upgrade it to $wanted"
    fi
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
where=""
note() { report="$report$1
"; }
differ() { differs="${differs:+$differs; }$1"; }

for tool in "HI hi human-intent" "SpecSync specsync specsync" "Fledge fledge fledge"; do
    set -- $tool
    want=$(pin "$3")
    have=$(installed "$2")
    note "$1: installed ${have:-not installed}, CI pins $want"
    if [ "$have" != "$want" ]; then
        differ "$1 ${have:-not installed} (CI $want)"
        locate "$1" "$2" "$want" exact
    fi
done

minimum=$(sed -n 's/^rust-version = "\(.*\)"/\1/p' "${CHOUGH_CARGO_FILE:-$root/Cargo.toml}")
for command in rustc cargo; do
    have=$(installed "$command")
    note "Rust ($command): installed ${have:-not installed}, CI stable and minimum $minimum"
    if [ -z "$have" ] || ! at_least "$have" "$minimum"; then
        differ "Rust ($command) ${have:-not installed} (minimum $minimum)"
        locate "Rust ($command)" "$command" "$minimum" minimum
    fi
done

printf '%s' "$report"
if [ -n "$differs" ]; then
    verdict="differs: $differs"
    echo "toolchain: differs from CI: $differs$where" >&2
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
    printf '%s\n%s\n%s%s\n' "$verdict" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$report" "$where" > "$log.$$" && mv -f "$log.$$" "$log"
fi

[ -z "$differs" ]
