//! Chough's record files: one Markdown file per record, starting with front matter in
//! a strict subset of YAML (REQ-records-001).
//!
//! The subset is `key: value` scalars, plain or double-quoted with `\"`, `\\` and `\n`
//! escapes; flow lists `[a, b]`; block lists of scalars; and block lists of flat maps of
//! scalars. Anything else is refused with the file, the line and the rule. Reading is
//! tolerant: a key Chough does not know is kept byte for byte, in place, when Chough
//! rewrites the front matter; blank and comment lines stay where they are; CRLF reads as
//! LF. Chough owns the front matter and the sections it appends to; every other body byte
//! belongs to whoever wrote it.
// Plan step 3 wires the record layer into sessions and removes this allowance.
#![allow(dead_code)]

use anyhow::{Result, bail, ensure};
use std::path::Path;

/// The bound of every record kind not named below.
pub(crate) const RECORD_BOUND: u64 = 65_536;
/// The bound of a session record and of an archived session story.
pub(crate) const SESSION_BOUND: u64 = 1_048_576;
/// The bound of a migration receipt.
pub(crate) const RECEIPT_BOUND: u64 = 4_194_304;

/// The record kinds Chough writes, each with its byte bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Session,
    Story,
    Evidence,
    Bug,
    Inbox,
    Journal,
    Receipt,
}

impl Kind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Kind::Session => "session",
            Kind::Story => "session story",
            Kind::Evidence => "evidence entry",
            Kind::Bug => "bug",
            Kind::Inbox => "inbox entry",
            Kind::Journal => "journal post",
            Kind::Receipt => "migration receipt",
        }
    }

    pub(crate) fn bound(self) -> u64 {
        match self {
            Kind::Session | Kind::Story => SESSION_BOUND,
            Kind::Receipt => RECEIPT_BOUND,
            _ => RECORD_BOUND,
        }
    }
}

/// A front matter value: everything is text to Chough.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Value {
    Scalar(String),
    List(Vec<String>),
    Maps(Vec<Vec<(String, String)>>),
}

#[derive(Clone, Debug)]
enum Item {
    /// A key, its value and, while unchanged, the exact text it was read from.
    Entry {
        key: String,
        value: Value,
        raw: Option<String>,
    },
    /// A blank or comment line, kept where it was.
    Raw(String),
}

/// One record: its front matter and its Markdown body.
#[derive(Clone, Debug)]
pub(crate) struct Record {
    items: Vec<Item>,
    body: String,
}

