#!/bin/sh
# regenerate.sh <tag> — rebuild tests/fixtures/releases/<tag>/ from scratch.
#
# Builds the tagged chough release in a throwaway worktree (heavy build, so it
# waits its turn on the shared heavy-tests lock), then drives that binary
# through a small scenario in a neutral temporary project and copies the
# resulting .chough tree plus the project files the records need into the
# fixture folder. Stub `hi` and `specsync` stand-ins on PATH mean no real
# native state is needed (the same pattern as the repository's tests).
#
# Neutral by construction: HOME, USER and the project live under
# /tmp/chough-fixture/<tag>/; the script fails if the fixture mentions the
# caller's real home, user or host.
set -eu

TAG="${1:-}"
case "$TAG" in
  v0.1.0|v0.2.0|v0.3.0|v0.4.0|v0.5.0) ;;
  *) echo "usage: regenerate.sh <v0.N.0>" >&2; exit 2 ;;
esac

CALLER_HOME=$HOME
HERE=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
REPO=$(git -C "$HERE" rev-parse --show-toplevel)
LOCK=/Users/leif/Development/_CorvidLabs/_worktrees/.locks/heavy-tests.lock
BUILD=/tmp/chough-build/$TAG
WORK=/tmp/chough-fixture/$TAG
BIN=$BUILD/target/release/chough
OUT=$HERE/$TAG

# --- 1. build the tag -------------------------------------------------------
if [ ! -x "$BIN" ]; then
  if [ ! -d "$BUILD" ]; then
    git -C "$REPO" worktree add "$BUILD" "$TAG" >/dev/null
  fi
  echo "building $TAG (waits for the shared lock)…" >&2
  /usr/bin/lockf -k "$LOCK" "$HOME/.cargo/bin/cargo" +1.96.0 build --release --locked \
    --manifest-path "$BUILD/Cargo.toml" >&2
fi
[ -x "$BIN" ] || { echo "no release binary for $TAG" >&2; exit 1; }

# --- 2. neutral temporary project -------------------------------------------
rm -rf "$WORK"
mkdir -p "$WORK/home" "$WORK/project" "$WORK/bin" "$WORK/project/specs/fixture"
cd "$WORK/project"
export HOME="$WORK/home" USER=fixture LOGNAME=fixture
export PATH="$WORK/bin:$PATH"

# `hi` stand-in: one small export holding the criteria the fixture references.
cat > "$WORK/bin/hi" <<'EOF'
#!/bin/sh
printf '%s\n' '{"hi":1,"export":1,"files":[{"criteria":[{"id":"LIFE-1"},{"id":"LIFE-3"},{"id":"FLOW-6"},{"id":"FLOW-7"},{"id":"FLOW-8"}],"retired":[{"id":"LIFE-2"}]}]}'
EOF

# `specsync` stand-in (from the repository tests): summary fields come from
# .stub-* files so the scenario can move native state along.
cat > "$WORK/bin/specsync" <<'EOF'
#!/bin/sh
state=$(cat .stub-state 2>/dev/null || echo draft)
approval=$(cat .stub-approval 2>/dev/null || echo true)
correction=$(cat .stub-correction 2>/dev/null || echo true)
validity=$(cat .stub-validity 2>/dev/null || echo authenticated-history)
next=$(cat .stub-next 2>/dev/null || echo "stub next action for $state")
affected=$(cat .stub-affected 2>/dev/null || echo '')
nospec=$(cat .stub-no-spec 2>/dev/null || echo true)
fields=",\"correction_valid\":$correction"
if [ "$approval" != none ]; then fields="$fields,\"approval_valid\":$approval"; fi
if [ "$state" = archived ] && [ "$validity" != none ]; then
  fields="$fields,\"terminal_evidence\":{\"validity\":\"$validity\"}"
fi
change=""
if [ "$affected" != none ]; then
  change=",\"change\":{\"affected_specs\":[$affected],\"no_spec_change\":$nospec}"
fi
printf '{"summary":{"id":"%s","state":"%s","next_action":"%s"%s}%s}\n' "$3" "$state" "$next" "$fields" "$change"
EOF
chmod +x "$WORK/bin/hi" "$WORK/bin/specsync"

# --- 3. project scaffolding the records need --------------------------------
git init -q
git config user.name "Fixture"
git config user.email "fixture@example.invalid"
git config commit.gpgsign false
cat > README.md <<'EOF'
# Fixture project

