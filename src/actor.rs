//! The acting agent of a record (REQ-records-007): who wrote a Timeline entry, an
//! evidence entry or a bug, as far as the environment says. It is an unauthenticated
//! claim and never stands in for a human decision (FLOW-16).

use crate::leaks::{Local, find};
use crate::markdown::Value as FrontValue;
use serde::{Deserialize, Serialize};

/// Set by the operator or the agent host: the agent's name.
pub(crate) const AGENT: &str = "CHOUGH_ACTOR";
/// The agent's model.
pub(crate) const MODEL: &str = "CHOUGH_ACTOR_MODEL";
/// The hc terminal, such as `t370`.
pub(crate) const TERMINAL: &str = "CHOUGH_HC_TERMINAL";
/// Set by Claude Code in the processes it starts.
pub(crate) const CLAUDE_CODE: &str = "CLAUDECODE";
/// Set by hc in the terminals it runs.
pub(crate) const HC_TERMINAL: &str = "HC_TERMINAL_ID";
/// How a `by` field names an unknown agent.
const UNKNOWN_AGENT: &str = "unknown agent";
/// What precedes the terminal in a `by` field.
const HC: &str = "hc";
/// The longest agent or model value kept.
const VALUE_LIMIT: usize = 80;
/// The longest terminal name kept.
const TERMINAL_LIMIT: usize = 64;

/// The acting agent: each part only when known.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Actor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) terminal: Option<String>,
}

impl Actor {
    /// The actor this process runs as, or None when nothing is known. A value that
    /// fails the leak check, or a terminal that is not a plain hc name, is left out
    /// with a warning on standard error; neither fails the command.
    pub(crate) fn current() -> Option<Actor> {
        Self::from_lookup(
            Local::current(),
            |name| std::env::var(name).ok(),
            |warning| {
                eprintln!("warning: {warning}");
            },
        )
    }

    /// The actor from `lookup`, checked against `local`; each value left out is
    /// reported through `warn`.
    pub(crate) fn from_lookup(
        local: &Local,
        lookup: impl Fn(&str) -> Option<String>,
        mut warn: impl FnMut(String),
    ) -> Option<Actor> {
        let set = |name: &str| lookup(name).filter(|value| !value.trim().is_empty());
        let agent = set(AGENT).or_else(|| set(CLAUDE_CODE).map(|_| "claude-code".to_string()));
        let model = set(MODEL);
        let terminal = set(TERMINAL).or_else(|| set(HC_TERMINAL));
        let mut keep = |field: &str, value: Option<String>, limit: usize| {
            let value = value?.trim().to_string();
            if value.len() > limit || value.chars().any(char::is_control) {
                warn(format!(
                    "the actor's {field} is longer than {limit} bytes or holds a control character, so it is left out"
                ));
                return None;
            }
            if let Some(found) = find(local, &value).first() {
                warn(format!(
                    "the actor's {field} holds {}, so it is left out",
                    found.kind.name()
                ));
                return None;
            }
            Some(value)
        };
        let agent = keep("agent", agent, VALUE_LIMIT);
        let model = keep("model", model, VALUE_LIMIT);
        let terminal = keep("terminal", terminal, TERMINAL_LIMIT).filter(|terminal| {
            // An hc terminal address or name: never a path, a host or a network address.
            let plain = terminal
                .chars()
                .all(|char| char.is_ascii_alphanumeric() || char == '-' || char == '_');
            if !plain {
                warn(
                    "the actor's terminal is not a plain hc terminal name (letters, digits, - and _), so it is left out"
                        .to_string(),
                );
            }
            plain
        });
        let actor = Actor {
            agent,
            model,
            terminal,
        };
        (actor != Actor::default()).then_some(actor)
    }

    // Plan steps 4, 5 and 6 (evidence, bugs, native approval) call it.
    #[cfg_attr(not(test), allow(dead_code))]
    /// The agent account a native step records when an agent acts on authority given
    /// in advance (REQ-cli-029): `<agent>-<terminal>`, or None without both.
    pub(crate) fn account(&self) -> Option<String> {
        Some(format!(
            "{}-{}",
            self.agent.as_deref()?,
            self.terminal.as_deref()?
        ))
    }

    /// The actor as a Timeline entry's `by` field and as one line for people
    /// (REQ-history-002): `claude-code (claude-opus-5-5, hc t382)`, leaving out what is
    /// unknown; an unknown agent reads `unknown agent`.
    pub(crate) fn display(&self) -> String {
        let mut text = self
            .agent
            .clone()
            .unwrap_or_else(|| UNKNOWN_AGENT.to_string());
        let mut known = vec![];
        if let Some(model) = &self.model {
            known.push(model.clone());
        }
        if let Some(terminal) = &self.terminal {
            known.push(format!("{HC} {terminal}"));
        }
        if !known.is_empty() {
            text.push_str(&format!(" ({})", known.join(", ")));
        }
        text
    }

    /// The actor a `by` field gives, as [`Actor::display`] writes it; None when it
    /// names nothing.
    pub(crate) fn from_by(text: &str) -> Option<Actor> {
        let text = text.trim();
        let (agent, inside) = match text
            .strip_suffix(')')
            .and_then(|rest| rest.rsplit_once(" ("))
        {
            Some((agent, inside)) => (agent, Some(inside)),
            None => (text, None),
        };
        let mut actor = Actor {
            agent: (!agent.is_empty() && agent != UNKNOWN_AGENT).then(|| agent.to_string()),
            ..Actor::default()
        };
        for part in inside.into_iter().flat_map(|inside| inside.split(", ")) {
            match part.strip_prefix(&format!("{HC} ")) {
                Some(terminal) => actor.terminal = Some(terminal.to_string()),
                None => actor.model = Some(part.to_string()),
            }
        }
        (actor != Actor::default()).then_some(actor)
    }

