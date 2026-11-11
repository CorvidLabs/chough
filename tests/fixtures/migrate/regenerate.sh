#!/bin/sh
# regenerate.sh <adopted|vaulted-adopter> — rebuild tests/fixtures/migrate/<name>/ from
# scratch with real old chough builds, for the 0.6.0 migration tests (REQ-records-009).
#
# adopted: a 0.1.0 legacy checkpoint and transitions.jsonl, whose history 0.5.0 adopts
#   (`history adopt --apply`, then `session adopt adopter`); the adopter archived with
#   six more sessions, one more driven to done so the KNOW-29 roll-up moves the oldest
#   into knowledge/attachments/ (0.5.0 keeps the anchor holder out of it); two open
#   sessions, one with a decision that holds a home path and one with inbox entries.
# vaulted-adopter: the same start on the last build before #142 (260ecc94, between
#   0.4.0 and 0.5.0), whose roll-up moves the adopter itself into the vault while
#   transitions.jsonl stays unchanged; then one open session. That build refuses
#   history writes on this tree (the local bug fixed by #142).
#
# Each build is made once in /tmp/chough-build/<name> (a worktree of this repository,
# built --release under the shared heavy lock when CHOUGH_HEAVY_LOCK names it); nothing
# is installed. Stub `hi` and `specsync` stand-ins run on PATH, and HOME, USER and the
# project live under /tmp/chough-fixture/, so the fixture names nothing of the caller's.
set -eu

NAME="${1:-}"
case "$NAME" in
  adopted|vaulted-adopter) ;;
  *) echo "usage: regenerate.sh <adopted|vaulted-adopter>" >&2; exit 2 ;;
esac

CALLER_HOME=$HOME
HERE=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
REPO=$(git -C "$HERE" rev-parse --show-toplevel)
WORK=/tmp/chough-fixture/migrate-$NAME
OUT=$HERE/$NAME

# build <rev> <folder>: the release binary of <rev>, built once.
build() {
  dir=/tmp/chough-build/$2
  bin=$dir/target/release/chough
  if [ ! -x "$bin" ]; then
    [ -d "$dir" ] || git -C "$REPO" worktree add --detach "$dir" "$1" >/dev/null
    if [ -n "${CHOUGH_HEAVY_LOCK:-}" ]; then
      /usr/bin/lockf -k "$CHOUGH_HEAVY_LOCK" "$CALLER_HOME/.cargo/bin/cargo" +1.96.0 build \
        --release --locked --manifest-path "$dir/Cargo.toml" >&2
    else
      "$CALLER_HOME/.cargo/bin/cargo" +1.96.0 build --release --locked \
        --manifest-path "$dir/Cargo.toml" >&2
    fi
  fi
  [ -x "$bin" ] || { echo "no release binary for $1" >&2; exit 1; }
  echo "$bin"
}

LEGACY=$(build v0.1.0 v0.1.0)
case "$NAME" in
  adopted) B=$(build v0.5.0 v0.5.0) ;;
  vaulted-adopter) B=$(build 260ecc949ea01214f4bec43b30ea4226ce8fae59 pre-142) ;;
esac

rm -rf "$WORK"
mkdir -p "$WORK/home" "$WORK/bin" "$WORK/project"
cd "$WORK/project"
export HOME="$WORK/home" USER=fixture LOGNAME=fixture
export PATH="$WORK/bin:$PATH"
# The stand-ins of tests/fixtures/releases/regenerate.sh.
sed -n '/^cat > "\$WORK\/bin\/hi"/,/^chmod +x/p' "$HERE/../releases/regenerate.sh" > "$WORK/stubs.sh"
WORK="$WORK" sh "$WORK/stubs.sh"

git init -q
git config user.name "Fixture"
git config user.email "fixture@example.invalid"
git config commit.gpgsign false
printf '# Migration fixture project\n' > README.md
git add . && git commit -q -m "start"

# The 0.1.0 legacy checkpoint and its transitions.jsonl.
"$LEGACY" init --objective "Adopted objective" --intent-scope FIXT --module fixture >/dev/null
"$LEGACY" stage iterate --note "legacy iteration" >/dev/null
"$LEGACY" stage skip --reason "legacy skip" >/dev/null
"$LEGACY" stage advance >/dev/null
git add -A && git commit -q -m "0.1.0 records"

