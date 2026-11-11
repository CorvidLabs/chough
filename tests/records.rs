//! #235 in the 0.5.x layout, red on 0.5.0: Chough's records are JSON, not Markdown with
//! front matter. The 0.6.0 tests in this file replace it.

mod common;

use common::Fixture;

#[test]
fn every_record_is_markdown_and_keeps_human_edits() {
    fn json(root: &std::path::Path, relative: &str, out: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(root.join(relative)) else {
            return;
        };
        for entry in entries.flatten() {
            let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
            if name.ends_with("/local") {
                continue;
            }
            if entry.path().is_dir() {
                json(root, &name, out);
            } else if name.ends_with(".json") && !name.ends_with("/preferences.json") {
                out.push(name);
            }
        }
    }
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    let mut found = vec![];
    json(fixture.root(), ".chough", &mut found);
    assert!(found.is_empty(), "records written as JSON: {found:?}");
}
