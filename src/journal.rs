//! The knowledge journal (REQ-records-011, KNOW-33): one short dated post per finished
//! piece of work, written by create-new and never rewritten, so a person's edit is kept,
//! and the Atom feed of the newest posts, regenerated from them. A post links the
//! archived story, the pull request and the archived native change; it copies no record.

use crate::identity::{create_named, dated_name, utc_date};
use crate::markdown::{self, Kind, Record, Value};
use crate::records::{local_path, now, print_json, read_bounded_record};
use anyhow::{Result, bail, ensure};
use clap::Subcommand;
use serde_json::json;
use std::path::{Component, Path};
use std::process::Command;

/// The byte bound of a post's body (REQ-records-011).
pub(super) const BODY_LIMIT: usize = 1_200;
/// The byte bound of a post's title: a short human title (REQ-records-011).
pub(super) const TITLE_LIMIT: usize = 72;
/// The byte bound of the summary an agent gives a session's post (REQ-records-011).
pub(super) const SUMMARY_LIMIT: usize = 900;
/// How many posts the feed lists, newest first (REQ-records-011).
const FEED_POSTS: usize = 20;
/// The kinds a post may have (REQ-records-011).
const KINDS: [&str; 6] = ["shipped", "closed", "intent", "spec", "broke", "learned"];
/// The feed, beside the posts in `journal/`.
pub(super) const FEED: &str = "feed.xml";

#[derive(Subcommand)]
pub(super) enum JournalCommand {
    /// Write a post by create-new; it is never rewritten, so a person's edit is kept.
    Add {
        /// One of shipped, closed, intent, spec, broke or learned; with --record, the
        /// story's outcome sets it.
        #[arg(long, required_unless_present = "record", conflicts_with = "record")]
        kind: Option<String>,
        #[arg(long)]
        title: String,
        /// At most 1,200 bytes.
        #[arg(long)]
        body: String,
        /// A project path Git tracks or an http(s) URL; repeat for more.
        #[arg(long = "link")]
        links: Vec<String>,
        /// An archived story, by its ID: write that story's post for a backfill, dated by
        /// its close, with its links and `record`; refused when a post already records
        /// it (REQ-records-011).
        #[arg(long)]
        record: Option<String>,
    },
}

/// One post, as the index and the feed list it.
pub(super) struct Post {
    /// The file name without `.md`, which is also the post's `id`.
    pub(super) name: String,
    pub(super) title: String,
    /// `YYYY-MM-DD`.
    pub(super) date: String,
    /// The UTC time the post was written, `YYYY-MM-DDTHH:MM:SSZ`; None for a post that
    /// records none (migrated or hand-written) or an invalid one.
    pub(super) time: Option<String>,
    pub(super) kind: String,
}

/// The journal folder of the knowledge directory `dir`.
pub(super) fn dir_of(dir: &str) -> String {
    format!("{dir}/journal")
}

/// `chough journal add` (REQ-records-011).
pub(super) fn run(root: &Path, command: JournalCommand) -> Result<()> {
    match command {
        JournalCommand::Add {
            kind,
            title,
            body,
            links: given_links,
            record,
        } => {
            let dir = crate::library::enabled_dir(root)?;
            ensure!(
                body.len() <= BODY_LIMIT,
                "the body is {} bytes; a journal post holds at most {BODY_LIMIT}, so say less and link the record that holds the rest",
                body.len()
            );
            ensure!(
                !body.trim().is_empty(),
                "the body must not be blank; say what happened with --body <text>"
            );
            let links: Vec<String> = given_links.iter().map(|link| stored_link(link)).collect();
            for (given, link) in given_links.iter().zip(&links) {
                ensure!(
                    is_url(link) || tracked(root, link_path(link)),
                    "--link {given:?} is neither an http(s) URL nor a project path Git tracks"
                );
            }
            let labelled: Vec<Link> = links
                .iter()
                .map(|link| (link.clone(), Some(link.clone())))
                .collect();
            let (kind, post) = match record {
                Some(id) => post_for_record(root, &dir, &id, &title, body.trim(), &links)?,
                None => {
                    let kind = kind.unwrap_or_default();
                    let post = write_post(
                        root,
                        &dir,
                        &kind,
                        &title,
                        body.trim(),
                        (&labelled, &links),
                        None,
                    )?;
                    (kind, post)
                }
            };
            let index = crate::library::regenerate(root, &dir)?;
            print_json(&json!({
                "schema_version": 1,
                "post": post,
                "kind": kind,
                "index": index,
            }))
        }
    }
}

