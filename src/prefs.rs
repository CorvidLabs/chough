//! Standing project preferences and run-only consent, kept in separate records.

use crate::checkpoint::{Checkpoint, Consent, read_checkpoint, save_checkpoint};
use crate::records::{
    ensure_text, local_path, now, print_json, read_record, valid_name, write_replace,
};
use anyhow::{Context, Result, bail, ensure};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;

pub(super) const PREFERENCES: &str = ".chough/preferences.json";

/// Key, allowed values (empty means a validated name), default.
const KEYS: &[(&str, &[&str], &str)] = &[
    ("repeat-validation", &["off", "ask", "always"], "ask"),
    ("monitoring", &["off", "ask", "always"], "ask"),
    ("release", &["on-request", "standing"], "on-request"),
    (
        "monitor-initiation",
        &["capture-only", "propose-change"],
        "capture-only",
    ),
    ("librarian", &["off", "on"], "off"),
    ("library-dir", &[], "library"),
    ("required-lane", &[], ""),
    ("retention", &["off", "auto"], "off"),
];
const CONSENT_KEYS: [&str; 4] = ["repeat-validation", "monitoring", "release", "retention"];

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Preference {
    key: String,
    value: String,
    source: String,
    scope: String,
    set_at_unix_seconds: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Preferences {
    schema_version: u32,
    preferences: Vec<Preference>,
}

#[derive(Subcommand)]
pub(super) enum PrefCommand {
    /// Show effective preferences and where each value came from.
    Show,
    /// Record a standing project preference stated by a human.
    Set {
        key: String,
        value: String,
        /// The human statement or reference this preference comes from.
        #[arg(long)]
        source: String,
    },
    /// Remove a standing preference so the default applies again.
    Unset {
        key: String,
        #[arg(long)]
        source: String,
    },
}

#[derive(Subcommand)]
pub(super) enum ConsentCommand {
    /// Show run consents for the current objective.
    Show,
    /// Record consent for this objective only; it never becomes a standing preference.
    Grant {
        key: String,
        #[arg(long)]
        source: String,
    },
    /// Withdraw a run consent.
    Revoke { key: String },
}

fn key_definition(key: &str) -> Result<(&'static [&'static str], &'static str)> {
    KEYS.iter()
        .find(|(name, _, _)| *name == key)
        .map(|(_, values, default)| (*values, *default))
        .with_context(|| {
            format!(
                "unknown preference {key}; known: {}",
                KEYS.iter()
                    .map(|(name, _, _)| *name)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

fn read_preferences(root: &Path) -> Result<Preferences> {
    let path = local_path(root, PREFERENCES)?;
    if !path.exists() {
        return Ok(Preferences {
            schema_version: 1,
            preferences: vec![],
        });
    }
    let preferences: Preferences =
        serde_json::from_str(&read_record(&path)?).context("invalid preferences record")?;
    ensure!(
        preferences.schema_version == 1,
        "unsupported preferences schema_version"
    );
    for preference in &preferences.preferences {
        validate_value(&preference.key, &preference.value)?;
        ensure!(
            preference.scope == "project",
            "preference scope must be project"
        );
    }
    Ok(preferences)
}

fn validate_value(key: &str, value: &str) -> Result<()> {
    let (values, _) = key_definition(key)?;
    if values.is_empty() {
        ensure!(valid_name(value), "{key} needs a simple name value");
    } else {
        ensure!(
            values.contains(&value),
            "{key} accepts {}",
            values.join(", ")
        );
    }
    Ok(())
}

/// The effective value and its origin for a key.
pub(super) fn effective(root: &Path, key: &str) -> Result<(String, String)> {
    let (_, default) = key_definition(key)?;
    Ok(read_preferences(root)?
        .preferences
        .into_iter()
        .find(|preference| preference.key == key)
        .map_or((default.to_string(), "default".into()), |preference| {
            (
                preference.value,
                format!("standing preference: {}", preference.source),
            )
        }))
}

/// Why an optional track may run now, or None when nobody authorized it.
pub(super) fn authorized(root: &Path, session: &Checkpoint, key: &str) -> Result<Option<String>> {
    if let Some(consent) = session
        .run_consents
        .iter()
        .find(|consent| consent.key == key && consent.objective == session.objective)
    {
        return Ok(Some(format!("run consent: {}", consent.source)));
    }
    let (value, origin) = effective(root, key)?;
    Ok(matches!(value.as_str(), "always" | "standing" | "auto").then_some(origin))
}

pub(super) fn pref(root: &Path, command: PrefCommand) -> Result<()> {
    let mut preferences = read_preferences(root)?;
    match command {
        PrefCommand::Show => {}
        PrefCommand::Set { key, value, source } => {
            ensure_text("source", &source)?;
            validate_value(&key, &value)?;
            preferences
                .preferences
                .retain(|preference| preference.key != key);
            preferences.preferences.push(Preference {
                key,
                value,
                source,
                scope: "project".into(),
                set_at_unix_seconds: now()?,
            });
            preferences
                .preferences
                .sort_by(|left, right| left.key.cmp(&right.key));
            write_replace(
                root,
                PREFERENCES,
                &(serde_json::to_string_pretty(&preferences)? + "\n"),
            )?;
        }
        PrefCommand::Unset { key, source } => {
            ensure_text("source", &source)?;
            key_definition(&key)?;
            let before = preferences.preferences.len();
            preferences
                .preferences
                .retain(|preference| preference.key != key);
            ensure!(
                preferences.preferences.len() < before,
                "{key} has no standing preference"
            );
            write_replace(
                root,
                PREFERENCES,
                &(serde_json::to_string_pretty(&preferences)? + "\n"),
            )?;
        }
    }
    let mut effective_values = serde_json::Map::new();
    for (key, _, _) in KEYS {
        let (value, origin) = effective(root, key)?;
        effective_values.insert((*key).into(), json!({"value": value, "origin": origin}));
    }
    print_json(&json!({
        "schema_version": 1,
        "effective": effective_values,
        "note": "Standing preferences apply to this project until a human changes them; run consents live in the checkpoint and end with the objective.",
    }))
}

pub(super) fn consent(root: &Path, command: ConsentCommand) -> Result<()> {
    let mut session = read_checkpoint(root)?;
    match command {
        ConsentCommand::Show => {}
        ConsentCommand::Grant { key, source } => {
            ensure!(
                CONSENT_KEYS.contains(&key.as_str()),
                "run consent applies to {}",
                CONSENT_KEYS.join(", ")
            );
            ensure_text("source", &source)?;
            session.run_consents.retain(|consent| consent.key != key);
            session.run_consents.push(Consent {
                key,
                source,
                objective: session.objective.clone(),
                granted_at_unix_seconds: now()?,
            });
            save_checkpoint(root, &session)?;
        }
        ConsentCommand::Revoke { key } => {
            let before = session.run_consents.len();
            session.run_consents.retain(|consent| consent.key != key);
            if session.run_consents.len() == before {
                bail!("no run consent for {key}");
            }
            save_checkpoint(root, &session)?;
        }
    }
    print_json(&json!({
        "schema_version": 1,
        "objective": session.objective,
        "run_consents": session.run_consents,
    }))
}
