//! No leaks in committed records (REQ-records-006): local paths, the local user and host
//! names, email addresses and credentials are refused, or redacted on request, before a
//! record is written. Matchers are hand-written so no dependency is added; a refusal
//! names the field and the kind, never the matched text.
// Plan step 3 wires the record layer into every write and removes this allowance.
#![allow(dead_code)]

use anyhow::{Result, bail};
use std::sync::OnceLock;

/// What a match is; its name is what refusals, redactions and findings show.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Kind {
    HomePath,
    SystemPath,
    WindowsPath,
    HostName,
    UserName,
    Email,
    Token,
    PrivateKey,
    Authorization,
    Jwt,
    Credential,
}

impl Kind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Kind::HomePath => "home-path",
            Kind::SystemPath => "system-path",
            Kind::WindowsPath => "windows-path",
            Kind::HostName => "host-name",
            Kind::UserName => "user-name",
            Kind::Email => "email",
            Kind::Token => "token",
            Kind::PrivateKey => "private-key",
            Kind::Authorization => "authorization",
            Kind::Jwt => "jwt",
            Kind::Credential => "credential",
        }
    }
}

/// One match: its kind and byte range in the value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Match {
    pub(crate) kind: Kind,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

/// The local facts a value is checked against.
#[derive(Clone, Debug, Default)]
pub(crate) struct Local {
    user: Option<String>,
    hosts: Vec<String>,
}

impl Local {
    /// This machine's user and host names, read once per process.
    pub(crate) fn current() -> &'static Local {
        static LOCAL: OnceLock<Local> = OnceLock::new();
        LOCAL.get_or_init(|| {
            let user = ["USER", "LOGNAME", "USERNAME"]
                .iter()
                .find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()));
            Local::new(user.as_deref(), host_name().as_deref())
        })
    }

    /// Facts given explicitly, for tests and for a migration run on another machine's
    /// records.
    pub(crate) fn new(user: Option<&str>, host: Option<&str>) -> Local {
        let user = user
            .map(str::trim)
            .filter(|user| user.len() >= 2 && user.bytes().all(name_byte))
            .map(str::to_string);
        let mut hosts = vec![];
        if let Some(host) = host.map(str::trim) {
            for candidate in [host, host.split('.').next().unwrap_or(host)] {
                if candidate.len() >= 3
                    && !candidate.eq_ignore_ascii_case("localhost")
                    && candidate
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.'))
                    && !hosts
                        .iter()
                        .any(|known: &String| known.eq_ignore_ascii_case(candidate))
                {
                    hosts.push(candidate.to_string());
                }
            }
        }
        Local { user, hosts }
    }
}

#[cfg(unix)]
fn host_name() -> Option<String> {
    let mut buffer = [0_u8; 256];
    // SAFETY: the buffer is valid for its whole length, and gethostname writes at most
    // that many bytes.
    let status = unsafe { libc::gethostname(buffer.as_mut_ptr().cast(), buffer.len()) };
    if status != 0 {
        return None;
    }
    let end = buffer
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(buffer.len());
    String::from_utf8(buffer[..end].to_vec()).ok()
}

#[cfg(not(unix))]
fn host_name() -> Option<String> {
    std::env::var("COMPUTERNAME").ok()
}

static REDACTING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Whether this command was given `--redact`.
pub(crate) fn set_redacting(on: bool) {
    REDACTING.store(on, std::sync::atomic::Ordering::Relaxed);
}

pub(crate) fn redacting() -> bool {
    REDACTING.load(std::sync::atomic::Ordering::Relaxed)
}

/// Check `value` for `field`, or redact it in place under `--redact`; returns how many
/// matches were redacted.
pub(crate) fn guard(local: &Local, field: &str, value: &mut String) -> Result<usize> {
    if redacting() {
        let (text, found) = redact(local, value);
        if found > 0 {
            *value = text;
        }
        return Ok(found);
    }
    check(local, field, value)?;
    Ok(0)
}