/// The move into done archived session `id` (REQ-history-006): write its `shipped` post.
/// Never fails: `written` with the post, `skipped` with the librarian off, or `refused`
/// with the reason; a refusal never undoes the move.
pub(super) fn post_for_done(root: &Path, id: &str) -> serde_json::Value {
    report(post_for_story(root, id, "shipped"))
}

/// `chough session close` archived session `id` without done: write its `closed` post.
pub(super) fn post_for_close(root: &Path, id: &str) -> serde_json::Value {
    report(post_for_story(root, id, "closed"))
}

fn report(result: Result<Option<String>>) -> serde_json::Value {
    match result {
        Ok(Some(post)) => json!({"status": "written", "post": post}),
        Ok(None) => json!({"status": "skipped", "reason": "the librarian is off"}),
        Err(error) => json!({"status": "refused", "reason": format!("{error:#}")}),
    }
}

/// Write the post for archived session `id`, read from its story (REQ-records-011): the
/// title is the session's `title`, else one derived from its objective; the body is its
/// `summary`, one paragraph per line, else one derived from the story; either is cut at a
/// sentence end so the body stays within 1,200 bytes with its links.
fn post_for_story(root: &Path, id: &str, kind: &str) -> Result<Option<String>> {
    let Some(dir) = crate::library::librarian_dir(root)? else {
        return Ok(None);
    };
    let (story, record) = read_story(root, id)?;
    let (links, targets) = story_links(root, &record, &story);
    let title = record
        .scalar("title")
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .map_or_else(
            || derived_title(record.scalar("objective").unwrap_or(id)),
            str::to_string,
        );
    let up = up(&dir);
    let room = BODY_LIMIT
        .saturating_sub(heading(&title).len())
        .saturating_sub(links_line(&links, &up).len() + 2);
    let summary = record.list("summary");
    let body = if summary.is_empty() {
        derived_body(&record, room)
    } else {
        cut_at_sentence(&summary.join("\n\n"), room)
    };
    write_post(
        root,
        &dir,
        kind,
        &title,
        &body,
        (&links, &targets),
        Some((&story, None)),
    )
    .map(Some)
}

/// `chough journal add --record <story>` (REQ-records-011): the post of an archived story
/// that has none, for a backfill. Its kind follows the story's outcome, its date is the
/// story's `closed` and its time the latest Timeline time on that date, if any; it links
/// what the post at the move into done links, then each `--link`.
fn post_for_record(
    root: &Path,
    dir: &str,
    id: &str,
    title: &str,
    body: &str,
    given: &[String],
) -> Result<(String, String)> {
    let (story, record) = read_story(root, id)?;
    if let Some(existing) = post_recording(root, dir, &story)? {
        bail!(
            "refused:post-exists: {existing} already records {story}, so nothing was written; Chough never rewrites a post, so edit that post by hand instead"
        );
    }
    let kind = if record.scalar("outcome").map(str::trim) == Some("done") {
        "shipped"
    } else {
        "closed"
    };
    let Some(closed) = record
        .scalar("closed")
        .map(str::trim)
        .filter(|closed| is_date(closed))
    else {
        bail!("{story} has no `closed` date, so its post cannot be dated; nothing was written");
    };
    let time = latest_time(&record).filter(|time| time.starts_with(closed));
    let (mut links, mut targets) = story_links(root, &record, &story);
    for link in given {
        links.push((link.clone(), Some(link.clone())));
        targets.push(link.clone());
    }
    let post = write_post(
        root,
        dir,
        kind,
        title,
        body,
        (&links, &targets),
        Some((&story, Some((closed.to_string(), time)))),
    )?;
    Ok((kind.to_string(), post))
}

