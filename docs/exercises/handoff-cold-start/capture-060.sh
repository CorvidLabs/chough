#!/bin/sh
# Regenerates the 0.6.0 records block of seed.sh (the handoff cold start, TC-cli-0133).
#
# It seeds the 0.1.0 tree with `seed.sh --records=0.1.0` into a throwaway directory and
# runs the given Chough 0.6.0 build there: `chough migrate`, then
# `chough migrate --apply <digest> --consent "SYNTHETIC: ..."` with the previewed plan's
# digest. Both run on this machine and account, as the plan's redactions require. It
# checks the result:
# - the apply succeeded;
# - no 0.5.x record is left;
# - `chough next` reads the migrated session;
# - no file holds a local path, the capturing user name or this host name.
#
# Then it replaces the block between the BEGIN and END lines of seed.sh. The new block
# removes the 0.5.x files, writes byte for byte every file the migration wrote or
# changed, and commits them as the seventh commit. Last, it seeds again with the new
# block and checks that the tree equals the migrated one.
#
# The block keeps the dates the migration wrote; the receipt is named by the day the
# capture ran. Once embedded they are static, so every later seed builds the same tree.
# Commands run in a cleared environment, so no agent, model, hc terminal or identity of
# the capturing session reaches the records.
#
# Usage: sh capture-060.sh <path-to-a-chough-0.6.0-binary> [<build label>]
# The optional label, such as the commit the binary was built from, is added to the
# block's comment beside the binary's --version.
# Needs: sh, git, python3.
set -eu

fail() { printf 'capture failed: %s\n' "$*" >&2; exit 1; }