/// Refuse a value that leaks, naming the field and the kinds, never the text.
pub(crate) fn check(local: &Local, field: &str, value: &str) -> Result<()> {
    let found = find(local, value);
    if found.is_empty() {
        return Ok(());
    }
    let mut kinds: Vec<&str> = found.iter().map(|found| found.kind.name()).collect();
    kinds.sort_unstable();
    kinds.dedup();
    bail!(
        "refused:leak: {field} holds {}; remove it, or pass --redact to store `<redacted:<kind>>` instead; nothing was written",
        kinds.join(", ")
    )
}

/// Refuse a name a person gives, such as a session key or an explicit entry ID, that
/// holds this machine's host name as hyphen-separated words: a name becomes a committed
/// folder or file name, its `id` and its alias, and `--redact` cannot rename it
/// (REQ-records-006). Every other kind is checked as in any value.
pub(super) fn check_name(local: &Local, field: &str, name: &str) -> Result<()> {
    check(local, field, name)?;
    let words = |text: &str| -> String {
        let mut out = String::from("-");
        for character in text.chars() {
            if character.is_ascii_alphanumeric() {
                out.push(character.to_ascii_lowercase());
            } else if !out.ends_with('-') {
                out.push('-');
            }
        }
        if !out.ends_with('-') {
            out.push('-');
        }
        out
    };
    let name_words = words(name);
    if local
        .hosts
        .iter()
        .any(|host| name_words.contains(&words(host)))
    {
        bail!(
            "refused:leak: {field} holds host-name; choose a name without it; nothing was written"
        );
    }
    Ok(())
}

/// Replace every match with `<redacted:<kind>>`; returns the text and the count.
pub(crate) fn redact(local: &Local, value: &str) -> (String, usize) {
    let found = find(local, value);
    let mut out = String::with_capacity(value.len());
    let mut at = 0;
    for found in &found {
        out.push_str(&value[at..found.start]);
        out.push_str(&format!("<redacted:{}>", found.kind.name()));
        at = found.end;
    }
    out.push_str(&value[at..]);
    (out, found.len())
}

/// Every match in `value`, in order and not overlapping: where two overlap, the one
/// that starts first, then the longer, wins.
pub(crate) fn find(local: &Local, value: &str) -> Vec<Match> {
    let mut all = vec![];
    paths(value, &mut all);
    windows_paths(value, &mut all);
    names(local, value, &mut all);
    emails(value, &mut all);
    tokens(value, &mut all);
    private_keys(value, &mut all);
    authorization(value, &mut all);
    jwts(value, &mut all);
    credentials(value, &mut all);
    all.sort_by(|left, right| left.start.cmp(&right.start).then(right.end.cmp(&left.end)));
    let mut kept: Vec<Match> = vec![];
    for found in all {
        if kept.last().is_none_or(|last| found.start >= last.end) {
            kept.push(found);
        }
    }
    kept
}

/// The temporary roots a case definition may name on purpose (REQ-records-006).
const CASE_TEMP_ROOTS: [&str; 2] = ["/private/tmp/", "/tmp/"];

/// As [`find`], for a line of a case definition (`.chough/cases/*.cases.toml`): a path
/// under `/tmp/` or `/private/tmp/` is a deliberate test setting, such as a case's own
/// `XDG_DATA_HOME`, so it is not a leak, while a home path, the local user or host name,
/// an email or a credential in it still is (REQ-records-006). Each such root is read as
/// slashes, so the rest of its path is still checked as a path.
pub(crate) fn find_in_case_definition(local: &Local, value: &str) -> Vec<Match> {
    let bytes = value.as_bytes();
    let mut neutral = value.to_string();
    for root in CASE_TEMP_ROOTS {
        for (start, _) in value.match_indices(root) {
            if starts_word(bytes, start) {
                neutral.replace_range(start..start + root.len(), &"/".repeat(root.len()));
            }
        }
    }
    find(local, &neutral)
}

fn name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-')
}

/// Whether the byte before `at` ends a word, so a match may start at `at`.
fn starts_word(bytes: &[u8], at: usize) -> bool {
    at == 0
        || !(bytes[at - 1].is_ascii_alphanumeric()
            || matches!(bytes[at - 1], b'.' | b'_' | b'-' | b'~'))
}