/// Archived story `id` (given as an ID or as its path) and its record.
fn read_story(root: &Path, id: &str) -> Result<(String, Record)> {
    let id = id
        .trim()
        .trim_start_matches("./")
        .trim_start_matches(".chough/archive/sessions/")
        .trim_end_matches(".md");
    let story = crate::sessions::archive_file(id);
    let path = local_path(root, &story)?;
    ensure!(
        path.is_file(),
        "no archived story {id}: {story} does not exist, so nothing was written; name a story under .chough/archive/sessions/ by its ID"
    );
    let text = read_bounded_record(&path, Kind::Story.bound())?;
    let record = markdown::parse(&story, &text)?;
    Ok((story, record))
}

/// The links a story's post carries: the pull request when the session names one, the
/// story, and the native change's archive folder when it is archived.
fn story_links(root: &Path, record: &Record, story: &str) -> (Vec<Link>, Vec<String>) {
    let mut links: Vec<Link> = vec![];
    let mut targets = vec![];
    if let Some(pr) = record
        .scalar("pr")
        .map(str::trim)
        .filter(|pr| !pr.is_empty())
    {
        targets.push(pr.to_string());
        if is_url(pr) {
            links.push(("the pull request".to_string(), Some(pr.to_string())));
        } else {
            links.push((format!("the pull request {pr}"), None));
        }
    }
    links.push(("the story".to_string(), Some(story.to_string())));
    targets.push(story.to_string());
    if let Some(folder) = record
        .scalar("native_change")
        .map(str::trim)
        .filter(|change| !change.is_empty())
        .and_then(|change| crate::library::archive_dir_of(root, change))
    {
        let folder = format!("{}/{folder}/", crate::library::ARCHIVES);
        links.push(("the archived change".to_string(), Some(folder.clone())));
        targets.push(folder);
    }
    (links, targets)
}

/// The post in `<dir>/journal/` whose `record` names `story`, if any.
fn post_recording(root: &Path, dir: &str, story: &str) -> Result<Option<String>> {
    let journal = dir_of(dir);
    for name in crate::library::sorted_entries(root, &journal)? {
        if !name.ends_with(".md") {
            continue;
        }
        let relative = format!("{journal}/{name}");
        let Ok(text) = read_bounded_record(&local_path(root, &relative)?, Kind::Journal.bound())
        else {
            continue;
        };
        let Ok(record) = markdown::parse(&relative, &text) else {
            continue;
        };
        if record.scalar("record").map(str::trim) == Some(story) {
            return Ok(Some(relative));
        }
    }
    Ok(None)
}

/// The latest time among a story's Timeline entries, `YYYY-MM-DDTHH:MM:SSZ`.
fn latest_time(record: &Record) -> Option<String> {
    record
        .section(crate::session_md::TIMELINE)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.strip_prefix("- ")?.get(..20))
        .filter(|time| crate::session_md::parse_time(time).is_ok())
        .max()
        .map(str::to_string)
}

/// `chough checkpoint --title` (REQ-records-011): one line of at most 72 bytes, trimmed;
/// empty removes the title.
pub(super) fn checked_title(title: &str) -> Result<String> {
    let title = title.trim();
    ensure!(
        !title.contains(['\n', '\r']),
        "--title must be one line, so nothing was saved; give a short human title such as \"Chough 0.6.0: human-first records\""
    );
    ensure!(
        title.len() <= TITLE_LIMIT,
        "--title is {} bytes, over the {TITLE_LIMIT}-byte bound of a post's title, so nothing was saved; give a shorter human title",
        title.len()
    );
    Ok(title.to_string())
}

/// `chough checkpoint --summary` (REQ-records-011): its non-blank lines, trimmed, at most
/// 900 bytes in all with a line break between each; empty removes the summary.
pub(super) fn checked_summary(summary: &str) -> Result<Vec<String>> {
    let lines: Vec<String> = summary
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();
    let size = lines.iter().map(String::len).sum::<usize>() + lines.len().saturating_sub(1);
    ensure!(
        size <= SUMMARY_LIMIT,
        "--summary is {size} bytes, over the {SUMMARY_LIMIT}-byte bound of a post's summary, so nothing was saved; say less and let the story hold the rest"
    );
    Ok(lines)
}

