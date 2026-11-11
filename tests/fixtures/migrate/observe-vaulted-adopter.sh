#!/bin/sh
# observe-vaulted-adopter.sh <chough binary> — run the three history writes of
# TC-records-migrate-vaulted-adopter on a fresh copy of the committed
# tests/fixtures/migrate/vaulted-adopter tree, with the release fixtures'
# stand-ins, and print each exit code and the first line of its error.
# Run from the chough repository root.
set -u
BIN=$1
REPO=$(pwd)
D=$(mktemp -d)
cp -R "$REPO/tests/fixtures/migrate/vaulted-adopter/." "$D/"
mkdir -p "$D/.bin" "$D/.home"
sed -n '/^cat > "\$WORK\/bin\/hi"/,/^chmod +x/p' "$REPO/tests/fixtures/releases/regenerate.sh" \
  | sed 's#\$WORK/bin#$STUBS#g' > "$D/.stubs.sh"
STUBS="$D/.bin" sh "$D/.stubs.sh"
cd "$D"
git init -q
git add -A
git -c user.name=Fixture -c user.email=fixture@example.invalid -c commit.gpgsign=false commit -qm fixture
export HOME="$D/.home" USER=fixture PATH="$D/.bin:$PATH"
echo "binary: $("$BIN" --version)"
echo "tree: $D"
run() {
  "$BIN" "$@" >/dev/null 2>"$D/.err"
  printf '%s -> exit %s: %s\n' "$*" "$?" "$(head -n 1 "$D/.err")"
}
run checkpoint --awaiting-human Review
if "$BIN" checkpoint --help 2>/dev/null | grep -q -- '--close-gate'; then
  run checkpoint --close-gate Review --decided-by SYNTHETIC-human --words "Reviewed, go on" --where "fixture chat" --recorded-by SYNTHETIC-agent
else
  run checkpoint --clear-awaiting-human
fi
run stage advance
git diff --quiet HEAD -- .chough/transitions.jsonl && echo "transitions.jsonl unchanged"