/// The end of a path token that starts at `start`.
fn path_end(bytes: &[u8], start: usize) -> usize {
    let mut end = start;
    while end < bytes.len()
        && !bytes[end].is_ascii_whitespace()
        && !matches!(
            bytes[end],
            b'"' | b'\'' | b'`' | b')' | b'(' | b'<' | b'>' | b',' | b';' | b'|' | b']' | b'['
        )
    {
        end += 1;
    }
    // A sentence's closing period or colon is not part of the path.
    while end > start + 1 && matches!(bytes[end - 1], b'.' | b':') {
        end -= 1;
    }
    end
}

const HOME_ROOTS: [&str; 3] = ["/Users/", "/home/", "/root/"];
const SYSTEM_ROOTS: [&str; 5] = ["/private/", "/tmp/", "/var/", "/Volumes/", "/mnt/"];

fn paths(value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    for (roots, kind) in [
        (&HOME_ROOTS[..], Kind::HomePath),
        (&SYSTEM_ROOTS[..], Kind::SystemPath),
    ] {
        for root in roots {
            for (start, _) in value.match_indices(root) {
                if starts_word(bytes, start) {
                    all.push(Match {
                        kind,
                        start,
                        end: path_end(bytes, start),
                    });
                }
            }
        }
    }
    for (start, _) in value.match_indices("~/") {
        if starts_word(bytes, start) {
            all.push(Match {
                kind: Kind::HomePath,
                start,
                end: path_end(bytes, start),
            });
        }
    }
}

fn windows_paths(value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    for start in 0..bytes.len().saturating_sub(2) {
        if bytes[start].is_ascii_alphabetic()
            && bytes[start + 1] == b':'
            && matches!(bytes[start + 2], b'\\' | b'/')
            && (start == 0 || !(bytes[start - 1].is_ascii_alphanumeric() || matches!(bytes[start - 1], b'.' | b'_' | b'-' | b'/' | b':')))
            // `https://` and other schemes put two slashes after the colon.
            && !(bytes[start + 2] == b'/' && bytes.get(start + 3) == Some(&b'/'))
        {
            all.push(Match {
                kind: Kind::WindowsPath,
                start,
                end: path_end(bytes, start),
            });
        }
    }
}

fn names(local: &Local, value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    let lower = value.to_ascii_lowercase();
    for host in &local.hosts {
        let needle = host.to_ascii_lowercase();
        for (start, _) in lower.match_indices(&needle) {
            let end = start + needle.len();
            let before = start == 0
                || !(bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'-');
            let after =
                end == bytes.len() || !(bytes[end].is_ascii_alphanumeric() || bytes[end] == b'-');
            if before && after {
                all.push(Match {
                    kind: Kind::HostName,
                    start,
                    end,
                });
            }
        }
    }
    let Some(user) = &local.user else {
        return;
    };
    for (start, _) in value.match_indices(user.as_str()) {
        let end = start + user.len();
        let in_path = start > 0
            && matches!(bytes[start - 1], b'/' | b'\\')
            && (end == bytes.len() || matches!(bytes[end], b'/' | b'\\') || !name_byte(bytes[end]));
        let at_host = starts_word(bytes, start)
            && bytes.get(end) == Some(&b'@')
            && bytes
                .get(end + 1)
                .is_some_and(|byte| byte.is_ascii_alphanumeric());
        if in_path || at_host {
            let end = if at_host {
                host_end(bytes, end + 1)
            } else {
                end
            };
            all.push(Match {
                kind: Kind::UserName,
                start,
                end,
            });
        }
    }
}

fn host_end(bytes: &[u8], mut end: usize) -> usize {
    while end < bytes.len()
        && (bytes[end].is_ascii_alphanumeric() || matches!(bytes[end], b'-' | b'.'))
    {
        end += 1;
    }
    while end > 0 && bytes[end - 1] == b'.' {
        end -= 1;
    }
    end
}

fn emails(value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    for (at, _) in value.match_indices('@') {
        let mut start = at;
        while start > 0
            && (bytes[start - 1].is_ascii_alphanumeric()
                || matches!(bytes[start - 1], b'.' | b'_' | b'%' | b'+' | b'-'))
        {
            start -= 1;
        }
        if start == at || &value[start..at] == "git" {
            continue;
        }
        let end = host_end(bytes, at + 1);
        let domain = &value[at + 1..end];
        let Some((_, tld)) = domain.rsplit_once('.') else {
            continue;
        };
        if tld.len() >= 2 && tld.bytes().all(|byte| byte.is_ascii_alphabetic()) {
            all.push(Match {
                kind: Kind::Email,
                start,
                end,
            });
        }
    }
}

