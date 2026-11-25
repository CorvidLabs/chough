#!/usr/bin/env bash
# REQ-cli-072: the bundled chough-workflow skill routes every lifecycle writer
# through chough. Fenced `specsync change` commands are allowed only in the four
# unwrapped contexts (review, adopt, finalize, ship-status), each labeled, plus the
# wrapped-direct approve fence on standing authority. Fails naming file + line.
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
while IFS=: read -r file line text; do
  [ -z "$file" ] && continue
  start=$(( line > 3 ? line - 3 : 1 ))
  ctx=$(sed -n "${start},${line}p" "$file")
  sub=$(sed -nE 's/.*specsync change ([a-z-]+).*/\1/p' <<< "$text")
  case "$sub" in
    review|adopt|finalize|ship-status)
      grep -qE "Unwrapped step|unwrapped" <<< "$ctx" || { echo "FAIL $file:$line — unlabeled unwrapped-step fence: $text"; fail=1; } ;;
    approve)
      grep -q "WRAPPED step driven directly" <<< "$ctx" || { echo "FAIL $file:$line — unlabeled wrapped-direct fence: $text"; fail=1; } ;;
    *) echo "FAIL $file:$line — fenced 'specsync change $sub' has no unwrapped/labelled context: $text"; fail=1 ;;
  esac
done < <(awk '
  /^```/ { infence = !infence; next }
  infence && /specsync change/ { print FILENAME ":" FNR ":" $0 }
' $(find skills/chough-workflow .agents/skills/chough-workflow -name '*.md'))
if [ "$fail" -eq 0 ]; then
  echo "chough-first check: OK"
fi
exit $fail
