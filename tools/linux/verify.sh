#!/bin/sh
# `fledge run linux-verify`: run `fledge lanes run verify` on the current commit inside
# chough-linux:3 (REQ-cli-058, DRIVE-22). The commit is cloned from a Git bundle, so the
# container never reads the host's .git. Refuses with exit 3 before verifying when a
# tool in the image is not the pinned version.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
if ! docker image inspect chough-linux:3 >/dev/null 2>&1; then
    echo "linux-verify: no chough-linux:3 image; build it with \`fledge run linux-image\`" >&2
    exit 3
fi
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
commit=$(git -C "$root" rev-parse HEAD)
git -C "$root" bundle create -q "$work/repo.bundle" HEAD
chmod -R a+rX "$work"
echo "linux-verify: $commit"
docker run --rm -v "$work":/bundle:ro -e COMMIT="$commit" chough-linux:3 sh -c '
    set -eu
    git clone -q /bundle/repo.bundle /tmp/repo
    cd /tmp/repo
    git checkout -q "$COMMIT"
    sh tools/linux/check-versions.sh
    fledge lanes run verify
'
