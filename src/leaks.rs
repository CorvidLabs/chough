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
    users: Vec<String>,
    hosts: Vec<String>,
}

impl Local {
    /// This machine's user and host names, read once per process: the account's user
    /// name, and `USER` or `LOGNAME` when it names another (#379); the host name, and on
    /// macOS the LocalHostName (#368).
    pub(crate) fn current() -> &'static Local {
        static LOCAL: OnceLock<Local> = OnceLock::new();
        LOCAL.get_or_init(|| {
            let variable = ["USER", "LOGNAME", "USERNAME"]
                .iter()
                .find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()));
            let mut local = Local::new(account_user().as_deref(), host_name().as_deref());
            if let Some(user) = variable {
                local.add_user(&user);
            }
            if let Some(host) = local_host_name() {
                local.add_host(&host);
            }
            local
        })
    }

    /// Facts given explicitly, for tests and for a migration run on another machine's
    /// records.
    pub(crate) fn new(user: Option<&str>, host: Option<&str>) -> Local {
        let mut local = Local::default();
        if let Some(user) = user {
            local.add_user(user);
        }
        if let Some(host) = host {
            local.add_host(host);
        }
        local
    }

    /// Another name of the local user.
    pub(super) fn add_user(&mut self, user: &str) {
        let user = user.trim();
        if user.len() >= 2
            && user.bytes().all(name_byte)
            && !self.users.iter().any(|known| known == user)
        {
            self.users.push(user.to_string());
        }
    }

    /// Another name of this machine: the name, its first label and each as its
    /// `<name>.local` Bonjour name (#368).
    pub(super) fn add_host(&mut self, host: &str) {
        let host = host.trim();
        let label = host.split('.').next().unwrap_or(host);
        let bonjour = format!("{label}.local");
        for candidate in [host, label, bonjour.as_str()] {
            if candidate.len() >= 3
                && !candidate.eq_ignore_ascii_case("localhost")
                && !candidate.eq_ignore_ascii_case("localhost.local")
                && !candidate.starts_with('.')
                && candidate
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.'))
                && !self
                    .hosts
                    .iter()
                    .any(|known: &String| known.eq_ignore_ascii_case(candidate))
            {
                self.hosts.push(candidate.to_string());
            }
        }
    }
}

/// The user name of the account this process runs as, whatever `USER` and `LOGNAME` say
/// (#379).
#[cfg(unix)]
fn account_user() -> Option<String> {
    let mut entry = std::mem::MaybeUninit::<libc::passwd>::uninit();
    let mut buffer = vec![0_u8; 4096];
    let mut found: *mut libc::passwd = std::ptr::null_mut();
    // SAFETY: every pointer is valid for the call, the buffer for its whole length, and
    // `found` is set to `entry` or null.
    let status = unsafe {
        libc::getpwuid_r(
            libc::getuid(),
            entry.as_mut_ptr(),
            buffer.as_mut_ptr().cast(),
            buffer.len(),
            &mut found,
        )
    };
    if status != 0 || found.is_null() {
        return None;
    }
    // SAFETY: getpwuid_r succeeded, so `entry` is initialized and `pw_name` points into
    // `buffer` at a NUL-terminated name.
    let name = unsafe { std::ffi::CStr::from_ptr((*found).pw_name) };
    name.to_str().ok().map(str::to_string)
}

#[cfg(not(unix))]
fn account_user() -> Option<String> {
    None
}

/// The macOS LocalHostName, which Bonjour shares as `<name>.local` and which can differ
/// from the host name (#368), read from the system preferences without starting a
/// process.
#[cfg(target_os = "macos")]
fn local_host_name() -> Option<String> {
    let text =
        std::fs::read_to_string("/Library/Preferences/SystemConfiguration/preferences.plist")
            .ok()?;
    let after = &text[text.find("<key>LocalHostName</key>")? + 24..];
    let value = after.trim_start().strip_prefix("<string>")?;
    let name = &value[..value.find("</string>")?];
    (!name.is_empty()).then(|| name.to_string())
}

