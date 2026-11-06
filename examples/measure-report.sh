#!/bin/sh
# Before/after measurement of `chough next` (REQ-reporting-003).
#
# Runs two freshly built chough binaries against one frozen input root and prints what
# a fair comparison needs: the root and its revision, each binary's SHA-256 and version,
# first-process and repeated runs, and local time apart from native wait (from the
# CHOUGH_TRACE=1 native lines). A native read that did not complete (timed out,
# unavailable, failed, oversized or invalid) makes the native side incomplete: its wait
# is not compared, and no line ever combines local and native time into one verdict, so
# a local change is never reported as fixing native slowness. A first process does not
# imply a purged OS cache.
#
# Usage: sh examples/measure-report.sh OLD NEW ROOT [RUNS]
# RUNS defaults to 3 per binary. CHOUGH_NATIVE_TIMEOUT_SECONDS applies to both binaries.
set -eu

usage() {
    echo "usage: sh examples/measure-report.sh OLD NEW ROOT [RUNS]  (OLD and NEW are chough binaries, ROOT a Git project with a commit, RUNS a positive integer, default 3)" >&2
    exit 2
}

[ $# -ge 3 ] && [ $# -le 4 ] || usage
old=$1 new=$2 root=$3 runs=${4:-3}
for binary in "$old" "$new"; do
    if [ ! -f "$binary" ] || [ ! -x "$binary" ]; then
        echo "measure-report: $binary is not an executable file" >&2
        usage
    fi
done
if ! git -C "$root" rev-parse --verify HEAD >/dev/null 2>&1; then
    echo "measure-report: $root is not a Git project with a commit" >&2
    usage
fi
case $runs in
    '' | *[!0-9]* | 0) echo "measure-report: RUNS must be a positive integer" >&2; usage ;;
esac

sha256() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d ' ' -f 1
    else
        shasum -a 256 "$1" | cut -d ' ' -f 1
    fi
}
now_ms() {
    perl -MTime::HiRes=time -e 'printf "%d\n", time * 1000'
}
# Median of the numbers on standard input, one per line (the lower middle for an even count).
median() {
    sort -n | awk '{ values[NR] = $1 } END { if (NR == 0) print 0; else print values[int((NR + 1) / 2)] }'
}

root=$(cd "$root" && pwd -P)
revision=$(git -C "$root" rev-parse HEAD)
if [ -n "$(git -C "$root" status --porcelain)" ]; then dirty=yes; else dirty=no; fi
work=$(mktemp -d "${TMPDIR:-/tmp}/chough-measure.XXXXXX")
trap 'rm -rf "$work"' EXIT

echo "root $root revision $revision dirty $dirty"
echo "native_timeout_seconds ${CHOUGH_NATIVE_TIMEOUT_SECONDS:-default}"
echo "binary old sha256 $(sha256 "$old") version $("$old" --version)"
echo "binary new sha256 $(sha256 "$new") version $("$new" --version)"

incomplete=0
for side in old new; do
    if [ "$side" = old ]; then binary=$old; else binary=$new; fi
    run=1
    while [ "$run" -le "$runs" ]; do
        if [ "$run" -eq 1 ]; then kind=first-process; else kind=repeated; fi
        start=$(now_ms)
        set +e
        CHOUGH_TRACE=1 "$binary" --root "$root" next --json >"$work/stdout" 2>"$work/trace"
        code=$?
        set -e
        wall=$(($(now_ms) - start))
        grep '^chough-trace: native ' "$work/trace" >"$work/native" || true
        reads=$(wc -l <"$work/native" | tr -d ' ')
        open=$(grep -vc ' state=observed ' "$work/native" || true)
        native=$(grep ' state=observed ' "$work/native" | sed 's/.* elapsed_ms=\([0-9]*\).*/\1/' | awk '{ total += $1 } END { print total + 0 }')
        local_ms=$((wall - native))
        echo "run $side $run $kind exit=$code wall_ms=$wall native_ms=$native local_ms=$local_ms native_reads=$reads incomplete_reads=$open"
        echo "$local_ms" >>"$work/local-$side"
        echo "$native" >>"$work/native-$side"
        incomplete=$((incomplete + open))
        run=$((run + 1))
    done
done

echo "summary local old_median_ms=$(median <"$work/local-old") new_median_ms=$(median <"$work/local-new")"
if [ "$incomplete" -eq 0 ]; then
    echo "summary native old_median_ms=$(median <"$work/native-old") new_median_ms=$(median <"$work/native-new")"
else
    echo "summary native incomplete: $incomplete native reads timed out or were unavailable; no native comparison"
fi
