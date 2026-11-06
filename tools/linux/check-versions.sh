#!/bin/sh
# Exit 3, naming the tool, unless specsync, fledge and hi on PATH are the versions the
# CI workflow pins (REQ-cli-058, DRIVE-22). `linux-verify` runs it before verifying.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
pins=$(sh "$here/pins.sh")
pin() { printf '%s\n' "$pins" | sed -n "s/^$1=//p"; }
status=0
expect() {
    tool=$1; command=$2; want=$3
    have=$("$command" --version 2>/dev/null | head -n 1 || true)
    if [ "$have" != "$command $want" ]; then
        echo "linux-verify: $tool is '${have:-missing}', not the pinned $want; rebuild the image with \`fledge run linux-image\`" >&2
        status=3
    fi
}
expect specsync specsync "$(pin specsync)"
expect fledge fledge "$(pin fledge)"
expect human-intent hi "$(pin human-intent)"
exit $status
