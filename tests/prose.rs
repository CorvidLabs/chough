//! Bundled skill prose carries no em or en dashes, so host copies pass CorvidLabs docs gates (DRIVE-7).

use std::fs;
use std::path::Path;

fn walk(dir: &Path, found: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            walk(&path, found);
        } else if let Ok(text) = fs::read_to_string(&path) {
            for (number, line) in text.lines().enumerate() {
                if line.contains('\u{2013}') || line.contains('\u{2014}') {
                    found.push(format!("{}:{}", path.display(), number + 1));
                }
            }
        }
    }
}

#[test]
fn bundled_skill_files_contain_no_em_or_en_dashes() {
    // The bundled skill and its shipped host copy (LIFE-14).
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut found = Vec::new();
    for root in ["skills/chough-workflow", ".agents/skills/chough-workflow"] {
        walk(&manifest.join(root), &mut found);
    }
    assert!(
        found.is_empty(),
        "replace U+2013 and U+2014 with a colon, comma, parentheses or a new sentence: {found:?}"
    );
}
