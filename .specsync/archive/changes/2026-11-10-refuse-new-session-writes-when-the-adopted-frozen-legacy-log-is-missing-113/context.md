---
change: refuse-new-session-writes-when-the-adopted-frozen-legacy-log-is-missing-113
artifact: context
---

# Context

## Problem and current source

Leif's open Chough113 reports a new session appending after its project's adopted frozen legacy log is deleted. Current main4bbd5a6373d26c0ad069878f8f543842a4564a6f still calls adopted_anchor only for (None, Some(current)). anchor returns None when transitions.jsonl is absent; a new session intentionally has cursor.legacy None. Consequently the inspected branch can compare None to None without looking for remembered adoption proof. Existing REQ-history-002 says unexpected anchor changes refuse new writes; the history contract error table says anchor mismatch refuses. REQ-history-006/DRIVE-3 keeps sessions independent. This is a repair of that existing guard, with no canonical wording/schema/CLI or permanent HI/REQ ID change.

## Authority and ownership

Actor t221 prepares only under coordinator t19f's explicit113 queued allocation relaying Leif's request to work through all Chough issues/PRs. This records the relay source, not invented human words or implementation acceptance. Root retains actual scope receipt, implementation scheduling, genuine human review/acceptance and delivery. No production edit before actual root receipt; no compile/full/site until a separate root heavy lease. One isolated tree at current main, one lifecycle writer. Other196/eval/de31/153/147d/dirty187132/60 sources, gates and archives stay untouched.

## What actually ran

Live Fledge GitHub issue/PR and read-only main discovery: issue113 open,14 issues,0 PRs, main4bbd. Private eight-case CLI matrix on immutable older INT01 executable ef3289fefad932c05b8c0da0274990cc516fb6d701549f242c86e490a6942667: missing log/new session with open holder and with archived holder each returned0 and published a new event plus changed checkpoint. Original adopter and changed-byte controls refused1 without checkpoint/history mutation; intact, restored and never-had-log controls succeeded0. The same contract oracle returned1 for each missing-log witness and0 for six controls.

This executable is NOT a qualified current-main build. Selected history.rs and sessions.rs bytes equal retained INT01 source8a37, but five other production files differ; whole-source equivalence is false. Root supplied no qualified main artifact and explicitly disallowed compilation while another full lane holds the slot. Current-main runtime and the proposed Rust regression are NOT RUN. No product bug/failed observation is written as current-main execution. One unused archive-byte guard in the external probe was always true and is explicitly uncredited; a separate actual equality check confirms archived holder adoption cursor, not uncaptured pre-rename full bytes.
