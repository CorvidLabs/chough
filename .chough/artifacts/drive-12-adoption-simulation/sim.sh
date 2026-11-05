# sim.sh <base-ref> <legacy|sessions>: two parallel branches each record Chough work, a
# ledger line and a CHANGELOG bullet; report the files git merge-tree finds conflicted.
BASE=$1; MODE=$2; SRC=$PWD; CH=$SRC/target/debug/chough
rm -rf /tmp/sim && git clone -q "$SRC" /tmp/sim && cd /tmp/sim || exit 1
BASE=$(git rev-parse "$BASE") || exit 1
git config user.name sim; git config user.email sim@example.invalid; git config commit.gpgsign false
for b in a b; do
  git checkout -q -B sim-$b "$BASE"
  if [ "$MODE" = sessions ]; then $CH session new sim-change-$b --objective "parallel work $b" >/dev/null || exit 2; fi
  $CH checkpoint --objective "parallel work $b" --next-action "continue $b" >/dev/null || exit 3
  $CH stage reset --to plan --reason "start $b" >/dev/null || exit 4
  $CH stage pause --reason "wait $b" >/dev/null || exit 5
  $CH stage resume >/dev/null || exit 6
  printf '{"schema_version":1,"event":"simulated","id":"sim-%s"}\n' "$b" >> .chough/pruned.jsonl
  python3 - "$b" <<'PY'
import sys
p="CHANGELOG.md"; s=open(p).read()
s=s.replace("## [Unreleased]\n\n", "## [Unreleased]\n\n- Simulated change %s.\n" % sys.argv[1], 1)
open(p,"w").write(s)
PY
  git add -A && git commit -q -m "parallel work $b" || exit 7
  git diff --name-only "$BASE" HEAD | sed "s/^/  $b changed: /"
done
echo "conflicts ($MODE at $(git rev-parse --short "$BASE")):"
git merge-tree --write-tree --name-only --no-messages sim-a sim-b | tail -n +2 | sed 's/^/  /'
echo "merge-tree exit: $(git merge-tree --write-tree sim-a sim-b >/dev/null 2>&1; echo $?)"