A tiny project captured with an old chough release for the 0.6.0 migration
tests.
EOF
cat > specs/fixture/requirements.md <<'EOF'
# Fixture requirements

- **REQ-fixture-001**  The fixture project exists so old chough records have a
  contract companion to bind to.
EOF
cat > specs/fixture/fixture.spec.md <<'EOF'
# Fixture spec

Stand-in contract so old chough evidence has something to point at.
EOF
git add . && git commit -q -m "start"

# --- 4. drive the tagged chough ---------------------------------------------
"$BIN" init --objective "Fixture objective for $TAG" --intent-scope FIXT --module fixture >/dev/null

# stage moves: an iteration, a design skip, an advance (transitions + history)
"$BIN" stage iterate --note "fixture iteration" >/dev/null
"$BIN" stage skip --reason "no visuals in a fixture" >/dev/null
"$BIN" stage advance >/dev/null

# one case definition, checked and run
mkdir -p .chough/cases
cat > .chough/cases/fixture.cases.toml <<'EOF'
profile = "chough-testing/1"
module = "fixture"

[[case]]
id = "TC-fixture-0001"
title = "The fixture project's one case"
status = "active"
track = "functional"
method = "automated"
criteria = ["LIFE-1"]
requirements = ["REQ-fixture-001"]
paths = ["README.md"]
environments = ["fixture"]
artifacts = ["test-log"]
expected = ["the README prints its heading"]

[case.bind]
adapter = "command"
argv = ["cat", "README.md"]
expect_output = ["# Fixture project"]
EOF
"$BIN" cases check >/dev/null
"$BIN" cases run TC-fixture-0001 --environment fixture >/dev/null

# an evidence observation with a small artifact (v0.1.0 wants the ID
# positional and the artifact flag; later versions accept both too)
printf 'fixture artifact: one small line\n' > artifact.txt
# v0.1.0 wants the observation ID positional and the artifact flag; later
# versions refuse an old-style ID, so it is omitted there
if [ "$TAG" = v0.1.0 ]; then
  ID_ARG=fixture-observation
else
  ID_ARG=
fi
"$BIN" evidence add $ID_ARG --criterion LIFE-1 \
  --requirement REQ-fixture-001 --contract specs/fixture/fixture.spec.md \
  --kind product-test --outcome passed --environment fixture \
  --method automated --summary "fixture observation: the one case passes" \
  --artifact artifact.txt >/dev/null

# a bug
"$BIN" bug add --title "Fixture bug: example record" \
  --symptom "a fixture symptom" --trigger "capturing the fixture" \
  --expected "no bug" --actual "a demonstration record" \
  --environment fixture >/dev/null

git add . && git commit -q -m "fixture records"

# an active and an archived session where the version has sessions
case "$TAG" in
  v0.4.0)
    "$BIN" session adopt fixture-first >/dev/null
    "$BIN" session new fixture-second --objective "Second fixture objective" >/dev/null
    # drive the first session to done against the archived stub state, then
    # switching sessions archives it
    echo archived > .stub-state
    "$BIN" session use fixture-first >/dev/null
    i=0
    while [ $i -lt 10 ]; do
      "$BIN" stage advance >/dev/null 2>&1 || true
      i=$((i + 1))
    done
    "$BIN" session use fixture-second >/dev/null
    [ -f .chough/sessions/archive/fixture-first.json ] || {
      echo "v0.4.0: fixture-first did not archive" >&2; exit 1; }
    ;;
  v0.5.0)
    "$BIN" session adopt fixture-first >/dev/null
    "$BIN" session new fixture-second --objective "Second fixture objective" >/dev/null
    "$BIN" session close fixture-second --reason "fixture demonstrates an archived session" >/dev/null
    [ -f .chough/sessions/archive/fixture-second.json ] || {
      echo "v0.5.0: fixture-second did not archive" >&2; exit 1; }
    ;;
esac

git add . && git commit -q -m "fixture sessions" 2>/dev/null || true

# --- 5. copy into the fixture folder ----------------------------------------
rm -rf "$OUT"
mkdir -p "$OUT"
cp -R .chough "$OUT/.chough"
rm -rf "$OUT/.chough/local"   # machine-local state never ships
cp README.md artifact.txt "$OUT/"
mkdir -p "$OUT/specs/fixture"
cp specs/fixture/requirements.md specs/fixture/fixture.spec.md "$OUT/specs/fixture/"

# --- 6. leak check ----------------------------------------------------------
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

echo "$TAG fixture regenerated at $OUT" >&2
du -sh "$OUT" >&2
