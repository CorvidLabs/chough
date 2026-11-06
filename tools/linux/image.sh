#!/bin/sh
# `fledge run linux-image`: build chough-linux:1, chough-linux:2 and chough-linux:3 with
# the tool versions the CI workflow pins (REQ-cli-058, DRIVE-22).
set -eu
here=$(cd "$(dirname "$0")" && pwd)
pins=$(sh "$here/pins.sh")
pin() { printf '%s\n' "$pins" | sed -n "s/^$1=//p"; }
set -- \
    --build-arg "SPECSYNC_VERSION=$(pin specsync)" \
    --build-arg "FLEDGE_VERSION=$(pin fledge)" \
    --build-arg "HUMAN_INTENT_VERSION=$(pin human-intent)"
echo "linux-image: $(printf '%s' "$pins" | tr '\n' ' ')"
docker build "$@" --target toolchain -t chough-linux:1 "$here"
docker build "$@" --target dev -t chough-linux:2 "$here"
docker build "$@" --target runner -t chough-linux:3 "$here"
echo "linux-image: built chough-linux:1, chough-linux:2 and chough-linux:3"