#[cfg(not(target_os = "macos"))]
fn local_host_name() -> Option<String> {
    None
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

/// What `value` holds that an error must not show (REQ-records-006, #367): the kinds of
/// leak it matches, and control characters, invisible formatting ones included, or
/// `None` when an error may show it.
pub(super) fn unshowable(value: &str) -> Option<String> {
    let mut held: Vec<&str> = find(Local::current(), value)
        .iter()
        .map(|found| found.kind.name())
        .collect();
    held.sort_unstable();
    held.dedup();
    if value.chars().any(hidden) {
        held.push("control characters");
    }
    (!held.is_empty()).then(|| held.join(", "))
}

/// `value`, given as `field`, as an error shows it (REQ-records-006, #367): the value
/// itself when it is clean, otherwise `<field holding what>`, never the value.
pub(super) fn shown(field: &str, value: &str) -> String {
    match unshowable(value) {
        None => value.to_string(),
        Some(held) => format!("<{field} holding {held}>"),
    }
}

/// `text`, a message that quotes values a person gave, such as a usage error from the
/// argument parser, with each leak in it replaced by `<value holding <kind>>` and each
/// control or invisible formatting character by `<control character>`, so it shows what
/// a value holds and never the value (REQ-records-006, #367). A line break stays.
pub(super) fn masked(text: &str) -> String {
    let found = find(Local::current(), text);
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    let push_clean = |out: &mut String, part: &str| {
        for character in part.chars() {
            if hidden(character) && character != '\n' {
                out.push_str("<control character>");
            } else {
                out.push(character);
            }
        }
    };
    for found in &found {
        push_clean(&mut out, &text[at..found.start]);
        out.push_str(&format!("<value holding {}>", found.kind.name()));
        at = found.end;
    }
    push_clean(&mut out, &text[at..]);
    out
}

/// Whether `character` is a control character or an invisible formatting one, such as a
/// right-to-left override or a zero-width space, which a terminal acts on or hides.
fn hidden(character: char) -> bool {
    character.is_control()
        || matches!(
            character,
            '\u{ad}'
                | '\u{61c}'
                | '\u{180e}'
                | '\u{200b}'..='\u{200f}'
                | '\u{202a}'..='\u{202e}'
                | '\u{2060}'..='\u{2064}'
                | '\u{2066}'..='\u{206f}'
                | '\u{feff}'
                | '\u{fff9}'..='\u{fffb}'
        )
}

/// Refuse a name a person gives, such as a session key or an explicit entry ID, that
/// holds this machine's host name as hyphen-separated words: a name becomes a committed
/// folder or file name, its `id` and its alias, and `--redact` cannot rename it
/// (REQ-records-006). Since the name is a path, this holds for a short all-letter host
/// name too, as it does inside any path. Every other kind is checked as in any value.
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
    for (start, _) in value.match_indices('~') {
        // `~/` is this user's home, and `~<user>/` another user's (#368); a user name
        // starts with a letter or `_`, so `~1/3` is a fraction.
        let name_end = run(bytes, start + 1, name_byte);
        let named = name_end == start + 1
            || bytes[start + 1].is_ascii_alphabetic()
            || bytes[start + 1] == b'_';
        if starts_word(bytes, start) && named && bytes.get(name_end) == Some(&b'/') {
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

/// Whether a host name is a leak anywhere: one of 8 or more characters, or holding a
/// digit, a hyphen or a dot. A shorter all-letter one may be an ordinary word, so it is
/// a leak only where it names a host (REQ-records-006).
fn host_anywhere(host: &str) -> bool {
    host.len() >= 8 || !host.bytes().all(|byte| byte.is_ascii_alphabetic())
}

/// The byte ranges of the URLs in `value`: a scheme, `://` and the rest of its token.
fn urls(value: &str) -> Vec<(usize, usize)> {
    let bytes = value.as_bytes();
    let mut found = vec![];
    for (at, _) in value.match_indices("://") {
        let mut start = at;
        while start > 0
            && (bytes[start - 1].is_ascii_alphanumeric()
                || matches!(bytes[start - 1], b'+' | b'-' | b'.'))
        {
            start -= 1;
        }
        if start < at {
            found.push((start, path_end(bytes, at + 3)));
        }
    }
    found
}

/// Whether the name at `start..end` names a host: inside a path, as a component after a
/// `/` or `\` or before a `/`, inside a URL, or after `<user>@` (REQ-records-006). A `\`
/// after the name is not a separator, since in TOML or JSON it escapes a quote. An scp-style
/// remote path, `<host>:/<absolute path>`, names the host and the absolute path it names on
/// it, so the host is in context there like it is in a path (REQ-records-006).
fn names_a_host(bytes: &[u8], start: usize, end: usize, urls: &[(usize, usize)]) -> bool {
    let in_path =
        (start > 0 && matches!(bytes[start - 1], b'/' | b'\\')) || bytes.get(end) == Some(&b'/');
    let at_host = start >= 2 && bytes[start - 1] == b'@' && name_byte(bytes[start - 2]);
    let in_url = urls.iter().any(|(from, to)| *from <= start && end <= *to);
    let scp = bytes.get(end) == Some(&b':')
        && bytes
            .get(end + 1)
            .is_some_and(|byte| *byte == b'/' || *byte == b'\\');
    in_path || at_host || in_url || scp
}

fn names(local: &Local, value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    let lower = value.to_ascii_lowercase();
    let urls = urls(value);
    for host in &local.hosts {
        let needle = host.to_ascii_lowercase();
        let anywhere = host_anywhere(host);
        // A dotted name, such as a fully qualified one, counts glued into a hyphenated
        // name too, as in `build-<host>-1` (#368); a single label might be one word of a
        // slug, so a hyphen still continues it.
        let glued = host.contains('.');
        let continues = |byte: u8| byte.is_ascii_alphanumeric() || (byte == b'-' && !glued);
        for (start, _) in lower.match_indices(&needle) {
            let end = start + needle.len();
            let before = start == 0 || !continues(bytes[start - 1]);
            let after = end == bytes.len() || !continues(bytes[end]);
            if before && after && (anywhere || names_a_host(bytes, start, end, &urls)) {
                all.push(Match {
                    kind: Kind::HostName,
                    start,
                    end,
                });
            }
        }
    }
    for user in &local.users {
        user_names(user, value, all);
    }
}

/// The top folders of a system's root, which name no user even when a user has the same
/// name, as `/dev/` for a user named `dev` (#379).
const ROOT_FOLDERS: [&str; 24] = [
    "Applications",
    "Library",
    "System",
    "Users",
    "Volumes",
    "bin",
    "boot",
    "cores",
    "dev",
    "etc",
    "home",
    "lib",
    "lib64",
    "media",
    "mnt",
    "nix",
    "opt",
    "private",
    "proc",
    "root",
    "run",
    "sbin",
    "snap",
    "srv",
];

/// The places `user`, the local user name, appears in `value`: inside a path, but not as
/// the first folder of an absolute path where it names a system root, or as
/// `<user>@<host>` (REQ-records-006).
fn user_names(user: &str, value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    for (start, _) in value.match_indices(user) {
        let end = start + user.len();
        let at_root = start > 0
            && bytes[start - 1] == b'/'
            && (start == 1
                || (!name_byte(bytes[start - 2]) && !matches!(bytes[start - 2], b'/' | b'~')))
            && ROOT_FOLDERS.contains(&user);
        let in_path = start > 0
            && !at_root
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

/// Whether `byte` before a token prefix makes the prefix the middle of a word, so no
/// token starts there. A hyphen does not: a slug, an ID or a CI run name glues a token
/// to one, as `x-ghp_…`, and it is still a token (REQ-records-006, #267).
fn word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// A token prefix, the fewest bytes that follow it, which bytes may follow, and which
/// byte before the prefix continues a word, so no token starts there.
type TokenForm = (&'static str, usize, fn(u8) -> bool, fn(u8) -> bool);

fn tokens(value: &str, all: &mut Vec<Match>) {
    let bytes = value.as_bytes();
    let forms: [TokenForm; 14] = [
        ("ghp_", 30, |byte| byte.is_ascii_alphanumeric(), word_byte),
        ("gho_", 30, |byte| byte.is_ascii_alphanumeric(), word_byte),
        ("ghu_", 30, |byte| byte.is_ascii_alphanumeric(), word_byte),
        ("ghs_", 30, |byte| byte.is_ascii_alphanumeric(), word_byte),
        ("ghr_", 30, |byte| byte.is_ascii_alphanumeric(), word_byte),
        (
            "github_pat_",
            20,
            |byte| byte.is_ascii_alphanumeric() || byte == b'_',
            word_byte,
        ),
        ("glpat-", 20, token_byte, word_byte),
        ("xoxb-", 10, token_byte, word_byte),
        ("xoxp-", 10, token_byte, word_byte),
        ("xoxa-", 10, token_byte, word_byte),
        // A Slack app-level token (#368).
        ("xapp-", 10, token_byte, word_byte),
        // A Hugging Face access token (#368).
        ("hf_", 30, |byte| byte.is_ascii_alphanumeric(), word_byte),
        // `sk-` is short and common in slugs (`-sk-learn-…`, a generated bug ID), so a
        // hyphen before it continues a word, as in 0.6.0 (#316); the distinctive forms
        // above keep the hyphen boundary of #267.
        ("sk-", 20, token_byte, token_byte),
        ("AIza", 30, token_byte, word_byte),
    ];
    for (prefix, minimum, allowed, continues) in forms {
        for (start, _) in value.match_indices(prefix) {
            if start > 0 && continues(bytes[start - 1]) {
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
            if (start == 0 || !word_byte(bytes[start - 1])) && end - start == 20 {
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
        "password",
        "passwd",
        "secret",
        "secret_key",
        "secret_access_key",
        "token",
        "api_key",
        "apikey",
        "api-key",
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
            // A quoted key, as in JSON's `{"password": …}`, closes its quote first (#368).
            let mut cursor = key_end;
            if matches!(bytes.get(cursor), Some(b'"' | b'\'')) {
                cursor += 1;
            }
            cursor = run(bytes, cursor, |byte| byte == b' ');
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

    /// REQ-records-006 (#267): a hyphen before a token form is a boundary, as a slug,
    /// an ID or a CI run name glues one; a letter, digit or underscore still continues
    /// a word.
    #[test]
    fn a_token_glued_to_a_hyphen_is_found() {
        for value in [
            concat!("probe x-", "ghp_", "abcdefghij0123456789abcdefghij0123"),
            concat!("run-", "glp", "at-abcdefghijklmnopqrstuvwx"),
            concat!("ci-", "AKIA", "ABCDEFGHIJKLMNOP"),
        ] {
            assert_eq!(kinds(value), ["token"], "{value:?}");
            assert!(check(&local(), "summary", value).is_err(), "{value:?}");
        }
        for value in [
            concat!("probe x", "ghp_", "abcdefghij0123456789abcdefghij0123"),
            concat!("probe x_", "ghp_", "abcdefghij0123456789abcdefghij0123"),
            concat!("ciX", "AKIA", "ABCDEFGHIJKLMNOP"),
        ] {
            assert_eq!(kinds(value), Vec::<&str>::new(), "{value:?}");
        }
    }

    /// REQ-records-006 (#316): `sk-` after a hyphen continues a word, as in 0.6.0, so a
    /// slug such as a bug ID generated from "Fix flaky sk-learn tests in CI pipeline" is
    /// not a model-provider key; at the start of a value or after a space it still is,
    /// and the distinctive forms keep #267's hyphen boundary.
    #[test]
    fn sk_after_a_hyphen_continues_a_word() {
        for value in [
            "2026-11-18-fix-flaky-sk-learn-tests-in-ci-pipeline",
            "2026-11-18-use-sk-learn-for-the-classifier-model",
            concat!("x-", "sk-", "abcdefghijklmnopqrstuvwx"),
        ] {
            assert_eq!(kinds(value), Vec::<&str>::new(), "{value:?}");
            assert!(check(&local(), "id", value).is_ok(), "{value:?}");
        }
        for value in [
            concat!("sk-", "abcdefghijklmnopqrstuvwx"),
            concat!("key ", "sk-", "abcdefghijklmnopqrstuvwx"),
            concat!("probe x-", "ghp_", "abcdefghij0123456789abcdefghij0123"),
            concat!("run-", "glp", "at-abcdefghijklmnopqrstuvwx"),
            concat!("team-", "xo", "xb-0123456789abcdef"),
            concat!("ci-", "AKIA", "ABCDEFGHIJKLMNOP"),
        ] {
            assert_eq!(kinds(value), ["token"], "{value:?}");
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

    /// REQ-records-006, TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host:
    /// a short all-letter host name, which may be an ordinary word, is a leak only inside
    /// a path or a URL or as `<user>@<host>`, in the write guard and in doctor's scan
    /// alike; a host name of 8 or more characters, or one holding a digit, a hyphen or a
    /// dot, is a leak anywhere. Every host name here is synthetic, never this machine's.
    #[test]
    fn a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host() {
        let short = Local::new(Some("SYNTHETICuser"), Some("pebble.local"));
        let kinds = |local: &Local, value: &str| -> Vec<&'static str> {
            find(local, value)
                .iter()
                .map(|found| found.kind.name())
                .collect()
        };

        for word in [
            "a pebble in the shoe",
            "Pebble, then gravel.",
            "Leif: \"keep the pebble\" (coordinator chat, 2026-11-12)",
        ] {
            // The write guard: a value and a whole record are written.
            check(&short, "lessons", word).unwrap();
            let mut record = crate::markdown::Record::new(&format!("## Lessons\n\n- {word}\n"));
            record
                .set(
                    "relay",
                    crate::markdown::Value::Maps(vec![vec![("where".into(), word.into())]]),
                )
                .unwrap();
            record.check_leaks(&short).unwrap();
            // Doctor's scan: no finding in a record line or a case definition line.
            assert_eq!(kinds(&short, word), Vec::<&str>::new(), "{word:?}");
            assert!(
                find_in_case_definition(&short, &format!("title = {word:?}")).is_empty(),
                "{word:?}"
            );
        }

        for placed in [
            "mount /srv/pebble/data",
            "notes in pebble/notes.md",
            r"share \\pebble\docs",
            "open https://pebble:8080/status",
            "see https://example.com/search?q=pebble",
            "ssh bob@pebble",
            "built on pebble.local today",
        ] {
            assert_eq!(kinds(&short, placed), ["host-name"], "{placed:?}");
            let error = check(&short, "lessons", placed).unwrap_err().to_string();
            assert!(
                error.starts_with("refused:leak: lessons holds host-name"),
                "{error}"
            );
            assert_eq!(
                find_in_case_definition(&short, placed).len(),
                1,
                "{placed:?}"
            );
        }
        assert_eq!(
            redact(&short, "ssh bob@pebble").0,
            "ssh bob@<redacted:host-name>"
        );
        // A given name becomes a committed path, so it still may not hold the name.
        assert!(check_name(&short, "key", "synthetic-pebble-key").is_err());

        // Seven letters is short; eight letters, a digit, a hyphen or a dot is not.
        let seven = Local::new(None, Some("pebbles"));
        assert_eq!(kinds(&seven, "two pebbles"), Vec::<&str>::new());
        assert_eq!(kinds(&seven, "in /srv/pebbles/x"), ["host-name"]);
        for (host, value) in [
            ("goldfish", "a goldfish bowl"),
            ("pebble2", "built on pebble2 today"),
            ("pebble-box", "built on pebble-box today"),
            ("pebble.lan", "built on pebble.lan today"),
        ] {
            let local = Local::new(None, Some(host));
            assert_eq!(kinds(&local, value), ["host-name"], "{host}");
            assert!(check(&local, "lessons", value).is_err(), "{host}");
        }
        let dotted = Local::new(None, Some("pebble.lan"));
        assert_eq!(kinds(&dotted, "a pebble"), Vec::<&str>::new());
    }

    /// REQ-records-006, TC-records-scp-style-host-path: a value holding an scp-style
    /// remote path, `<host>:/<absolute path>`, is checked like the path it carries; a home
    /// or system root after the colon refuses as a path leak, and the host part counts as
    /// the local host name when it matches, in the write guard and in doctor's scan alike.
    /// Every host name here is synthetic, never this machine's.
    #[test]
    fn an_scp_style_remote_path_is_checked_like_the_path_it_carries() {
        let local = Local::new(Some("SYNTHETICuser"), Some("brp"));
        let kinds = |local: &Local, value: &str| -> Vec<&'static str> {
            find(local, value)
                .iter()
                .map(|found| found.kind.name())
                .collect()
        };

        for placed in [
            "brp:/Users/SYNTHETIC-person/project",
            "brp:/home/SYNTHETIC-person/project",
            "ssh brp:/Users/SYNTHETIC-person/project",
        ] {
            assert_eq!(
                kinds(&local, placed),
                ["host-name", "home-path"],
                "{placed:?}"
            );
            let error = check(&local, "lessons", placed).unwrap_err().to_string();
            assert!(
                error.starts_with("refused:leak: lessons holds home-path, host-name"),
                "{error}"
            );
            assert_eq!(
                find_in_case_definition(&local, placed).len(),
                2,
                "{placed:?}"
            );
        }
        assert_eq!(
            kinds(&local, "brp:/tmp/SYNTHETIC-scratch/x"),
            ["host-name", "system-path"]
        );
        let error = check(&local, "lessons", "brp:/tmp/SYNTHETIC-scratch/x")
            .unwrap_err()
            .to_string();
        assert!(
            error.starts_with("refused:leak: lessons holds host-name, system-path"),
            "{error}"
        );
        // The host part names the local host, so it is a host-name leak even when the path
        // it names is not under a leak root; a host that is not this machine's is not, while
        // the path it names is still checked as a path.
        assert_eq!(kinds(&local, "brp:/srv/data"), ["host-name"]);
        assert_eq!(
            kinds(&local, "other:/Users/SYNTHETIC-person/x"),
            ["home-path"]
        );
        // With --redact each part is replaced, counting both of them.
        assert_eq!(
            redact(&local, "see brp:/Users/SYNTHETIC-person/project").0,
            "see <redacted:host-name>:<redacted:home-path>"
        );
    }

    /// #368: the listed forms: a quoted JSON key, `SECRET_KEY` and `AWS_SECRET_ACCESS_KEY`
    /// assignments, `xapp-` and `hf_` tokens and `~<user>/` paths, while a fraction such
    /// as `~1/3` and an ordinary `secret` word are no match.
    #[test]
    fn the_listed_forms_are_leaks() {
        let local = local();
        let kinds = |value: &str| -> Vec<&'static str> {
            find(&local, value)
                .iter()
                .map(|found| found.kind.name())
                .collect()
        };
        assert_eq!(
            kinds(concat!("{\"password\": \"", "hunter2hunter2\"}")),
            ["credential"]
        );
        assert_eq!(
            kinds(concat!("{'secret': '", "abcdefgh12345678'}")),
            ["credential"]
        );
        assert_eq!(
            kinds(concat!("SECRET_KEY = ", "abcdefgh12345678")),
            ["credential"]
        );
        assert_eq!(
            kinds(concat!(
                "export AWS_SECRET_ACCESS_KEY=",
                "wJalrXUtnFEMIK7MDENG"
            )),
            ["credential"]
        );
        assert_eq!(
            kinds(concat!("slack xapp-", "1-A0123ABCDEF-1234567890123-abcdef")),
            ["token"]
        );
        assert_eq!(
            kinds(concat!("hf_", "abcdefghijklmnopqrstuvwxyzABCDEFGH")),
            ["token"]
        );
        assert_eq!(kinds("see ~bob/projects/app"), ["home-path"]);
        assert_eq!(kinds("see ~/projects/app"), ["home-path"]);
        assert!(kinds("about ~1/3 of the runs").is_empty());
        assert!(kinds("keep the secret_key out of git").is_empty());
        assert!(kinds("the \"password\": field is documented").is_empty());
        // A short `hf_` word is no token.
        assert!(kinds("hf_short").is_empty());
    }

    /// #368: a host's `<name>.local` Bonjour name, the macOS LocalHostName given as
    /// another host and its `.local` name are host names, and a dotted host name glued
    /// into a hyphenated build name still is, while a single label stays a word of a slug.
    #[test]
    fn bonjour_names_and_dotted_hosts_in_build_names_are_host_names() {
        let mut local = Local::new(None, Some("ci-box.example.com"));
        local.add_host("Studio");
        let kinds = |value: &str| -> Vec<&'static str> {
            find(&local, value)
                .iter()
                .map(|found| found.kind.name())
                .collect()
        };
        assert_eq!(kinds("ping ci-box.local now"), ["host-name"]);
        assert_eq!(kinds("ping studio.local now"), ["host-name"]);
        assert_eq!(kinds("built on build-ci-box.example.com-1"), ["host-name"]);
        // A short all-letter LocalHostName alone is still an ordinary word.
        assert!(kinds("the Studio session").is_empty());
        assert_eq!(kinds("ssh me@studio"), ["host-name"]);
        // A single label inside a slug is a word of it, as before.
        assert!(kinds("fix-ci-box-flake").is_empty());
    }

    /// #379: a user named like a system root names no user as the first folder of an
    /// absolute path, as `/dev/disk2s1`, but still does deeper in a path or as
    /// `<user>@<host>`; every name the user goes by counts.
    #[test]
    fn a_system_root_is_no_user_name_path() {
        let mut local = Local::new(Some("dev"), None);
        local.add_user("SYNTHETICother");
        let kinds = |value: &str| -> Vec<&'static str> {
            find(&local, value)
                .iter()
                .map(|found| found.kind.name())
                .collect()
        };
        assert!(kinds("mounted /dev/disk2s1").is_empty());
        assert!(kinds("(/dev/null)").is_empty());
        assert_eq!(kinds("see /opt/dev/x"), ["user-name"]);
        assert_eq!(kinds("ssh dev@build-box"), ["user-name"]);
        assert_eq!(kinds("see /srv/SYNTHETICother/x"), ["user-name"]);
    }

    /// #367: an error shows a clean value as it is, and otherwise names what it holds,
    /// leak kinds and control or invisible formatting characters alike, never the value.
    #[test]
    fn an_unclean_value_is_named_by_what_it_holds() {
        assert_eq!(shown("bug ID", "2026-01-01-totals"), "2026-01-01-totals");
        let token = concat!("x ghp_", "abcdefghij0123456789abcdefghij0123 y");
        assert_eq!(shown("bug ID", token), "<bug ID holding token>");
        assert_eq!(
            shown("name", "a\u{1b}[31mb"),
            "<name holding control characters>"
        );
        assert_eq!(
            shown("name", "a\u{202e}b"),
            "<name holding control characters>"
        );
        assert_eq!(
            shown("name", "a\u{200b}b"),
            "<name holding control characters>"
        );
        assert_eq!(unshowable("plain words"), None);
    }

    /// #367: a usage error that quotes a value names each leak in it and each control
    /// character by what it is, and keeps everything else, line breaks included.
    #[test]
    fn a_quoted_message_is_masked() {
        let token = concat!("ghp_", "abcdefghij0123456789abcdefghij0123");
        assert_eq!(
            masked(&format!(
                "error: invalid value 'x {token} y' for '--kind'\n"
            )),
            "error: invalid value 'x <value holding token> y' for '--kind'\n"
        );
        assert_eq!(
            masked("error: unexpected argument 'a\u{1b}[31m' found"),
            "error: unexpected argument 'a<control character>[31m' found"
        );
        assert_eq!(masked("plain\ntext"), "plain\ntext");
    }
}