here=$(cd "$(dirname "$0")" && pwd)
binary=${1:-}
label=${2:-}
[ -n "$binary" ] || fail "usage: sh capture-060.sh <path-to-a-chough-0.6.0-binary> [<build label>]"
case "$binary" in /*) ;; *) binary=$(pwd)/$binary ;; esac
[ -f "$binary" ] && [ -x "$binary" ] || fail "$binary is not an executable file"
for tool in git python3; do
    command -v "$tool" >/dev/null 2>&1 || fail "missing required tool: $tool"
done

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT INT TERM
mkdir "$work/home"

# A cleared environment: PATH, the user name the leak guard checks, and the
# reproducible Git settings seed.sh uses. Nothing else of this session passes.
run() {
    env -i PATH="$PATH" HOME="$work/home" XDG_CONFIG_HOME="$work/home/.config" \
        USER="${USER:-}" LOGNAME="${LOGNAME:-}" TZ=UTC LC_ALL=C \
        GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 "$@"
}

run sh "$here/seed.sh" "$work/seed" --records=0.1.0 >/dev/null
version=$(run "$binary" --version)
[ -z "$label" ] || version="$version ($label)"

run "$binary" migrate --root "$work/seed" --json >"$work/plan.json" 2>"$work/plan.err" ||
    fail "chough migrate refused: $(cat "$work/plan.json" "$work/plan.err")"
digest=$(python3 -I -c 'import json, sys; print(json.load(open(sys.argv[1]))["plan_digest"])' \
    "$work/plan.json") || fail "no plan_digest in the preview"

consent="SYNTHETIC: Robin Ash, the fixture's maintainer, consented to migrating csvsum to Chough 0.6.0 records (seed.sh, captured by capture-060.sh)"
run "$binary" migrate --root "$work/seed" --apply "$digest" --consent "$consent" \
    >"$work/apply.out" 2>"$work/apply.err" ||
    fail "chough migrate --apply refused: $(cat "$work/apply.out" "$work/apply.err")"
run "$binary" next --root "$work/seed" >/dev/null 2>"$work/next.err" ||
    fail "chough next cannot read the migrated tree: $(cat "$work/next.err")"

day=$(date -u +%Y-%m-%d)
(
    cd "$work/seed"
    run git status --porcelain=v1 -z --untracked-files=all
) >"$work/status"

python3 -I - "$work/seed" "$work/status" "$here/seed.sh" "$day" "$version" "$digest" "$work/plan.json" <<'PY'
import json
import os
import re
import socket
import stat
import sys

tree, status_path, seed_path, day, version, digest, plan_path = sys.argv[1:8]
kept = set(json.load(open(plan_path)).get("kept", {}))

def quote(text):
    return "'" + text.replace("'", "'\\''") + "'"

entries = open(status_path, "rb").read().split(b"\0")
removed, written = [], []
for entry in entries:
    if not entry:
        continue
    code, path = entry[:2].decode(), entry[3:].decode()
    if "D" in code:
        removed.append(path)
    elif code in ("??", " M", "M ", "A "):
        written.append(path)
    else:
        sys.exit(f"capture failed: unexpected status {code!r} for {path}")
removed.sort()
written.sort()

legacy_files = {
    ".chough/session.json", ".chough/session.v1.json", ".chough/transitions.jsonl",
    ".chough/pruned.jsonl",
}
legacy_dirs = (".chough/history/", ".chough/evidence/", ".chough/artifacts/")
left = []
for root, dirs, files in os.walk(tree):
    dirs[:] = [d for d in dirs if d != ".git"]
    for name in files:
        rel = os.path.relpath(os.path.join(root, name), tree)
        if rel in kept:
            continue
        if (
            rel in legacy_files
            or rel.startswith(legacy_dirs)
            or (rel.startswith(".chough/bugs/") and rel.endswith(".json"))
            or (rel.startswith(".chough/sessions/") and rel.count("/") == 2 and rel.endswith(".json"))
        ):
            left.append(rel)
if left:
    sys.exit("capture failed: 0.5.x records are left: " + ", ".join(sorted(left)))

markers = ["/Users/", "/home/", "/private/", "/tmp/", "/var/", "/Volumes/", "/mnt/", "~/"]
user = os.environ.get("USER", "")
host = socket.gethostname().split(".")[0]
leaks = []
blocks = []
for path in written:
    full = os.path.join(tree, path)
    info = os.lstat(full)
    if not stat.S_ISREG(info.st_mode):
        sys.exit(f"capture failed: {path} is not a regular file")
    try:
        text = open(full, "rb").read().decode("utf-8")
    except UnicodeDecodeError:
        sys.exit(f"capture failed: {path} is not UTF-8 text")
    for marker in markers:
        if marker in text:
            leaks.append(f"{path}: a local path ({marker})")
    # A name counts as Chough's own guard counts it: as a whole word, not inside
    # another one (a host named `mac` is not found in `machine-local`).
    for word, kind in ((user, "the capturing user name"), (host, "this host name")):
        whole = r"(?<![A-Za-z0-9-])" + re.escape(word) + r"(?![A-Za-z0-9-])"
        if len(word) >= 3 and re.search(whole, text, re.IGNORECASE):
            leaks.append(f"{path}: {kind}")
    line = f"    printf '%s' {quote(text)} >{quote(path)}"
    if info.st_mode & 0o111:
        line += f"\n    chmod 755 {quote(path)}"
    blocks.append(line)
if leaks:
    sys.exit("capture failed: leaks in the migrated records:\n" + "\n".join(leaks))

dirs = sorted({os.path.dirname(p) for p in written if os.path.dirname(p)})
body = [
    "records_060() {",
    f"    # Captured from {version} on {day}; migration plan {digest}.",
]
if removed:
    body.append("    git rm -q -- \\")
    body += [f"        {quote(p)} \\" for p in removed[:-1]]
    body.append(f"        {quote(removed[-1])}")
if dirs:
    body.append("    mkdir -p -- " + " ".join(quote(d) for d in dirs))
body += blocks
body.append(f'    commit "{day}T12:00:00Z" "Migrate the Chough records to 0.6.0 (chough migrate --apply)"')
body.append("}")

begin = "# BEGIN 0.6.0 RECORDS (generated by capture-060.sh)\n"
end = "# END 0.6.0 RECORDS\n"
seed = open(seed_path).read()
if seed.count(begin) != 1 or seed.count(end) != 1:
    sys.exit("capture failed: seed.sh lacks exactly one BEGIN and one END line")
head, rest = seed.split(begin)
_, tail = rest.split(end)
open(seed_path, "w").write(head + begin + "\n".join(body) + "\n" + end + tail)
print(f"captured {len(written)} written and {len(removed)} removed files into seed.sh")
PY

expected=$(cd "$work/seed" && run git add -A && run git write-tree)
run sh "$here/seed.sh" "$work/reseed" >/dev/null
actual=$(cd "$work/reseed" && run git rev-parse 'HEAD^{tree}')
[ "$expected" = "$actual" ] ||
    fail "seed.sh with the new block builds tree $actual, not the migrated tree $expected"
printf 'Captured %s (plan %s); seed.sh now builds tree %s\n' "$version" "$digest" "$actual"
