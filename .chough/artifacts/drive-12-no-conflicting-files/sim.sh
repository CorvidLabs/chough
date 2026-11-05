# sim2.sh: two parallel branches from HEAD each start a session, add a case in their own
# file and run it, note one shared bug, rebuild the vault, append a ledger line and add a
# CHANGELOG bullet. With the Chough drivers configured, merge and report what conflicts.
SRC=$PWD; CH=$SRC/target/debug/chough
rm -rf /tmp/sim && git clone -q "$SRC" /tmp/sim && cd /tmp/sim || exit 1
BASE=$(git rev-parse HEAD); BUG=$(ls .chough/bugs | head -1 | sed 's/.json$//')
git config user.name sim; git config user.email sim@example.invalid; git config commit.gpgsign false
git config merge.chough-session.driver "$CH checkpoint --merge %O %A %B"
git config merge.chough-bug.driver "$CH bug merge %O %A %B"
for b in a b; do
  git checkout -q -B sim-$b "$BASE"
  $CH session new sim-change-$b --objective "parallel work $b" >/dev/null || exit 2
  cat > .chough/cases/cli.sim-$b.cases.toml <<CASES || exit 3
profile = "chough-testing/1"
module = "cli"

[[case]]
id = "TC-cli-sim-$b"
title = "Simulated case $b"
status = "active"
track = "functional"
method = "automated"
criteria = ["DRIVE-12"]
requirements = ["REQ-cli-001"]
paths = ["src/"]
environments = ["local-linux-aarch64"]
artifacts = ["test-log"]
expected = ["It prints ok"]

[case.bind]
adapter = "command"
argv = ["echo", "ok $b"]
expect_output = ["ok $b"]
CASES
  git add -A && git commit -q -m "case $b" || exit 4
  $CH cases run TC-cli-sim-$b --environment local-linux-aarch64 >/dev/null || exit 5
  $CH bug update "$BUG" --note "Seen on branch $b" >/dev/null || exit 6
  $CH stage pause --reason "wait $b" >/dev/null && $CH stage resume >/dev/null || exit 7
  printf '{"schema_version":1,"event":"simulated","id":"sim-%s"}\n' "$b" >> .chough/pruned.jsonl
  python3 - "$b" <<'PY'
import sys
p="CHANGELOG.md"; s=open(p).read()
s=s.replace("## [Unreleased]\n\n", "## [Unreleased]\n\n- Simulated change %s.\n" % sys.argv[1], 1)
open(p,"w").write(s)
PY
  $CH library index >/dev/null || exit 8
  git add -A && git commit -q -m "parallel work $b" || exit 9
  echo "$b changed $(git diff --name-only "$BASE" HEAD | wc -l) files, $(git diff --name-only "$BASE" HEAD -- knowledge | wc -l) of them in knowledge/"
done
echo "merge-tree in a clone without the Chough drivers configured:"
git config --unset merge.chough-session.driver; git config --unset merge.chough-bug.driver
git merge-tree --write-tree --name-only --no-messages sim-a sim-b | tail -n +2 | sed 's/^/  conflict: /'
git config merge.chough-session.driver "$CH checkpoint --merge %O %A %B"
git config merge.chough-bug.driver "$CH bug merge %O %A %B"
git checkout -q sim-a
if git -c merge.ff=false merge -q --no-edit sim-b >/tmp/merge.log 2>&1; then echo "merge with the drivers: clean"; else echo "merge with the drivers: CONFLICT"; git diff --name-only --diff-filter=U | sed 's/^/  /'; fi
echo "  bug notes kept: $(grep -c 'Seen on branch' .chough/bugs/$BUG.json)"
echo "  case files: $(ls .chough/cases/cli.sim-*.cases.toml | wc -l); cases check: $($CH cases list >/dev/null 2>&1 && echo ok || echo failed)"
echo "  knowledge check before rebuild: $($CH library index --check >/dev/null 2>&1 && echo current || echo stale)"
$CH library index >/dev/null; echo "  knowledge check after chough library index: $($CH library index --check >/dev/null 2>&1 && echo current || echo stale)"
