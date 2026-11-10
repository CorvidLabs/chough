---
change: record-the-acting-agent-model-and-hc-terminal-on-each-transition-evidence-entry-and-bug-201
artifact: design
---

# Design

`src/actor.rs` holds `Actor {agent?, model?, terminal?}` (serde, deny_unknown_fields, every field skipped when absent) and `current()`, which reads the variables once per record and returns None when nothing is known. `Transition`, `Evidence` and `Bug` gain `actor: Option<Actor>` with `skip_serializing_if = "Option::is_none"`, so records written without an identity keep their exact bytes and older records read unchanged. Bug IDs come from an explicit creation payload that does not include the actor. A direct-native-write event carries the actor of the stage move that recorded it. The next view adds `last_actor` (from the latest transition) and per-observation `recorded_by`; status adds `last_actor` in its checkpoint section and already shows each observation record.

Compatibility: older builds deserialize these records with deny_unknown_fields and refuse a record holding `actor`. Mixed-version teams must upgrade every agent's Chough before one records an actor; the CHANGELOG names the first release that reads it.
