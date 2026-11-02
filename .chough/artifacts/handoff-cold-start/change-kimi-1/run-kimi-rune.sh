#!/bin/sh
# Change run of the handoff cold start (TC-cli-0133), rubric sha256 f9083521...
# Seeded repo HEAD c1b7e74b, tree 9be61b11; chough binary and skill from the implementation tip 9009d9a (sha256 3d9cf60b...).
# Drives interactive Kimi Code CLI 0.41.0 (--auto, model kimi-code/k3.1-pro-highspeed) through
# CorvidLabs/rune 0.9.0 in the disposable seeded repo only. The only input sent is the prompt.
R=/private/tmp/claude-501/-Users-leif-Development--CorvidLabs-chough/08d48ab4-faba-46d5-97eb-7e943011e4d4/scratchpad/coldstart-change
N=chough-coldstart-change
cd "$R/repo" || exit 1
export PATH="$R/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/leif/.kimi-code/bin"
date -u +%FT%TZ > "$R/started.txt"
rune session start --name "$N" -- kimi --auto -m kimi-code/k3.1-pro-highspeed > "$R/rune-start.json"
# A new folder first shows Kimi's "Trust this folder?" dialog. The baseline folder was
# already trusted (in its not-run attempt 2), so accept the default "Trust and continue"
# with Enter here. This is host setup, not a message to the agent.
# Wait until Kimi's input box and status line are on screen (up to 2 minutes).
i=0
until rune session read --name "$N" --screen --json 2>/dev/null | grep -q 'context: 0%'; do
  if rune session read --name "$N" --screen --json 2>/dev/null | grep -q 'Trust this folder'; then
    echo "Accepting Kimi's folder trust dialog" >> "$R/started.txt"
    rune session send --name "$N" --no-wait -- "" > "$R/rune-trust.json"
    sleep 3
  fi
  i=$((i+1)); [ $i -gt 60 ] && { echo "Kimi did not become ready"; rune session stop --name "$N"; exit 1; }
  sleep 2
done
sleep 3
# Send the only prompt; return once Kimi has been quiet for 3 minutes, or after 45 minutes.
rune session send --name "$N" --settle-ms=180000 --timeout-ms=2700000 --screen -- "Resume this project." > "$R/rune-send.json"
# Confirm the prompt was taken (a session exists and context is no longer 0%).
rune session read --name "$N" --screen --json > "$R/rune-after-send.json"
rune session read --name "$N" --screen > "$R/rune-transcript.json"
date -u +%FT%TZ >> "$R/started.txt"
rune session stop --name "$N" > "$R/rune-stop.json"
# Kimi's own session record holds every tool call (the command log).
kimi export -y -o "$R/kimi-session.zip" > "$R/kimi-export.txt" 2>&1
git -C "$R/repo" status --short
ls -la "$R"
