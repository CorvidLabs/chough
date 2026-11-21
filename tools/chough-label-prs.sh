#!/usr/bin/env bash
# Label one pull request with the Chough main stage its introduced sessions share.
#
# Usage: chough-label-prs.sh <PR_NUMBER> [ROOT]
#
# ROOT is the pull request checkout. The default is the current directory.
# Copy this file to tools/chough-label-prs.sh. The workflow calls it there.
#
# A session's pr: field is not used. The script reads stage: from the session
# files this pull request introduces, compared with the merge base of its base
# branch. One distinct main stage applies chough-stage:<stage>. Zero, several,
# or a stage outside the main line applies no stage label. Stale chough-stage
# labels are removed either way.
#
# Main stages, in order: intent design plan build test accept done.
# Requires: gh, bash, awk, git.
set -euo pipefail

PR="${1:?PR number required}"
ROOT="${2:-$(pwd)}"
cd "$ROOT"

main_stage() {
  case "$1" in
    intent|design|plan|build|test|accept|done) return 0 ;;
    *) return 1 ;;
  esac
}

base=""
base_name="$(gh pr view "$PR" --json baseRefName -q .baseRefName 2>/dev/null || true)"
refs=()
if [[ -n "$base_name" ]]; then
  refs+=("origin/$base_name" "$base_name")
fi
refs+=(origin/main main)
for ref in "${refs[@]}"; do
  base="$(git merge-base "$ref" HEAD 2>/dev/null || true)"
  [[ -n "$base" ]] && break
done
if [[ -z "$base" ]]; then
  echo "labeled PR #$PR: stage=none (could not determine merge-base)"
  exit 0
fi

changed_files="$(git diff --name-only "$base" HEAD -- \
  '.chough/sessions/' '.chough/archive/sessions/' 2>/dev/null || true)"

map_file="$(mktemp)"
trap 'rm -f "$map_file"' EXIT

while IFS= read -r f; do
  [[ -z "$f" ]] && continue
  case "$f" in
    .chough/sessions/*/session.md|.chough/archive/sessions/*.md) ;;
    *) continue ;;
  esac
  [[ -e "$f" ]] || continue
  stage="$(awk -F': ' '
    /^stage:/ {
      v=$2; gsub(/^"/, "", v); gsub(/"$/, "", v); print v; exit
    }
  ' "$f")"
  [[ -z "$stage" || "$stage" == '""' ]] && continue
  printf '%s\n' "$stage" >> "$map_file"
done <<< "$changed_files"

distinct="$(sort -u "$map_file" | grep -v '^$' || true)"
stage=""
reason=""
if [[ -n "$distinct" ]]; then
  count="$(printf '%s\n' "$distinct" | grep -c .)"
  if [[ "$count" -eq 1 ]]; then
    stage="$(printf '%s\n' "$distinct" | head -n 1)"
    if ! main_stage "$stage"; then
      reason="stage '$stage' is not a main stage"
      stage=""
    fi
  else
    reason="multiple distinct stages on this PR: $(printf '%s' "$distinct" | tr '\n' ' ')"
  fi
fi

current_labels="$(gh pr view "$PR" --json labels -q '.labels[].name' 2>/dev/null || true)"
to_remove=()
while IFS= read -r lbl; do
  case "$lbl" in
    chough-stage:*) to_remove+=("$lbl") ;;
  esac
done <<< "$current_labels"

if [[ ${#to_remove[@]} -gt 0 ]]; then
  for lbl in "${to_remove[@]}"; do
    gh pr edit "$PR" --remove-label "$lbl" >/dev/null
  done
fi

if [[ -n "$stage" ]]; then
  gh pr edit "$PR" --add-label "chough-stage:$stage" >/dev/null
  echo "labeled PR #$PR: stage=$stage"
elif [[ -n "$reason" ]]; then
  echo "labeled PR #$PR: stage=none ($reason)"
else
  echo "labeled PR #$PR: stage=none"
fi
