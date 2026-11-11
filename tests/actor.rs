//! #201 in the 0.5.x layout: no record names the acting agent. Red on 0.5.0; the 0.6.0
//! tests in this file replace it (REQ-records-007).

mod common;

use common::Fixture;

fn chough_text(root: &std::path::Path, relative: &str, out: &mut String) {
    let Ok(entries) = std::fs::read_dir(root.join(relative)) else {
        return;
    };
    for entry in entries.flatten() {
        let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
        if entry.path().is_dir() {
            chough_text(root, &name, out);
        } else if let Ok(text) = std::fs::read_to_string(entry.path()) {
            out.push_str(&text);
        }
    }
}

#[test]
fn the_override_variables_name_the_agent_model_and_terminal() {
    let fixture = Fixture::new();
    fixture.init();
    let output = fixture.run_with_env(
        &["stage", "iterate", "--note", "round"],
        &[
            ("CHOUGH_ACTOR", "SYNTHETIC-agent"),
            ("CHOUGH_ACTOR_MODEL", "SYNTHETIC-model"),
            ("CHOUGH_HC_TERMINAL", "t901"),
        ],
    );
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let mut text = String::new();
    chough_text(fixture.root(), ".chough", &mut text);
    assert!(
        text.contains("SYNTHETIC-agent") && text.contains("t901"),
        "no record names the acting agent"
    );
}
