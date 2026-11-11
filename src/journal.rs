//! The knowledge journal (REQ-records-011, KNOW-33): one short dated post per finished
//! piece of work, written by create-new and never rewritten, so a person's edit is kept,
//! and the Atom feed of the newest posts, regenerated from them. A post links the
//! archived story, the pull request and the archived native change; it copies no record.

use crate::identity::{create_named, dated_name, utc_date};
use crate::markdown::{self, Kind, Record, Value};
use crate::records::{local_path, now, print_json, read_bounded_record};
use anyhow::{Result, ensure};
use clap::Subcommand;
use serde_json::json;
use std::path::Path;

/// The byte bound of a post's body (REQ-records-011).
pub(super) const BODY_LIMIT: usize = 1_200;
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
        /// One of shipped, closed, intent, spec, broke or learned.
        #[arg(long)]
        kind: String,
        #[arg(long)]
        title: String,
        /// At most 1,200 bytes.
        #[arg(long)]
        body: String,
        /// A repository path that exists or an http(s) URL; repeat for more.
        #[arg(long = "link")]
        links: Vec<String>,
    },
}

/// One post, as the index and the feed list it.
pub(super) struct Post {
    /// The file name without `.md`, which is also the post's `id`.
    pub(super) name: String,
    pub(super) title: String,
    /// `YYYY-MM-DD`.
    pub(super) date: String,
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
            links,
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
            for link in &links {
                ensure!(
                    is_url(link) || local_path(root, link).is_ok_and(|path| path.exists()),
                    "--link {link:?} is neither an http(s) URL nor a repository path that exists"
                );
            }
            let labelled: Vec<Link> = links
                .iter()
                .map(|link| (link.clone(), Some(link.clone())))
                .collect();
            let post = write_post(
                root,
                &dir,
                &kind,
                &title,
                body.trim(),
                (&labelled, &links),
                None,
            )?;
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

/// Write the post for archived session `id`, read from its story: the title is the
/// objective and the body the story's first Outcome or Where it stands paragraph, cut
/// at a sentence end so the body stays within 1,200 bytes with its links.
fn post_for_story(root: &Path, id: &str, kind: &str) -> Result<Option<String>> {
    let Some(dir) = crate::library::librarian_dir(root)? else {
        return Ok(None);
    };
    let story = crate::sessions::archive_file(id);
    let text = read_bounded_record(&local_path(root, &story)?, Kind::Story.bound())?;
    let record = markdown::parse(&story, &text)?;
    let title = record
        .scalar("objective")
        .map(str::trim)
        .filter(|objective| !objective.is_empty())
        .unwrap_or(id)
        .to_string();
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
    links.push(("the story".to_string(), Some(story.clone())));
    targets.push(story.clone());
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
    let up = up(&dir);
    let room = BODY_LIMIT
        .saturating_sub(heading(&title).len())
        .saturating_sub(links_line(&links, &up).len() + 2);
    let paragraph = cut_at_sentence(&first_paragraph(&record), room);
    write_post(
        root,
        &dir,
        kind,
        &title,
        &paragraph,
        (&links, &targets),
        Some(&story),
    )
    .map(Some)
}

/// The first non-blank paragraph of the story's Outcome section, else of its Where it
/// stands section, as one line of text.
fn first_paragraph(record: &Record) -> String {
    for heading in ["Outcome", "Where it stands"] {
        let Some(section) = record.section(heading) else {
            continue;
        };
        let paragraph = section
            .lines()
            .filter(|line| !line.starts_with("## "))
            .collect::<Vec<_>>()
            .join("\n");
        if let Some(found) = paragraph
            .split("\n\n")
            .map(str::trim)
            .find(|part| !part.is_empty())
        {
            return found.split_whitespace().collect::<Vec<_>>().join(" ");
        }
    }
    String::new()
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

fn heading(title: &str) -> String {
    format!("# {title}\n\n")
}

/// The relative prefix from a post in `<dir>/journal/` back to the repository root.
fn up(dir: &str) -> String {
    "../".repeat(dir.split('/').filter(|part| !part.is_empty()).count() + 1)
}

fn is_url(link: &str) -> bool {
    link.starts_with("https://") || link.starts_with("http://")
}

/// A post's link: its label and, unless it is plain text, a repository path or an
/// http(s) URL.
type Link = (String, Option<String>);

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
    story: Option<&str>,
) -> Result<String> {
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
    let date = utc_date(now()?);
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
            kind: field("kind").unwrap_or_default(),
        });
    }
    found.sort_by(|left, right| {
        right
            .date
            .cmp(&left.date)
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
pub(super) fn feed(posts: &[Post]) -> String {
    let updated = posts
        .first()
        .map_or("1970-01-01".to_string(), |post| post.date.clone());
    let mut out = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
<feed xmlns=\"http://www.w3.org/2005/Atom\">\n\
  <title>Project journal</title>\n\
  <id>urn:chough:journal</id>\n\
  <updated>{updated}T00:00:00Z</updated>\n\
  <author><name>Chough</name></author>\n\
  <link rel=\"self\" href=\"{FEED}\"/>\n"
    );
    for post in posts.iter().take(FEED_POSTS) {
        out.push_str(&format!(
            "  <entry>\n\
    <title>{title}</title>\n\
    <id>urn:chough:journal:{name}</id>\n\
    <updated>{date}T00:00:00Z</updated>\n\
    <link href=\"{name}.md\"/>\n\
    <category term=\"{kind}\"/>\n\
  </entry>\n",
            title = xml(&post.title),
            name = xml(&post.name),
            date = post.date,
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

    #[test]
    fn the_feed_escapes_text_and_lists_at_most_twenty_posts() {
        let posts: Vec<Post> = (0..25)
            .map(|index| Post {
                name: format!("2026-11-{:02}-post", 25 - index),
                title: "A & B <c>".into(),
                date: format!("2026-11-{:02}", 25 - index),
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