/// A post title derived from an objective (REQ-records-011): the text before the first
/// `: `, `; `, `, `, ` (` or ` — ` that starts at least 16 bytes in, cut at a word to at
/// most 72 bytes and ending in `…` when cut.
fn derived_title(objective: &str) -> String {
    let text = objective.split_whitespace().collect::<Vec<_>>().join(" ");
    let end = [": ", "; ", ", ", " (", " — "]
        .iter()
        .filter_map(|separator| {
            text.match_indices(separator)
                .map(|(at, _)| at)
                .find(|at| *at >= 16)
        })
        .min()
        .unwrap_or(text.len());
    let clause = text[..end]
        .trim_end_matches(['.', ',', ';', ':'])
        .trim_end();
    if clause.len() <= TITLE_LIMIT {
        return clause.to_string();
    }
    let mut cut = TITLE_LIMIT - '…'.len_utf8();
    while !clause.is_char_boundary(cut) {
        cut -= 1;
    }
    let word = clause[..cut].rfind(' ').filter(|at| *at > 0).unwrap_or(cut);
    format!("{}…", clause[..word].trim_end())
}

/// A post body derived from a story (REQ-records-011): the Goal's first paragraph, then
/// `Why:` with the first Decisions and why entry, `Verified:` with the newest What was
/// verified entry and `Next:` with the first open thread, an empty part left out. Each
/// part is cut at a sentence end to its share of `room`; a short part's spare share goes
/// to the parts that need it, the Goal first.
fn derived_body(record: &Record, room: usize) -> String {
    use crate::session_md::{DECISIONS, GOAL, VERIFIED, items, open_threads};
    let decision = items(record, DECISIONS).into_iter().next().map(|decision| {
        decision
            .strip_prefix("decided: ")
            .map_or(decision.clone(), str::to_string)
    });
    // Each part: its label's length, its text with the label, and its weight.
    let parts: Vec<(usize, String, usize)> = [
        ("", Some(first_paragraph(record, GOAL)), 2),
        ("Why: ", decision, 1),
        (
            "Verified: ",
            items(record, VERIFIED).into_iter().next_back(),
            1,
        ),
        ("Next: ", open_threads(record).into_iter().next(), 1),
    ]
    .into_iter()
    .filter_map(|(label, text, weight)| {
        let text = plain(&text?);
        (!text.is_empty()).then(|| (label.len(), format!("{label}{text}"), weight))
    })
    .collect();
    let available = room.saturating_sub(2 * parts.len().saturating_sub(1));
    let weights: usize = parts.iter().map(|(_, _, weight)| weight).sum();
    let mut limits: Vec<usize> = parts
        .iter()
        .map(|(_, _, weight)| available * weight / weights.max(1))
        .collect();
    let mut spare: usize = parts
        .iter()
        .zip(&limits)
        .map(|((_, text, _), limit)| limit.saturating_sub(text.len()))
        .sum();
    for ((_, text, _), limit) in parts.iter().zip(limits.iter_mut()) {
        if text.len() > *limit {
            let more = (text.len() - *limit).min(spare);
            *limit += more;
            spare -= more;
        }
    }
    parts
        .iter()
        .zip(&limits)
        .map(|((label, text, _), limit)| (label, cut_at_sentence(text, *limit)))
        // A part cut down to its label says nothing, so it is left out.
        .filter(|(label, part)| {
            part.get(**label..)
                .is_some_and(|rest| rest.chars().any(char::is_alphanumeric))
        })
        .map(|(_, part)| part)
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// The first non-blank paragraph of section `heading`, as one line of text.
fn first_paragraph(record: &Record, heading: &str) -> String {
    let Some(section) = record.section(heading) else {
        return String::new();
    };
    let paragraph = section
        .lines()
        .filter(|line| !line.starts_with("## "))
        .collect::<Vec<_>>()
        .join("\n");
    paragraph
        .split("\n\n")
        .map(str::trim)
        .find(|part| !part.is_empty())
        .map(|found| found.split_whitespace().collect::<Vec<_>>().join(" "))
        .unwrap_or_default()
}

/// `text` as one plain line: Markdown links become their text, so a link relative to the
/// story never breaks in the post, and escaped brackets lose their backslash.
fn plain(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        let escaped = open > 0 && rest.as_bytes()[open - 1] == b'\\';
        let link = (!escaped)
            .then(|| {
                let close = open + rest[open..].find("](")?;
                let end = close + rest[close..].find(')')?;
                Some((close, end))
            })
            .flatten();
        match link {
            Some((close, end)) => {
                out.push_str(&rest[..open]);
                out.push_str(&rest[open + 1..close]);
                rest = &rest[end + 1..];
            }
            None => {
                out.push_str(&rest[..=open]);
                rest = &rest[open + 1..];
            }
        }
    }
    out.push_str(rest);
    out.replace("\\[", "[")
        .replace("\\]", "]")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// `text` cut to at most `limit` bytes at a sentence end; without one, at a word, marked
/// with an ellipsis.
fn cut_at_sentence(text: &str, limit: usize) -> String {
    if text.len() <= limit {
        return text.to_string();
    }
    let mut end = limit;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let head = &text[..end];
    let sentence = head
        .char_indices()
        .filter(|(at, character)| {
            matches!(character, '.' | '!' | '?')
                && text[at + character.len_utf8()..]
                    .chars()
                    .next()
                    .is_none_or(char::is_whitespace)
        })
        .map(|(at, character)| at + character.len_utf8())
        .next_back();
    if let Some(end) = sentence {
        return text[..end].to_string();
    }
    let room = limit.saturating_sub('…'.len_utf8());
    let mut end = room.min(head.len());
    while !head.is_char_boundary(end) {
        end -= 1;
    }
    let word = head[..end].rfind(' ').unwrap_or(end);
    format!("{}…", head[..word].trim_end())
}

/// A post's own heading: its title as inert Markdown, as the index line holds it
/// (REQ-cli-044), so a title can add neither a link nor HTML to the post.
fn heading(title: &str) -> String {
    format!("# {}\n\n", crate::vault::inert(title))
}

/// The relative prefix from a post in `<dir>/journal/` back to the repository root.
fn up(dir: &str) -> String {
    "../".repeat(dir.split('/').filter(|part| !part.is_empty()).count() + 1)
}

fn is_url(link: &str) -> bool {
    link.starts_with("https://") || link.starts_with("http://")
}

/// A `--link` as the post stores it: an http(s) URL as given, or a project path with any
/// leading `./` dropped and its `#fragment` kept, so a tracked file is accepted however a
/// person writes the relative path (REQ-cli-044).
fn stored_link(link: &str) -> String {
    if is_url(link) {
        return link.to_string();
    }
    let mut path = link;
    while let Some(rest) = path.strip_prefix("./") {
        path = rest;
    }
    path.to_string()
}

/// The path of a stored project link, before any `#fragment`: what must be tracked.
fn link_path(link: &str) -> &str {
    link.split_once('#').map_or(link, |(path, _)| path)
}

/// Whether `link` is a project path Git tracks: a file in its index, or a folder that
/// holds one. A link must resolve in every clone and on the repository host
/// (REQ-cli-044), so a path under `.git/`, one the ignore rules match (even when forced
/// into the index, as machine-local files are), an untracked path and a missing one do
/// not count, and neither does any path outside a Git repository.
fn tracked(root: &Path, link: &str) -> bool {
    if Path::new(link)
        .components()
        .any(|part| part == Component::Normal(".git".as_ref()))
        || !local_path(root, link).is_ok_and(|path| path.exists())
    {
        return false;
    }
    let git = |arguments: &[&str]| {
        crate::records::run_owned(Command::new("git").args(arguments).current_dir(root), None).ok()
    };
    // `check-ignore` exits 0 for an ignored path, and with --no-index it also reads a
    // tracked one; it refuses pathspec magic, so only `ls-files` reads the link
    // literally, never as a glob.
    let ignored = git(&["check-ignore", "--quiet", "--no-index", "--", link])
        .is_some_and(|output| output.status.success());
    !ignored
        && git(&["--literal-pathspecs", "ls-files", "-z", "--", link])
            .is_some_and(|output| output.status.success() && !output.stdout.is_empty())
}

/// A post's link: its label and, unless it is plain text, a repository path or an
/// http(s) URL.
type Link = (String, Option<String>);

/// A backfilled post's date and, when known, its time (REQ-records-011).
type Dated = (String, Option<String>);

/// `Read more:` with each link as a standard relative Markdown link (REQ-cli-044).
fn links_line(links: &[Link], up: &str) -> String {
    if links.is_empty() {
        return String::new();
    }
    let rendered: Vec<String> = links
        .iter()
        .map(|(label, target)| match target {
            Some(target) if is_url(target) => format!("[{label}]({target})"),
            Some(target) => format!("[{label}]({up}{target})"),
            None => label.clone(),
        })
        .collect();
    format!("Read more: {}\n", rendered.join(" · "))
}

/// Write one post by create-new in `<dir>/journal/<date>-<slug>.md`, with one clash
/// suffix when that name exists; an existing post is never overwritten.
fn write_post(
    root: &Path,
    dir: &str,
    kind: &str,
    title: &str,
    paragraph: &str,
    (links, targets): (&[Link], &[String]),
    story: Option<(&str, Option<Dated>)>,
) -> Result<String> {
    let (story, when) = story.map_or((None, None), |(story, when)| (Some(story), when));
    ensure!(
        KINDS.contains(&kind),
        "--kind {kind:?} is not a journal kind; use one of {}",
        KINDS.join(", ")
    );
    let title = title.trim();
    ensure!(
        !title.is_empty() && !title.contains('\n'),
        "the title must be one non-blank line"
    );
    // REQ-records-011 (Q1): a post records the time it was written; its name keeps the date.
    // A backfilled post takes its story's date and the latest time on it, or no time.
    let (date, time) = match when {
        Some(when) => when,
        None => {
            let at = now()?;
            (utc_date(at), Some(crate::session_md::iso_time(at)))
        }
    };
    let name = dated_name(&date, title, "post");
    let up = up(dir);
    let mut body = heading(title);
    if !paragraph.is_empty() {
        body.push_str(paragraph);
        body.push_str("\n\n");
    }
    body.push_str(&links_line(links, &up));
    let render = |name: &str| -> Result<String> {
        let mut record = Record::new(&body);
        record.set_scalar("id", name)?;
        record.set_scalar("title", title)?;
        record.set_scalar("type", "journal")?;
        record.set(
            "tags",
            Value::List(vec!["chough/journal".into(), format!("chough/{kind}")]),
        )?;
        record.set_scalar("date", &date)?;
        if let Some(time) = &time {
            record.set_scalar("time", time)?;
        }
        record.set_scalar("kind", kind)?;
        record.set("links", Value::List(targets.to_vec()))?;
        if let Some(story) = story {
            record.set_scalar("record", story)?;
        }
        if let Some(actor) = crate::actor::Actor::current() {
            record.set("actor", actor.front_value())?;
        }
        let local = crate::leaks::Local::current();
        if crate::leaks::redacting() {
            record.redact_leaks(local);
        } else {
            record.check_leaks(local)?;
        }
        record.render_within(Kind::Journal)
    };
    let journal = dir_of(dir);
    std::fs::create_dir_all(local_path(root, &journal)?)?;
    let name = create_named(root, &journal, &name, ".md", Kind::Journal.bound(), render)?;
    Ok(format!("{journal}/{name}.md"))
}

/// Every post in `<dir>/journal/`, newest first by date and then name. A post a person
/// broke by hand is still listed, by its file name.
pub(super) fn posts(root: &Path, dir: &str) -> Result<Vec<Post>> {
    let journal = dir_of(dir);
    let mut found = vec![];
    for name in crate::library::sorted_entries(root, &journal)? {
        let Some(stem) = name.strip_suffix(".md") else {
            continue;
        };
        let relative = format!("{journal}/{name}");
        let path = local_path(root, &relative)?;
        if !path.is_file() {
            continue;
        }
        let record = read_bounded_record(&path, Kind::Journal.bound())
            .ok()
            .and_then(|text| markdown::parse(&relative, &text).ok());
        let field = |key: &str| {
            record
                .as_ref()
                .and_then(|record| record.scalar(key))
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty() && !value.contains('\n'))
        };
        let date = field("date")
            .filter(|date| is_date(date))
            .or_else(|| {
                stem.get(..10)
                    .filter(|date| is_date(date))
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "1970-01-01".to_string());
        found.push(Post {
            name: stem.to_string(),
            title: field("title").unwrap_or_else(|| stem.to_string()),
            date,
            time: field("time").filter(|time| crate::session_md::parse_time(time).is_ok()),
            kind: field("kind").unwrap_or_default(),
        });
    }
    // Newest first: by date, then a timed post by its time before an untimed one, then by
    // name, so the order needs no file time and the same posts give the same bytes.
    found.sort_by(|left, right| {
        right
            .date
            .cmp(&left.date)
            .then_with(|| right.time.is_some().cmp(&left.time.is_some()))
            .then_with(|| right.time.cmp(&left.time))
            .then_with(|| left.name.cmp(&right.name))
    });
    Ok(found)
}

fn is_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 10
        && bytes.iter().enumerate().all(|(at, byte)| match at {
            4 | 7 => *byte == b'-',
            _ => byte.is_ascii_digit(),
        })
}