    // Plan steps 4, 5 and 6 (evidence, bugs, native approval) call it.
    #[cfg_attr(not(test), allow(dead_code))]
    /// The front matter value records write under `actor`: one map of the known parts.
    pub(crate) fn front_value(&self) -> FrontValue {
        let mut map = vec![];
        for (key, value) in [
            ("agent", &self.agent),
            ("model", &self.model),
            ("terminal", &self.terminal),
        ] {
            if let Some(value) = value {
                map.push((key.to_string(), value.clone()));
            }
        }
        FrontValue::Maps(vec![map])
    }

    // Plan steps 4, 5 and 6 (evidence, bugs, native approval) call it.
    #[cfg_attr(not(test), allow(dead_code))]
    /// The actor a record's front matter `actor` holds, when it is one well-formed map.
    pub(crate) fn from_front(value: &FrontValue) -> Option<Actor> {
        let FrontValue::Maps(maps) = value else {
            return None;
        };
        let [map] = maps.as_slice() else {
            return None;
        };
        let mut actor = Actor::default();
        for (key, value) in map {
            let slot = match key.as_str() {
                "agent" => &mut actor.agent,
                "model" => &mut actor.model,
                "terminal" => &mut actor.terminal,
                _ => return None,
            };
            *slot = Some(value.clone());
        }
        (actor != Actor::default()).then_some(actor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn actor(pairs: &[(&str, &str)]) -> (Option<Actor>, Vec<String>) {
        let environment: BTreeMap<String, String> = pairs
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        let local = Local::new(Some("SYNTHETIC-user"), Some("SYNTHETIC-host"));
        let mut warnings = vec![];
        let found = Actor::from_lookup(
            &local,
            |name| environment.get(name).cloned(),
            |warning| warnings.push(warning),
        );
        (found, warnings)
    }

    #[test]
    fn overrides_win_and_claude_code_and_hc_fill_in() {
        let (found, _) = actor(&[
            (AGENT, "kimi-code"),
            (MODEL, "kimi-k3"),
            (TERMINAL, "t37b"),
            (CLAUDE_CODE, "1"),
            (HC_TERMINAL, "t99"),
        ]);
        let found = found.unwrap();
        assert_eq!(found.account().as_deref(), Some("kimi-code-t37b"));
        assert_eq!(found.display(), "kimi-code (kimi-k3, hc t37b)");
        assert_eq!(Actor::from_by(&found.display()), Some(found.clone()));
        let (found, _) = actor(&[(CLAUDE_CODE, "1"), (HC_TERMINAL, "t382")]);
        let found = found.unwrap();
        assert_eq!(found.agent.as_deref(), Some("claude-code"));
        assert_eq!(found.model, None);
        assert_eq!(found.account().as_deref(), Some("claude-code-t382"));
    }

    #[test]
    fn the_by_field_round_trips_whatever_is_known() {
        for actor in [
            Actor {
                agent: Some("claude-code".into()),
                model: None,
                terminal: Some("t382".into()),
            },
            Actor {
                agent: Some("claude-code".into()),
                model: Some("m-1".into()),
                terminal: None,
            },
            Actor {
                agent: None,
                model: None,
                terminal: Some("t1".into()),
            },
            Actor {
                agent: Some("solo".into()),
                model: None,
                terminal: None,
            },
        ] {
            let by = actor.display();
            assert_eq!(Actor::from_by(&by), Some(actor), "{by}");
        }
        assert_eq!(
            Actor::from_by("claude-code (hc t382)")
                .unwrap()
                .terminal
                .as_deref(),
            Some("t382")
        );
        assert_eq!(Actor::from_by("  "), None);
    }

    #[test]
    fn nothing_known_is_no_actor() {
        assert_eq!(actor(&[]), (None, vec![]));
        assert_eq!(actor(&[(AGENT, "  ")]), (None, vec![]));
    }

    #[test]
    fn a_leaking_or_odd_value_is_left_out_with_a_warning() {
        let (found, warnings) = actor(&[(CLAUDE_CODE, "1"), (HC_TERMINAL, "/Users/someone/t1")]);
        assert_eq!(found.unwrap().terminal, None);
        assert!(
            warnings[0].contains("terminal holds home-path"),
            "{warnings:?}"
        );
        for terminal in ["10.0.0.7", "machine/t7", "SYNTHETIC-host"] {
            let (found, warnings) = actor(&[(CLAUDE_CODE, "1"), (TERMINAL, terminal)]);
            assert_eq!(found.unwrap().terminal, None, "{terminal}");
            assert_eq!(warnings.len(), 1, "{terminal}: {warnings:?}");
        }
        let (found, warnings) = actor(&[(AGENT, "someone@example.com")]);
        assert_eq!(found, None);
        assert!(warnings[0].contains("agent holds email"), "{warnings:?}");
    }

    #[test]
    fn the_front_matter_value_round_trips() {
        let (found, _) = actor(&[(AGENT, "claude-code"), (TERMINAL, "t382")]);
        let found = found.unwrap();
        let value = found.front_value();
        assert_eq!(
            value,
            FrontValue::Maps(vec![vec![
                ("agent".to_string(), "claude-code".to_string()),
                ("terminal".to_string(), "t382".to_string())
            ]])
        );
        assert_eq!(Actor::from_front(&value), Some(found));
        assert_eq!(Actor::from_front(&FrontValue::Scalar("x".into())), None);
    }
}
