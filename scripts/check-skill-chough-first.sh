#!/usr/bin/env bash
# REQ-cli-072: the bundled chough-workflow skill routes every lifecycle writer
# through chough. Fenced `specsync change` commands are allowed only in the four
# unwrapped contexts, each labeled. Fails naming file + line otherwise.
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
while IFS=: read -r file line text; do
  [ -z "$file" ] && continue
  start=$(( line > 3 ? line - 3 : 1 ))
  ctx=$(sed -n "${start},${line}p" "$file")
  if ! grep -q "Unwrapped step\|unwrapped" <<< "$ctx"; then
    echo "FAIL $file:$line — fenced 'specsync change' outside an unwrapped-step context:"
    echo "  $text"
    fail=1
  fi
done < <(awk '
  /^```/ { infence = !infence; next }
  infence && /specsync change/ { print FILENAME ":" FNR ":" $0 }
' $(find skills/chough-workflow -name '*.md'))
if [ "$fail" -eq 0 ]; then
  echo "chough-first check: OK (every fenced specsync change is a labeled unwrapped step)"
fi
exit $fail