fn xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// The Atom feed of the newest 20 posts (REQ-records-011): identical bytes for the same
/// posts, since every time in it comes from a post's date.
/// When `post` was written, for the feed: its time, or its date at midnight UTC.
fn updated(post: &Post) -> String {
    post.time
        .clone()
        .unwrap_or_else(|| format!("{}T00:00:00Z", post.date))
}

pub(super) fn feed(posts: &[Post]) -> String {
    let updated_at = posts
        .first()
        .map_or("1970-01-01T00:00:00Z".to_string(), updated);
    let mut out = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
<feed xmlns=\"http://www.w3.org/2005/Atom\">\n\
  <title>Project journal</title>\n\
  <id>urn:chough:journal</id>\n\
  <updated>{updated_at}</updated>\n\
  <author><name>Chough</name></author>\n\
  <link rel=\"self\" href=\"{FEED}\"/>\n"
    );
    for post in posts.iter().take(FEED_POSTS) {
        out.push_str(&format!(
            "  <entry>\n\
    <title>{title}</title>\n\
    <id>urn:chough:journal:{name}</id>\n\
    <updated>{updated}</updated>\n\
    <link href=\"{name}.md\"/>\n\
    <category term=\"{kind}\"/>\n\
  </entry>\n",
            title = xml(&post.title),
            name = xml(&post.name),
            updated = updated(post),
            kind = xml(&post.kind),
        ));
    }
    out.push_str("</feed>\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_long_paragraph_is_cut_at_a_sentence_end() {
        let text = format!("{}. Second sentence here.", "a".repeat(20));
        assert_eq!(cut_at_sentence(&text, 30), format!("{}.", "a".repeat(20)));
        assert_eq!(cut_at_sentence("short", 30), "short");
        let words = "one two three four five six";
        let cut = cut_at_sentence(words, 14);
        assert!(cut.len() <= 14 && cut.ends_with('…'), "{cut}");
    }

    /// REQ-records-011: a derived title is the objective's first clause from 16 bytes in,
    /// cut at a word within 72 bytes.
    #[test]
    fn a_derived_title_is_the_first_clause_within_72_bytes() {
        assert_eq!(
            derived_title("Ship a focused change"),
            "Ship a focused change"
        );
        assert_eq!(
            derived_title("Prepare the Chough 0.5.0 release: version 0.5.0 and a dated change log"),
            "Prepare the Chough 0.5.0 release"
        );
        assert_eq!(
            derived_title("chough#170: a project in a subdirectory records evidence (DRIVE-18)"),
            "chough#170: a project in a subdirectory records evidence"
        );
        let long = derived_title(&"word ".repeat(30));
        assert!(
            long.len() <= TITLE_LIMIT && long.ends_with("word…"),
            "{long}"
        );
    }

    #[test]
    fn the_feed_escapes_text_and_lists_at_most_twenty_posts() {
        let posts: Vec<Post> = (0..25)
            .map(|index| Post {
                name: format!("2026-11-{:02}-post", 25 - index),
                title: "A & B <c>".into(),
                date: format!("2026-11-{:02}", 25 - index),
                time: None,
                kind: "shipped".into(),
            })
            .collect();
        let feed = feed(&posts);
        assert_eq!(feed.matches("<entry>").count(), 20);
        assert!(feed.contains("A &amp; B &lt;c&gt;"));
        assert!(feed.contains("<updated>2026-11-25T00:00:00Z</updated>"));
        assert_eq!(feed, super::feed(&posts));
    }
}