/// The end of a run of `allowed` bytes starting at `from`.
fn run(bytes: &[u8], from: usize, allowed: impl Fn(u8) -> bool) -> usize {
    let mut end = from;
    while end < bytes.len() && allowed(bytes[end]) {
        end += 1;
    }
    end
}

fn token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-')
}

/// A token prefix, the fewest bytes that follow it, and which bytes may follow.
type TokenForm = (&'static str, usize, fn(u8) -> bool);

fn tokens(value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    let forms: [TokenForm; 12] = [
        ("ghp_", 30, |byte| byte.is_ascii_alphanumeric()),
        ("gho_", 30, |byte| byte.is_ascii_alphanumeric()),
        ("ghu_", 30, |byte| byte.is_ascii_alphanumeric()),
        ("ghs_", 30, |byte| byte.is_ascii_alphanumeric()),
        ("ghr_", 30, |byte| byte.is_ascii_alphanumeric()),
        ("github_pat_", 20, |byte| {
            byte.is_ascii_alphanumeric() || byte == b'_'
        }),
        ("glpat-", 20, token_byte),
        ("xoxb-", 10, token_byte),
        ("xoxp-", 10, token_byte),
        ("xoxa-", 10, token_byte),
        ("sk-", 20, token_byte),
        ("AIza", 30, token_byte),
    ];
    for (prefix, minimum, allowed) in forms {
        for (start, _) in value.match_indices(prefix) {
            if start > 0 && token_byte(bytes[start - 1]) {
                continue;
            }
            let end = run(bytes, start + prefix.len(), allowed);
            if end - start - prefix.len() >= minimum {
                all.push(Match {
                    kind: Kind::Token,
                    start,
                    end,
                });
            }
        }
    }
    for prefix in ["AKIA", "ASIA"] {
        for (start, _) in value.match_indices(prefix) {
            let end = run(bytes, start + prefix.len(), |byte| {
                byte.is_ascii_uppercase() || byte.is_ascii_digit()
            });
            if (start == 0 || !token_byte(bytes[start - 1])) && end - start == 20 {
                all.push(Match {
                    kind: Kind::Token,
                    start,
                    end,
                });
            }
        }
    }
}

fn private_keys(value: &str, all: &mut Vec<Match>) {
    for (start, _) in value.match_indices("-----BEGIN ") {
        let Some(header_end) = value[start..]
            .find("-----\n")
            .or_else(|| value[start + 11..].find("-----").map(|at| at + 11))
        else {
            continue;
        };
        let header = &value[start..start + header_end];
        if !header.contains("PRIVATE KEY") {
            continue;
        }
        let end = value[start..]
            .find("-----END ")
            .and_then(|at| {
                value[start + at + 9..]
                    .find("-----")
                    .map(|close| start + at + 9 + close + 5)
            })
            .unwrap_or(value.len());
        all.push(Match {
            kind: Kind::PrivateKey,
            start,
            end,
        });
    }
}

fn authorization(value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    let lower = value.to_ascii_lowercase();
    let credential_byte = |byte: u8| {
        byte.is_ascii_alphanumeric()
            || matches!(byte, b'.' | b'_' | b'~' | b'+' | b'/' | b'=' | b'-')
    };
    for (scheme, minimum, needs_header) in [("bearer ", 16, false), ("basic ", 8, true)] {
        for (start, _) in lower.match_indices(scheme) {
            if !starts_word(bytes, start) {
                continue;
            }
            if needs_header
                && !lower[..start]
                    .trim_end()
                    .trim_end_matches([':', '='])
                    .trim_end()
                    .ends_with("authorization")
            {
                continue;
            }
            let from = run(bytes, start + scheme.len(), |byte| byte == b' ');
            let end = run(bytes, from, credential_byte);
            if end - from >= minimum {
                all.push(Match {
                    kind: Kind::Authorization,
                    start,
                    end,
                });
            }
        }
    }
}

fn jwts(value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    for (start, _) in value.match_indices("eyJ") {
        if start > 0 && token_byte(bytes[start - 1]) {
            continue;
        }
        let first = run(bytes, start, token_byte);
        if first - start < 10 || bytes.get(first) != Some(&b'.') {
            continue;
        }
        let second = run(bytes, first + 1, token_byte);
        if second - first - 1 < 8 || bytes.get(second) != Some(&b'.') {
            continue;
        }
        let third = run(bytes, second + 1, token_byte);
        if third - second > 8 {
            all.push(Match {
                kind: Kind::Jwt,
                start,
                end: third,
            });
        }
    }
}

fn credentials(value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    let lower = value.to_ascii_lowercase();
    for word in [
        "password", "passwd", "secret", "token", "api_key", "apikey", "api-key",
    ] {
        for (at, _) in lower.match_indices(word) {
            let key_end = at + word.len();
            // The word ends the key: `GITHUB_TOKEN=` counts, `tokens:` does not.
            if bytes
                .get(key_end)
                .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
            {
                continue;
            }
            let mut cursor = run(bytes, key_end, |byte| byte == b' ');
            if !matches!(bytes.get(cursor), Some(b'=' | b':')) {
                continue;
            }
            cursor = run(bytes, cursor + 1, |byte| byte == b' ');
            if matches!(bytes.get(cursor), Some(b'"' | b'\'')) {
                cursor += 1;
            }
            if matches!(bytes.get(cursor), Some(b'$' | b'<' | b'{' | b'*')) {
                continue;
            }
            let mut end = run(bytes, cursor, |byte| {
                !byte.is_ascii_whitespace() && !matches!(byte, b'"' | b'\'' | b',' | b';')
            });
            // The value ends where Markdown takes over: at a link's `](`, and before
            // closing emphasis, code, brackets or a sentence's full stop, so the markup
            // Chough adds around a claim never lengthens it (REQ-records-006).
            if let Some(link) = value[cursor..end].find("](") {
                end = cursor + link;
            }
            while end > cursor && matches!(bytes[end - 1], b'.' | b'*' | b'_' | b'`' | b')' | b']')
            {
                end -= 1;
            }
            if end - cursor >= 8 {
                let start = lower[..at]
                    .rfind(|character: char| {
                        !(character.is_ascii_alphanumeric() || character == '_' || character == '-')
                    })
                    .map_or(0, |found| found + 1);
                all.push(Match {
                    kind: Kind::Credential,
                    start,
                    end,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local() -> Local {
        Local::new(Some("alice"), Some("alices-mbp.local"))
    }

    fn kinds(value: &str) -> Vec<&'static str> {
        find(&local(), value)
            .iter()
            .map(|found| found.kind.name())
            .collect()
    }

    /// REQ-records-006: in a case definition a temporary path is not a leak, but a user
    /// name inside it, a home path and a credential still are.
    #[test]
    fn a_case_definition_may_name_a_temporary_folder() {
        let local = Local::new(Some("alice"), Some("alice-mbp"));
        let kinds = |line: &str| -> Vec<&str> {
            find_in_case_definition(&local, line)
                .iter()
                .map(|found| found.kind.name())
                .collect()
        };
        assert!(kinds(r#"env = { XDG_DATA_HOME = "/tmp/case-data" }"#).is_empty());
        assert!(kinds(r#"argv = ["env", "X=/private/tmp/case", "true"]"#).is_empty());
        assert_eq!(kinds("X=/tmp/alice/data"), ["user-name"]);
        assert_eq!(kinds("X=/Users/bob/data"), ["home-path"]);
        assert_eq!(kinds("X=/var/folders/ab/T/x"), ["system-path"]);
        assert_eq!(find(&local, "X=/tmp/case-data").len(), 1);
    }

    #[test]
    fn each_kind_is_found_and_named() {
        let cases = [
            ("ran in /Users/alice/src/app", "home-path"),
            ("log at /home/bob/x.log", "home-path"),
            ("see ~/notes.md", "home-path"),
            ("cwd=/private/var/folders/ab/T/x", "system-path"),
            ("wrote /tmp/out.txt", "system-path"),
            ("disk /Volumes/Data/x", "system-path"),
            ("C:\\Users\\alice\\x", "windows-path"),
            ("built on alices-mbp today", "host-name"),
            ("built on ALICES-MBP.local today", "host-name"),
            ("ssh alice@build-box", "user-name"),
            ("mount /opt/alice/data", "user-name"),
            ("mail bob.smith+x@example.com now", "email"),
            (
                concat!("token ", "ghp_", "abcdefghijklmnopqrstuvwxyz0123456789"),
                "token",
            ),
            (concat!("key glp", "at-abcdefghijklmnopqrstuvwx"), "token"),
            (concat!("slack xo", "xb-123456789012-abcdef"), "token"),
            (concat!("aws AKIA", "ABCDEFGHIJKLMNOP id"), "token"),
            (
                concat!("model sk-", "ant-api03-abcdefghijklmnopqrstuv"),
                "token",
            ),
            (
                concat!(
                    "-----BEGIN OPENSSH ",
                    "PRIVATE KEY-----\nabc\n-----END OPENSSH ",
                    "PRIVATE KEY-----"
                ),
                "private-key",
            ),
            (
                concat!(
                    "curl -H 'Authorization: Bearer ",
                    "abcdefgh",
                    "ijklmnop1234'"
                ),
                "authorization",
            ),
            (
                concat!("Authorization: Basic ", "dXNlcjpw", "YXNzd29y", "ZA=="),
                "authorization",
            ),
            (
                concat!(
                    "jwt ",
                    "eyJ",
                    "hbGciOiJIUzI1NiJ9.",
                    "eyJ",
                    "zdWIiOiIxMjM0In0.abcdefghijkl"
                ),
                "jwt",
            ),
            (concat!("GITHUB_TOKEN=", "abcdef123456"), "credential"),
            (concat!("password: ", "hunter2", "hunter2"), "credential"),
            (concat!("api_key = \"", "0123456789abcdef\""), "credential"),
        ];
        for (value, kind) in cases {
            assert_eq!(kinds(value), vec![kind], "{value:?}");
            let error = check(&local(), "summary", value).unwrap_err().to_string();
            assert!(
                error.starts_with(&format!("refused:leak: summary holds {kind}")),
                "{error}"
            );
            for found in find(&local(), value) {
                assert!(!error.contains(&value[found.start..found.end]), "{error}");
            }
        }
    }

    #[test]
    fn ordinary_text_passes() {
        for value in [
            "relative src/main.rs and ./tmp/x and ../var/y",
            "https://github.com/CorvidLabs/chough/pull/238 and https://example.com/var/tmp/x",
            "git clone git@github.com:CorvidLabs/chough.git",
            "Leif approved: \"ok i approve\" (coordinator chat, 2026-11-11)",
            "the bearer of news; basic setup; tokens: 3; token: abc",
            "secret: keep it short",
            "token: $GITHUB_TOKEN and password=<redacted:credential>",
            "alice reviewed it",
            "package @types/node at chough@0.6.0",
            "cargo test --locked --test sessions",
            "",
        ] {
            assert_eq!(kinds(value), Vec::<&str>::new(), "{value:?}");
            check(&local(), "summary", value).unwrap();
        }
    }

    #[test]
    fn redaction_replaces_each_match_and_counts_them() {
        let (text, count) = redact(
            &local(),
            concat!(
                "at /Users/alice/x by alice@alices-mbp with ",
                "ghp_",
                "abcdefghijklmnopqrstuvwxyz0123456789 done"
            ),
        );
        assert_eq!(
            text,
            "at <redacted:home-path> by <redacted:user-name> with <redacted:token> done"
        );
        assert_eq!(count, 3);
        assert_eq!(redact(&local(), "clean"), ("clean".to_string(), 0));
    }

    #[test]
    fn a_short_or_generic_host_name_is_not_matched() {
        let generic = Local::new(Some("x"), Some("localhost"));
        assert!(find(&generic, "localhost and x and /x/").is_empty());
        let short = Local::new(None, Some("mb"));
        assert!(find(&short, "mb of memory").is_empty());
    }
}