impl Record {
    /// A new record with no front matter keys and this body.
    pub(crate) fn new(body: &str) -> Self {
        Self {
            items: vec![],
            body: body.to_string(),
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<&Value> {
        self.items.iter().find_map(|item| match item {
            Item::Entry {
                key: name, value, ..
            } if name == key => Some(value),
            _ => None,
        })
    }

    /// A scalar value; a missing key is `None`.
    pub(crate) fn scalar(&self, key: &str) -> Option<&str> {
        match self.get(key) {
            Some(Value::Scalar(text)) => Some(text),
            _ => None,
        }
    }

    /// A list of scalars; a missing key or an empty value is an empty list.
    pub(crate) fn list(&self, key: &str) -> Vec<String> {
        match self.get(key) {
            Some(Value::List(items)) => items.clone(),
            Some(Value::Scalar(text)) if !text.is_empty() => vec![text.clone()],
            _ => vec![],
        }
    }

    /// A list of flat maps; a missing key is an empty list.
    pub(crate) fn maps(&self, key: &str) -> Vec<Vec<(String, String)>> {
        match self.get(key) {
            Some(Value::Maps(maps)) => maps.clone(),
            _ => vec![],
        }
    }

    pub(crate) fn keys(&self) -> Vec<&str> {
        self.items
            .iter()
            .filter_map(|item| match item {
                Item::Entry { key, .. } => Some(key.as_str()),
                Item::Raw(_) => None,
            })
            .collect()
    }

    /// Set a key, in place when it exists and at the end otherwise. Setting the value
    /// it already has keeps its exact text.
    pub(crate) fn set(&mut self, key: &str, value: Value) -> Result<()> {
        ensure!(
            valid_key(key),
            "front matter key {key:?} is outside the subset"
        );
        if let Value::Maps(maps) = &value {
            for map in maps {
                ensure!(
                    !map.is_empty(),
                    "a list item of `{key}` needs at least one key"
                );
                for (name, _) in map {
                    ensure!(valid_key(name), "map key {name:?} is outside the subset");
                }
            }
        }
        for item in &mut self.items {
            if let Item::Entry {
                key: name,
                value: current,
                raw,
            } = item
                && name == key
            {
                if *current != value {
                    *current = value;
                    *raw = None;
                }
                return Ok(());
            }
        }
        self.items.push(Item::Entry {
            key: key.to_string(),
            value,
            raw: None,
        });
        Ok(())
    }

    pub(crate) fn set_scalar(&mut self, key: &str, value: &str) -> Result<()> {
        self.set(key, Value::Scalar(value.to_string()))
    }

    pub(crate) fn remove(&mut self, key: &str) {
        self.items
            .retain(|item| !matches!(item, Item::Entry { key: name, .. } if name == key));
    }

    pub(crate) fn body(&self) -> &str {
        &self.body
    }

    /// The text of the section under `## <heading>`, up to the next heading of level one
    /// or two; `None` when there is no such section.
    pub(crate) fn section(&self, heading: &str) -> Option<&str> {
        let (start, end) = section_span(&self.body, heading)?;
        Some(&self.body[start..end])
    }

    /// Append `text` (whole lines) to the end of the `## <heading>` section, before the
    /// blank lines that close it, or create the section at the end of the body. Every
    /// other body byte is kept.
    pub(crate) fn append_to_section(&mut self, heading: &str, text: &str) -> Result<()> {
        ensure!(
            !heading.is_empty() && !heading.contains('\n'),
            "a section heading is one non-empty line"
        );
        let mut text = text.replace("\r\n", "\n");
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        match section_span(&self.body, heading) {
            Some((start, end)) => {
                let mut insert = end;
                let bytes = self.body.as_bytes();
                // Back over the blank lines that close the section.
                while insert > start && bytes[insert - 1] == b'\n' {
                    let line_start = self.body[..insert - 1].rfind('\n').map_or(0, |at| at + 1);
                    if line_start < start || !self.body[line_start..insert - 1].trim().is_empty() {
                        break;
                    }
                    insert = line_start;
                }
                let mut addition = String::new();
                if insert > 0 && !self.body[..insert].ends_with('\n') {
                    addition.push('\n');
                }
                // The first item of an empty section keeps a blank line after its heading.
                if insert == start {
                    addition.push('\n');
                }
                addition.push_str(&text);
                self.body.insert_str(insert, &addition);
            }
            None => {
                if !self.body.is_empty() {
                    if !self.body.ends_with('\n') {
                        self.body.push('\n');
                    }
                    if !self.body.ends_with("\n\n") {
                        self.body.push('\n');
                    }
                }
                self.body.push_str(&format!("## {heading}\n\n{text}"));
            }
        }
        Ok(())
    }

    /// Append `mark` to the line of the first item `- <item>` in the `## <heading>`
    /// section: the one in-place edit of a section Chough owns (REQ-records-004).
    pub(crate) fn mark_item(&mut self, heading: &str, item: &str, mark: &str) -> Result<()> {
        let (start, end) = section_span(&self.body, heading)
            .ok_or_else(|| anyhow::anyhow!("no `## {heading}` section"))?;
        let mut offset = start;
        for line in self.body[start..end].split_inclusive('\n') {
            let content = line.trim_end_matches('\n');
            if content.strip_prefix("- ") == Some(item) {
                self.body.insert_str(offset + content.len(), mark);
                return Ok(());
            }
            offset += line.len();
        }
        bail!("no item `{item}` under `## {heading}`")
    }

    /// The whole file: front matter between two `---` lines, then the body.
    pub(crate) fn render(&self) -> String {
        let mut out = String::from("---\n");
        for item in &self.items {
            match item {
                Item::Raw(line) => out.push_str(line),
                Item::Entry { raw: Some(raw), .. } => out.push_str(raw),
                Item::Entry { key, value, .. } => out.push_str(&render_entry(key, value)),
            }
        }
        out.push_str("---\n");
        out.push_str(&self.body);
        out
    }

    /// The rendered file, refused when it is over the bound of `kind`.
    pub(crate) fn render_within(&self, kind: Kind) -> Result<String> {
        let text = self.render();
        let bound = kind.bound();
        ensure!(
            text.len() as u64 <= bound,
            "refused: this {} would be {} bytes, over its bound of {bound} bytes; nothing was written",
            kind.name(),
            text.len()
        );
        Ok(text)
    }
}

impl Record {
    /// Every value this record would write, with the field it is in: front matter keys
    /// by name (`key`, `key[2]`, `key[0].name`) and the body as `body`.
    pub(crate) fn fields(&self) -> Vec<(String, String)> {
        let mut out = vec![];
        for item in &self.items {
            let Item::Entry { key, value, .. } = item else {
                continue;
            };
            match value {
                Value::Scalar(text) => out.push((key.clone(), text.clone())),
                Value::List(items) => {
                    for (index, text) in items.iter().enumerate() {
                        out.push((format!("{key}[{index}]"), text.clone()));
                    }
                }
                Value::Maps(maps) => {
                    for (index, map) in maps.iter().enumerate() {
                        for (name, text) in map {
                            out.push((format!("{key}[{index}].{name}"), text.clone()));
                        }
                    }
                }
            }
        }
        out.push(("body".to_string(), self.body.clone()));
        out
    }

    /// Refuse the write when any value leaks (REQ-records-006), naming the first field
    /// and its kinds, never the text.
    pub(crate) fn check_leaks(&self, local: &crate::leaks::Local) -> Result<()> {
        for (field, value) in self.fields() {
            crate::leaks::check(local, &field, &value)?;
        }
        Ok(())
    }

    /// Replace every leaking match with `<redacted:<kind>>`; returns how many.
    pub(crate) fn redact_leaks(&mut self, local: &crate::leaks::Local) -> usize {
        let mut count = 0;
        let mut redact = |text: &mut String| {
            let (redacted, found) = crate::leaks::redact(local, text);
            if found > 0 {
                *text = redacted;
                count += found;
            }
            found > 0
        };
        for item in &mut self.items {
            let Item::Entry { value, raw, .. } = item else {
                continue;
            };
            let mut changed = false;
            match value {
                Value::Scalar(text) => changed = redact(text),
                Value::List(items) => {
                    for text in items.iter_mut() {
                        changed |= redact(text);
                    }
                }
                Value::Maps(maps) => {
                    for (_, text) in maps.iter_mut().flatten() {
                        changed |= redact(text);
                    }
                }
            }
            if changed {
                *raw = None;
            }
        }
        redact(&mut self.body);
        count
    }
}

/// The record in `dir` whose `id` or `aliases` holds `id`, as its path and record;
/// `None` when no record holds it. An ID held by two records is refused, naming both
/// (REQ-records-003). Files that do not read as records are skipped here; `chough
/// doctor` reports them.
pub(crate) fn resolve(root: &Path, dir: &str, id: &str) -> Result<Option<(String, Record)>> {
    let folder = crate::records::local_path(root, dir)?;
    let entries = match std::fs::read_dir(&folder) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".md"))
        .collect();
    names.sort();
    let mut found: Vec<(String, Record)> = vec![];
    for name in names {
        let relative = format!("{dir}/{name}");
        let Ok(text) = crate::records::read_bounded_record(&folder.join(&name), RECEIPT_BOUND)
        else {
            continue;
        };
        let Ok(record) = parse(&relative, &text) else {
            continue;
        };
        if record.scalar("id") == Some(id) || record.list("aliases").iter().any(|alias| alias == id)
        {
            found.push((relative, record));
        }
    }
    match found.len() {
        0 | 1 => Ok(found.pop()),
        _ => bail!(
            "refused: {id} is the id or alias of more than one record ({}); `chough doctor` reports it",
            found
                .iter()
                .map(|(path, _)| path.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// Write a new record by create-new; false when the file already exists.
pub(crate) fn write_new(root: &Path, relative: &str, record: &Record, kind: Kind) -> Result<bool> {
    let text = record.render_within(kind)?;
    crate::records::write_new_bounded(root, relative, &text, kind.bound())
}

/// Replace a record atomically. Callers hold the writer lock (REQ-cli-040).
pub(crate) fn replace(root: &Path, relative: &str, record: &Record, kind: Kind) -> Result<()> {
    let text = record.render_within(kind)?;
    crate::records::write_replace_bounded(root, relative, &text, kind.bound())
}

/// Read a record file; `file` names it in every refusal.
pub(crate) fn parse(file: &str, text: &str) -> Result<Record> {
    let text = text.replace("\r\n", "\n");
    let mut lines = text.split_inclusive('\n');
    match lines.next() {
        Some("---\n") | Some("---") => {}
        _ => bail!("{file}:1: a record starts with a `---` line before its front matter"),
    }
    let mut front = vec![];
    let mut closed = false;
    let mut consumed = 4;
    for line in lines.by_ref() {
        consumed += line.len();
        if line == "---\n" || line == "---" {
            closed = true;
            break;
        }
        front.push(line);
    }
    ensure!(
        closed,
        "{file}:{}: the front matter has no closing `---` line",
        front.len() + 1
    );
    let body = text.get(consumed.min(text.len())..).unwrap_or_default();
    let items = parse_front(file, &front)?;
    Ok(Record {
        items,
        body: body.to_string(),
    })
}

fn parse_front(file: &str, lines: &[&str]) -> Result<Vec<Item>> {
    let mut items = vec![];
    let mut seen: Vec<String> = vec![];
    let mut index = 0;
    while index < lines.len() {
        let number = index + 2;
        let line = lines[index];
        let content = line.trim_end_matches('\n');
        if content.contains('\t') {
            bail!("{file}:{number}: a tab is outside the front matter subset; indent with spaces");
        }
        if content.trim().is_empty() || content.starts_with('#') {
            items.push(Item::Raw(line.to_string()));
            index += 1;
            continue;
        }
        if content.starts_with(' ') {
            bail!(
                "{file}:{number}: an indented line that is not a list item is a nested map, outside the subset"
            );
        }
        if content.starts_with('-') {
            bail!("{file}:{number}: a list item needs a key line above it");
        }
        let Some((key, rest)) = content.split_once(':') else {
            bail!("{file}:{number}: a front matter line is `key: value`");
        };
        ensure!(
            valid_key(key),
            "{file}:{number}: key {key:?} is outside the subset (letters, digits, `_`, `-` and `.`)"
        );
        ensure!(
            rest.is_empty() || rest.starts_with(' '),
            "{file}:{number}: a key is followed by `: ` and its value"
        );
        ensure!(
            !seen.iter().any(|name| name == key),
            "{file}:{number}: duplicate key `{key}`"
        );
        seen.push(key.to_string());
        let rest = rest.trim();
        let mut raw = line.to_string();
        index += 1;
        let value = if rest.is_empty() {
            let mut block = vec![];
            while index < lines.len() && lines[index].starts_with("  ") {
                block.push((index + 2, lines[index].trim_end_matches('\n')));
                raw.push_str(lines[index]);
                index += 1;
            }
            if block.is_empty() {
                Value::Scalar(String::new())
            } else {
                parse_block(file, &block)?
            }
        } else if rest.starts_with('[') {
            Value::List(parse_flow(file, number, rest)?)
        } else {
            Value::Scalar(parse_scalar(file, number, rest)?)
        };
        items.push(Item::Entry {
            key: key.to_string(),
            value,
            raw: Some(raw),
        });
    }
    Ok(items)
}

fn parse_block(file: &str, block: &[(usize, &str)]) -> Result<Value> {
    let mut scalars = vec![];
    let mut maps: Vec<Vec<(String, String)>> = vec![];
    for (number, line) in block {
        let number = *number;
        if let Some(item) = line.strip_prefix("  - ") {
            ensure!(
                !item.starts_with(' '),
                "{file}:{number}: a list item has one space after `-`"
            );
            if let Some((name, value)) = map_pair(item) {
                ensure!(
                    scalars.is_empty(),
                    "{file}:{number}: a list mixes scalars and maps, outside the subset"
                );
                let value = map_value(file, number, name, value)?;
                maps.push(vec![(name.to_string(), value)]);
            } else {
                ensure!(
                    maps.is_empty(),
                    "{file}:{number}: a list mixes scalars and maps, outside the subset"
                );
                ensure!(
                    !item.trim_start().starts_with('[') && !item.trim_start().starts_with('-'),
                    "{file}:{number}: a nested list is outside the subset"
                );
                scalars.push(parse_scalar(file, number, item.trim())?);
            }
        } else if line.trim() == "-" {
            bail!("{file}:{number}: an empty list item is outside the subset; write \"\"");
        } else if let Some(pair) = line.strip_prefix("    ") {
            let Some(map) = maps.last_mut() else {
                bail!(
                    "{file}:{number}: an indented line that is not a list item is a nested map, outside the subset"
                );
            };
            ensure!(
                !pair.starts_with(' ') && !pair.starts_with('-'),
                "{file}:{number}: a nested map or list inside a list item is outside the subset"
            );
            let Some((name, value)) = map_pair(pair) else {
                bail!("{file}:{number}: a map line in a list item is `key: value`");
            };
            ensure!(
                !map.iter().any(|(existing, _)| existing == name),
                "{file}:{number}: duplicate key `{name}` in one list item"
            );
            let value = map_value(file, number, name, value)?;
            map.push((name.to_string(), value));
        } else if map_pair(line.trim_start()).is_some() {
            bail!("{file}:{number}: a nested map is outside the subset");
        } else {
            bail!(
                "{file}:{number}: under a key, each line is a `  - ` list item or a `    key: value` line of one"
            );
        }
    }
    Ok(if maps.is_empty() {
        Value::List(scalars)
    } else {
        Value::Maps(maps)
    })
}

/// `name: value` when `text` starts with a valid key followed by `:` and a space or the
/// end; otherwise `None`, so a scalar holding `: ` is refused by the scalar rules.
fn map_pair(text: &str) -> Option<(&str, &str)> {
    let (name, rest) = text.split_once(':')?;
    if !valid_key(name) || !(rest.is_empty() || rest.starts_with(' ')) {
        return None;
    }
    Some((name, rest.trim()))
}

fn map_value(file: &str, number: usize, name: &str, value: &str) -> Result<String> {
    ensure!(
        !value.is_empty(),
        "{file}:{number}: `{name}` has no value; a nested map or list is outside the subset (write \"\" for empty)"
    );
    ensure!(
        !value.starts_with('['),
        "{file}:{number}: a list inside a list item is outside the subset"
    );
    parse_scalar(file, number, value)
}

fn parse_flow(file: &str, number: usize, text: &str) -> Result<Vec<String>> {
    let inner = text
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .ok_or_else(|| anyhow::anyhow!("{file}:{number}: a flow list is `[a, b]` on one line"))?;
    if inner.trim().is_empty() {
        return Ok(vec![]);
    }
    let mut items = vec![];
    let mut current = String::new();
    let mut quoted = false;
    let mut escaped = false;
    for character in inner.chars() {
        if quoted {
            current.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                quoted = false;
            }
            continue;
        }
        match character {
            '"' => {
                quoted = true;
                current.push(character);
            }
            ',' => {
                items.push(std::mem::take(&mut current));
            }
            '[' | ']' | '{' | '}' => {
                bail!("{file}:{number}: a nested flow list or map is outside the subset")
            }
            _ => current.push(character),
        }
    }
    ensure!(
        !quoted,
        "{file}:{number}: a quoted list item has no closing quote"
    );
    items.push(current);
    items
        .iter()
        .map(|item| {
            let item = item.trim();
            ensure!(
                !item.is_empty(),
                "{file}:{number}: an empty flow list item is outside the subset; write \"\""
            );
            parse_scalar(file, number, item)
        })
        .collect()
}

fn parse_scalar(file: &str, number: usize, text: &str) -> Result<String> {
    if let Some(rest) = text.strip_prefix('"') {
        let mut value = String::new();
        let mut characters = rest.chars();
        loop {
            match characters.next() {
                None => bail!("{file}:{number}: a quoted value has no closing quote"),
                Some('"') => break,
                Some('\\') => match characters.next() {
                    Some('"') => value.push('"'),
                    Some('\\') => value.push('\\'),
                    Some('n') => value.push('\n'),
                    Some(other) => bail!(
                        "{file}:{number}: the escape `\\{other}` is outside the subset (only \\\", \\\\ and \\n)"
                    ),
                    None => bail!("{file}:{number}: a quoted value has no closing quote"),
                },
                Some(character) => value.push(character),
            }
        }
        let trailing: String = characters.collect();
        ensure!(
            trailing.trim().is_empty(),
            "{file}:{number}: text after a closing quote is outside the subset"
        );
        return Ok(value);
    }
    let first = text.chars().next().unwrap_or(' ');
    let rule = match first {
        '\'' => Some("a single-quoted value is outside the subset; use double quotes"),
        '&' | '*' => Some("anchors and aliases are outside the subset"),
        '!' => Some("tags are outside the subset"),
        '|' | '>' => Some("block scalars are outside the subset"),
        '{' => Some("a flow map is outside the subset"),
        '%' | '@' | '`' => Some("a value cannot start with a reserved indicator; quote it"),
        _ => None,
    };
    if let Some(rule) = rule {
        bail!("{file}:{number}: {rule}");
    }
    ensure!(
        !text.starts_with("- ") && !text.starts_with("? ") && text != "-" && text != "?",
        "{file}:{number}: a value cannot start with `- ` or `? `; quote it"
    );
    ensure!(
        !text.contains(": ") && !text.ends_with(':') && !text.contains(" #"),
        "{file}:{number}: a plain value cannot hold `: `, end with `:` or hold ` #`; quote it"
    );
    Ok(text.to_string())
}

fn valid_key(key: &str) -> bool {
    let mut bytes = key.bytes();
    matches!(bytes.next(), Some(byte) if byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
}

fn render_entry(key: &str, value: &Value) -> String {
    match value {
        Value::Scalar(text) => format!("{key}: {}\n", render_scalar(text, false)),
        Value::List(items) if items.is_empty() => format!("{key}: []\n"),
        Value::List(items) => {
            let flow = items
                .iter()
                .map(|item| render_scalar(item, true))
                .collect::<Vec<_>>()
                .join(", ");
            if key.len() + flow.len() + 4 <= 100 {
                format!("{key}: [{flow}]\n")
            } else {
                let mut out = format!("{key}:\n");
                for item in items {
                    out.push_str(&format!("  - {}\n", render_scalar(item, false)));
                }
                out
            }
        }
        Value::Maps(maps) if maps.is_empty() => format!("{key}: []\n"),
        Value::Maps(maps) => {
            let mut out = format!("{key}:\n");
            for map in maps {
                for (position, (name, text)) in map.iter().enumerate() {
                    let lead = if position == 0 { "  - " } else { "    " };
                    out.push_str(&format!("{lead}{name}: {}\n", render_scalar(text, false)));
                }
            }
            out
        }
    }
}

/// A plain scalar when it reads back unchanged, otherwise a double-quoted one.
fn render_scalar(text: &str, in_flow: bool) -> String {
    if plain_safe(text, in_flow) {
        text.to_string()
    } else {
        let mut out = String::from("\"");
        for character in text.chars() {
            match character {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                other => out.push(other),
            }
        }
        out.push('"');
        out
    }
}

fn plain_safe(text: &str, in_flow: bool) -> bool {
    let Some(first) = text.chars().next() else {
        return false;
    };
    !text.starts_with(' ')
        && !text.ends_with(' ')
        && !text.ends_with(':')
        && !text.contains(": ")
        && !text.contains(" #")
        && !text.contains(['\n', '\r', '\t', '"', '\\'])
        && !"'&*!|>{[%@`#\"".contains(first)
        && !text.starts_with("- ")
        && !text.starts_with("? ")
        && text != "-"
        && text != "?"
        && !(in_flow && text.contains([',', '[', ']', '{', '}']))
}

/// The byte range of the text under `## <heading>`, after its heading line, up to the
/// next level one or two heading outside a code fence.
fn section_span(body: &str, heading: &str) -> Option<(usize, usize)> {
    let mut offset = 0;
    let mut fenced = false;
    let mut start = None;
    for line in body.split_inclusive('\n') {
        let content = line.trim_end_matches('\n').trim_end();
        let fence = content.trim_start().starts_with("```");
        if !fenced && !fence {
            let is_heading = content.starts_with("# ") || content.starts_with("## ");
            if let Some(begin) = start {
                if is_heading {
                    return Some((begin, offset));
                }
            } else if content.strip_prefix("## ") == Some(heading) {
                start = Some(offset + line.len());
            }
        }
        if fence {
            fenced = !fenced;
        }
        offset += line.len();
    }
    start.map(|begin| (begin, body.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SESSION: &str = "---\nkind: session\nid: 2026-11-10-alpha\naliases: [alpha, BUG-0012]\nobjective: \"Ship: alpha\"\nwaiting_on_human:\n  - gate: Scope approval\n    holds: plan\n    drives: approve\ncriteria:\n  - DRIVE-24\n  - DRIVE-25\n---\n# Ship alpha\n\n## Goal\n\nA person's words.\n\n## Timeline\n\n- 2026-11-10 entry one ^t-aaaaaa\n\n## Notes\n\nKeep me.\n";

    #[test]
    fn the_subset_reads_scalars_lists_and_maps() {
        let record = parse("s.md", SESSION).unwrap();
        assert_eq!(record.scalar("kind"), Some("session"));
        assert_eq!(record.scalar("objective"), Some("Ship: alpha"));
        assert_eq!(record.list("aliases"), vec!["alpha", "BUG-0012"]);
        assert_eq!(record.list("criteria"), vec!["DRIVE-24", "DRIVE-25"]);
        assert_eq!(
            record.maps("waiting_on_human"),
            vec![vec![
                ("gate".to_string(), "Scope approval".to_string()),
                ("holds".to_string(), "plan".to_string()),
                ("drives".to_string(), "approve".to_string()),
            ]]
        );
        assert_eq!(record.list("missing"), Vec::<String>::new());
        assert_eq!(record.scalar("missing"), None);
        assert_eq!(
            record.render(),
            SESSION,
            "an unchanged record renders byte for byte"
        );
    }

    #[test]
    fn unknown_keys_blank_lines_and_comments_survive_a_rewrite() {
        let text = "---\nkind: bug\n# a person's comment\nmy_note:   kept   as typed\n\nstatus: open\n---\nbody\n";
        let mut record = parse("b.md", text).unwrap();
        record.set_scalar("status", "verified").unwrap();
        record.set_scalar("kind", "bug").unwrap();
        assert_eq!(
            record.render(),
            "---\nkind: bug\n# a person's comment\nmy_note:   kept   as typed\n\nstatus: verified\n---\nbody\n"
        );
    }

    #[test]
    fn values_render_back_to_what_was_set() {
        let mut record = Record::new("");
        let awkward = [
            "",
            "plain",
            "has: colon",
            "ends:",
            " leading",
            "quote \" and \\ slash",
            "two\nlines",
            "# not a comment",
            "- not a list",
            "[not a list]",
            "'single'",
            "&anchor",
            "a, b",
        ];
        for (index, text) in awkward.iter().enumerate() {
            record.set_scalar(&format!("k{index}"), text).unwrap();
        }
        record
            .set(
                "flow",
                Value::List(awkward.iter().map(|text| text.to_string()).collect()),
            )
            .unwrap();
        record
            .set(
                "maps",
                Value::Maps(vec![vec![
                    ("gate".into(), "has: colon".into()),
                    ("holds".into(), "plan".into()),
                ]]),
            )
            .unwrap();
        let read = parse("r.md", &record.render()).unwrap();
        for (index, text) in awkward.iter().enumerate() {
            assert_eq!(read.scalar(&format!("k{index}")), Some(*text), "{text:?}");
        }
        assert_eq!(
            read.list("flow"),
            awkward.iter().map(|t| t.to_string()).collect::<Vec<_>>()
        );
        assert_eq!(read.maps("maps")[0][0].1, "has: colon");
    }

    #[test]
    fn crlf_reads_as_lf() {
        let crlf = SESSION.replace('\n', "\r\n");
        let record = parse("s.md", &crlf).unwrap();
        assert_eq!(record.render(), SESSION);
    }

    #[test]
    fn front_matter_outside_the_subset_is_refused_with_file_line_and_rule() {
        for (text, line, rule) in [
            ("---\nkey: value\n", "s.md:2", "no closing `---` line"),
            ("no front matter\n", "s.md:1", "starts with a `---` line"),
            ("---\nouter:\n  inner: value\n---\n", "s.md:3", "nested map"),
            (
                "---\nkey: &anchor value\n---\n",
                "s.md:2",
                "anchors and aliases",
            ),
            ("---\nkey: *alias\n---\n", "s.md:2", "anchors and aliases"),
            (
                "---\nkey: one\nkey: two\n---\n",
                "s.md:3",
                "duplicate key `key`",
            ),
            ("---\nkey: |\n  block\n---\n", "s.md:2", "block scalars"),
            ("---\nkey: {a: b}\n---\n", "s.md:2", "flow map"),
            ("---\nkey: [a, [b]]\n---\n", "s.md:2", "nested flow list"),
            ("---\nkey: 'single'\n---\n", "s.md:2", "single-quoted"),
            ("---\nkey: \"bad \\t escape\"\n---\n", "s.md:2", "escape"),
            ("---\nkey: a: b\n---\n", "s.md:2", "cannot hold `: `"),
            (
                "---\nlist:\n  - a\n  - k: v\n---\n",
                "s.md:4",
                "mixes scalars and maps",
            ),
            (
                "---\nlist:\n  - k: v\n      deep: x\n---\n",
                "s.md:4",
                "nested map or list",
            ),
            ("---\nlist:\n  - k:\n---\n", "s.md:3", "no value"),
            ("---\n\tkey: v\n---\n", "s.md:2", "tab"),
            ("---\n  - orphan\n---\n", "s.md:2", "nested map"),
        ] {
            let error = parse("s.md", text).unwrap_err().to_string();
            assert!(error.starts_with(&format!("{line}:")), "{text:?}: {error}");
            assert!(error.contains(rule), "{text:?}: {error}");
        }
    }

    #[test]
    fn appending_to_a_section_keeps_every_other_body_byte() {
        let mut record = parse("s.md", SESSION).unwrap();
        record
            .append_to_section("Timeline", "- 2026-11-11 entry two ^t-bbbbbb\n")
            .unwrap();
        let expected = SESSION.replace(
            "entry one ^t-aaaaaa\n\n## Notes",
            "entry one ^t-aaaaaa\n- 2026-11-11 entry two ^t-bbbbbb\n\n## Notes",
        );
        assert_eq!(record.render(), expected);
        assert_eq!(
            record.section("Timeline"),
            Some("\n- 2026-11-10 entry one ^t-aaaaaa\n- 2026-11-11 entry two ^t-bbbbbb\n\n")
        );
        record
            .append_to_section("Corrections", "- fixed\n")
            .unwrap();
        assert!(
            record
                .render()
                .ends_with("Keep me.\n\n## Corrections\n\n- fixed\n")
        );
    }

    #[test]
    fn a_heading_inside_a_code_fence_is_not_a_section() {
        let mut record = Record::new("## Log\n\n```\n## Log\n```\n- one\n\n## After\n");
        record.append_to_section("Log", "- two\n").unwrap();
        assert_eq!(
            record.body(),
            "## Log\n\n```\n## Log\n```\n- one\n- two\n\n## After\n"
        );
    }

    #[test]
    fn a_section_at_the_end_without_a_newline_gets_one() {
        let mut record = Record::new("## Log\n\n- one");
        record.append_to_section("Log", "- two").unwrap();
        assert_eq!(record.body(), "## Log\n\n- one\n- two\n");
    }

    #[test]
    fn a_record_over_its_bound_is_refused_and_nothing_is_written() {
        let root = tempfile::tempdir().unwrap();
        let mut record = Record::new(&"x".repeat(RECORD_BOUND as usize));
        record.set_scalar("kind", "bug").unwrap();
        let error = write_new(root.path(), ".chough/bugs/a.md", &record, Kind::Bug)
            .unwrap_err()
            .to_string();
        assert!(error.contains("over its bound of 65536 bytes"), "{error}");
        assert!(!root.path().join(".chough/bugs/a.md").exists());
        let session = Record::new(&"x".repeat(RECORD_BOUND as usize));
        assert!(
            write_new(
                root.path(),
                ".chough/sessions/a/session.md",
                &session,
                Kind::Session
            )
            .unwrap()
        );
        assert_eq!(Kind::Receipt.bound(), 4_194_304);
        assert_eq!(Kind::Story.bound(), 1_048_576);
    }

    #[test]
    fn leaks_are_checked_in_every_field_and_redacted_on_request() {
        let local = crate::leaks::Local::new(Some("alice"), Some("alices-mbp"));
        let mut record = Record::new("## Steps\n\nRan in /Users/alice/app.\n");
        record.set_scalar("id", "2026-11-10-x").unwrap();
        record
            .set(
                "relay",
                Value::Maps(vec![vec![("where".into(), "chat on alices-mbp".into())]]),
            )
            .unwrap();
        let error = record.check_leaks(&local).unwrap_err().to_string();
        assert_eq!(
            error.split(';').next().unwrap(),
            "refused:leak: relay[0].where holds host-name"
        );
        assert_eq!(record.redact_leaks(&local), 2);
        record.check_leaks(&local).unwrap();
        assert_eq!(record.maps("relay")[0][0].1, "chat on <redacted:host-name>");
        assert_eq!(record.body(), "## Steps\n\nRan in <redacted:home-path>.\n");
    }

    #[test]
    fn an_id_resolves_by_name_or_alias_and_a_shared_alias_is_refused() {
        let root = tempfile::tempdir().unwrap();
        let bug = |id: &str, aliases: &[&str]| {
            let mut record = Record::new("");
            record.set_scalar("id", id).unwrap();
            record
                .set(
                    "aliases",
                    Value::List(aliases.iter().map(|a| a.to_string()).collect()),
                )
                .unwrap();
            record
        };
        write_new(
            root.path(),
            "b/2026-11-10-a.md",
            &bug("2026-11-10-a", &["BUG-0012"]),
            Kind::Bug,
        )
        .unwrap();
        write_new(
            root.path(),
            "b/2026-11-10-b.md",
            &bug("2026-11-10-b", &["BUG-0013"]),
            Kind::Bug,
        )
        .unwrap();
        let by_alias = resolve(root.path(), "b", "BUG-0012").unwrap().unwrap();
        assert_eq!(by_alias.0, "b/2026-11-10-a.md");
        let by_name = resolve(root.path(), "b", "2026-11-10-b").unwrap().unwrap();
        assert_eq!(by_name.1.list("aliases"), vec!["BUG-0013"]);
        assert!(resolve(root.path(), "b", "BUG-9999").unwrap().is_none());
        assert!(resolve(root.path(), "missing", "x").unwrap().is_none());
        write_new(
            root.path(),
            "b/2026-11-10-c.md",
            &bug("2026-11-10-c", &["BUG-0012"]),
            Kind::Bug,
        )
        .unwrap();
        let error = resolve(root.path(), "b", "BUG-0012")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("b/2026-11-10-a.md, b/2026-11-10-c.md"),
            "{error}"
        );
    }

    #[test]
    fn write_new_never_replaces_and_replace_does() {
        let root = tempfile::tempdir().unwrap();
        let mut record = Record::new("first\n");
        record.set_scalar("id", "2026-11-10-a").unwrap();
        assert!(write_new(root.path(), "r/a.md", &record, Kind::Evidence).unwrap());
        let mut second = record.clone();
        second.append_to_section("Notes", "- later\n").unwrap();
        assert!(!write_new(root.path(), "r/a.md", &second, Kind::Evidence).unwrap());
        assert_eq!(
            std::fs::read_to_string(root.path().join("r/a.md")).unwrap(),
            record.render()
        );
        replace(root.path(), "r/a.md", &second, Kind::Evidence).unwrap();
        assert_eq!(
            std::fs::read_to_string(root.path().join("r/a.md")).unwrap(),
            second.render()
        );
    }
}
