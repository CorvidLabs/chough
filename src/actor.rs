//! The acting agent's identity on each transition, evidence entry and bug (#201,
//! LIFE-17): recorded automatically where the environment exposes it, overridable by
//! environment variables, and never a command failure when the identity is unknown.
//! The identity is a claim by the recording process, not authenticated.

use serde::{Deserialize, Serialize};

/// Overrides the detected agent name, for example `kimi-code` or `codex`.
const AGENT_VARIABLE: &str = "CHOUGH_ACTOR";
/// Overrides the detected model.
const MODEL_VARIABLE: &str = "CHOUGH_ACTOR_MODEL";
/// Overrides the detected hc terminal.
const TERMINAL_VARIABLE: &str = "CHOUGH_HC_TERMINAL";
/// Set by hc in every terminal it runs (observed with hc on macOS, 2026-11-10).
const HC_TERMINAL: &str = "HC_TERMINAL_ID";
/// Set by Claude Code for the commands it runs (observed in Claude Code, 2026-11-10).
const CLAUDE_CODE: &str = "CLAUDECODE";
/// The longest value kept from any one variable, in bytes.
const VALUE_LIMIT: usize = 128;

/// Who recorded a transition, an evidence entry or a bug.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct Actor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) terminal: Option<String>,
}

impl Actor {
    /// One display line: the agent, then the model and hc terminal when known.
    pub(super) fn display(&self) -> String {
        let mut text = self.agent.clone().unwrap_or_else(|| "unknown agent".into());
        if let Some(model) = &self.model {
            text.push_str(&format!(" ({model})"));
        }
        if let Some(terminal) = &self.terminal {
            text.push_str(&format!(" on hc {terminal}"));
        }
        text
    }
}

/// One trimmed, bounded, single-line value of `name`, or None when unset or blank.
fn variable(name: &str) -> Option<String> {
    let value = std::env::var(name).ok()?;
    let value = value.trim();
    if value.is_empty() || value.chars().any(char::is_control) {
        return None;
    }
    let mut end = value.len().min(VALUE_LIMIT);
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    Some(value[..end].to_string())
}

/// The acting agent, model and hc terminal of this process, or None when nothing is
/// known. `CHOUGH_ACTOR`, `CHOUGH_ACTOR_MODEL` and `CHOUGH_HC_TERMINAL` take
/// precedence; otherwise Claude Code is recognized by `CLAUDECODE` and the hc terminal
/// by `HC_TERMINAL_ID`. Other hosts are named through `CHOUGH_ACTOR`, for example in
/// their hc templates. Never fails.
pub(super) fn current() -> Option<Actor> {
    let agent = variable(AGENT_VARIABLE)
        .or_else(|| variable(CLAUDE_CODE).map(|_| "claude-code".to_string()));
    let model = variable(MODEL_VARIABLE);
    let terminal = variable(TERMINAL_VARIABLE).or_else(|| variable(HC_TERMINAL));
    let actor = Actor {
        agent,
        model,
        terminal,
    };
    (actor.agent.is_some() || actor.model.is_some() || actor.terminal.is_some()).then_some(actor)
}