# Its history adopted, then the checkpoint adopted as a session.
"$B" history adopt --apply >/dev/null
"$B" session adopt adopter >/dev/null

# done_now: drive the active session to done against the archived stub state.
done_now() {
  echo archived > .stub-state
  step=0
  while [ $step -lt 10 ]; do
    "$B" stage advance >/dev/null 2>&1 || true
    step=$((step + 1))
  done
  rm -f .stub-state
}

if [ "$NAME" = adopted ]; then
  # 0.5.0: the adopter and six more sessions closed; two open sessions, one with a
  # decision holding a home path and one with inbox entries; then one more session
  # driven to done, whose roll-up moves the oldest archived session (never the anchor
  # holder) into knowledge/attachments/.
  "$B" stage advance >/dev/null 2>&1 || true
  "$B" stage advance >/dev/null 2>&1 || true
  "$B" session close adopter --reason "adopted history archived" >/dev/null
  i=1
  while [ $i -le 6 ]; do
    "$B" session new "archived-$i" --objective "Archived work $i" >/dev/null
    "$B" stage advance >/dev/null 2>&1 || true
    "$B" session close "archived-$i" --reason "archived work $i done" >/dev/null
    i=$((i + 1))
  done
  "$B" session new open-one --objective "Open work one" >/dev/null
  "$B" checkpoint --decision "Notes kept in /Users/SYNTHETIC-user/notes" >/dev/null
  "$B" stage advance >/dev/null 2>&1 || true
  "$B" session new open-two --objective "Open work two" >/dev/null
  "$B" inbox add --session open-two --from subagent-1 --kind decision --text "Use the smaller fixture" >/dev/null
  "$B" inbox add --session open-two --from subagent-1 --kind question --text "Keep the second run?" >/dev/null
  "$B" session new finisher --objective "Finish to done" >/dev/null
  done_now
  [ -f .chough/sessions/archive/adopter.json ] || {
    echo "adopted: the adopter is not in the session archive" >&2; exit 1; }
  ls knowledge/attachments/*/sessions/*.json >/dev/null 2>&1 || {
    echo "adopted: no session rolled into the vault" >&2; exit 1; }
else
  # The pre-#142 build: no `session close`; a done session is archived by the next
  # `session new`, and the roll-up runs at each move into done with the librarian on.
  # Keeping the five newest by key, it moves the adopter itself into the vault.
  "$B" pref set librarian on --source "Fixture: the librarian keeps the vault" >/dev/null
  done_now
  i=1
  while [ $i -le 5 ]; do
    "$B" session new "archived-$i" --objective "Archived work $i" >/dev/null
    done_now
    i=$((i + 1))
  done
  "$B" session new finisher --objective "Finish to done" >/dev/null
  done_now
  "$B" session new working --objective "Work after the roll-up" >/dev/null
  [ -f knowledge/attachments/adopter/sessions/adopter.json ] || {
    echo "vaulted-adopter: the adopter did not roll into the vault" >&2; exit 1; }
  git diff --quiet HEAD -- .chough/transitions.jsonl || {
    echo "vaulted-adopter: transitions.jsonl changed" >&2; exit 1; }
fi
git add -A && git commit -q -m "fixture sessions"

rm -rf "$OUT"
mkdir -p "$OUT"
cp -R .chough "$OUT/.chough"
rm -rf "$OUT/.chough/local"   # machine-local state never ships
cp -R knowledge "$OUT/knowledge"
cp README.md "$OUT/"

# Leak check: nothing of the caller's home, user or host.
REAL_USER=$(id -un)
REAL_HOST=$(hostname -s)
BAD=0
for pat in "$CALLER_HOME" "$REAL_USER" "$REAL_HOST"; do
  HITS=$(grep -rIl -- "$pat" "$OUT" 2>/dev/null | head -5 || true)
  if [ -n "$HITS" ]; then
    echo "LEAK: fixture mentions $pat:" >&2
    echo "$HITS" >&2
    BAD=1
  fi
done
[ "$BAD" = 0 ] || exit 1
echo "$NAME fixture regenerated at $OUT" >&2
