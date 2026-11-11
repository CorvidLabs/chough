//! SpecSync module IDs with dots and HI sub-criterion IDs (DRIVE-1, issues 56 and 57).

mod common;

use common::Fixture;
use serde_json::json;

fn init_dotted(fixture: &Fixture) {
    fixture.ok(&[
        "init",
        "--objective",
        "Keep Raven's core services in focus",
        "--intent-scope",
        "UX",
        "--module",
        "chat.service",
        "--module",
        "identity-wipe.service",
        "--module",
        "cli",
    ]);
}

#[test]
fn dotted_module_ids_and_sub_criteria_are_accepted_and_resume() {
    let fixture = Fixture::new();
    fixture.write(
        "specs/chat.service/requirements.md",
        "### REQ-chat.service-1\nSend hybrid.\n",
    );
    init_dotted(&fixture);
    fixture.ok(&[
        "checkpoint",
        "--criterion",
        "UX-12.a",
        "--criterion",
        "SEND-1.a.1.b",
        "--requirement",
        "REQ-chat.service-1",
    ]);
    let session = fixture.ok(&["context", "--json"])["session"].clone();
    assert_eq!(
        session["modules"],
        json!(["chat.service", "identity-wipe.service", "cli"])
    );
    assert_eq!(
        session["references"]["criteria"],
        json!(["UX-12.a", "SEND-1.a.1.b"])
    );
    assert_eq!(
        session["references"]["requirements"],
        json!(["REQ-chat.service-1"])
    );
    let next = fixture.run(&["next"]);
    assert!(
        next.status.success(),
        "next refused a dotted checkpoint: {}",
        String::from_utf8_lossy(&next.stderr)
    );
}

#[test]
fn a_checkpoint_written_with_dotted_modules_is_read_by_every_view() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.set_front(
        "modules",
        "[chat.service, room.service, linked-wallet.service]",
    );
    fixture.set_front("criteria", "[UX-12.a, CHAT-34.a, BOTS-4.g]");
    for args in [
        &["next"][..],
        &["status"],
        &["context", "--json"],
        &["timing"],
    ] {
        let output = fixture.run(args);
        assert!(
            output.status.success(),
            "{args:?} refused: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Plain criterion IDs Chough accepted before hi sub-criteria, such as `UX-012`, stay
/// accepted, so a checkpoint that already holds one is still read by every view.
#[test]
fn plain_criterion_ids_accepted_before_stay_accepted() {
    let fixture = Fixture::new();
    fixture.init();
    for criterion in ["UX-012", "ux-1", "UX-a", "UX-"] {
        fixture.ok(&["checkpoint", "--criterion", criterion]);
    }
    for args in [&["next"][..], &["status"], &["context", "--json"]] {
        let output = fixture.run(args);
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn ids_neither_tool_accepts_are_still_refused() {
    let fixture = Fixture::new();
    for module in ["a/b", "..", "CON", "trailing.", " spaced", "a:b"] {
        let error = fixture.fails(&[
            "init",
            "--objective",
            "x",
            "--intent-scope",
            "UX",
            "--module",
            module,
        ]);
        assert!(error.contains("module"), "{module}: {error}");
        assert!(!fixture.exists(".chough/sessions"));
    }
    fixture.init();
    let original = fixture.read(&fixture.session_file());
    for criterion in ["UX-1.A", "UX-1..a", "UX-1.a.b", "UX-1.", "UX 1", "UX-1/a"] {
        let error = fixture.fails(&["checkpoint", "--criterion", criterion]);
        assert!(
            error.contains("invalid criterion ID"),
            "{criterion}: {error}"
        );
        assert_eq!(fixture.read(&fixture.session_file()), original);
    }
    for requirement in ["REQ cli-1", "REQ-cli/1", "REQ-cli-1!", "..", "._."] {
        let error = fixture.fails(&["checkpoint", "--requirement", requirement]);
        assert!(
            error.contains("invalid requirement ID"),
            "{requirement}: {error}"
        );
    }
}

#[test]
fn bugs_link_sub_criteria_and_dotted_requirements() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.write(
        "specs/chat.service/requirements.md",
        "### REQ-chat.service-1\nSend hybrid.\n",
    );
    let bug = fixture.ok(&[
        "bug",
        "add",
        "--title",
        "Sub-criterion link",
        "--symptom",
        "A sub-criterion link is refused",
        "--trigger",
        "Linking UX-12.a",
        "--expected",
        "The link is kept",
        "--actual",
        "invalid linked ID",
        "--environment",
        "test-local",
        "--criterion",
        "UX-12.a",
        "--requirement",
        "REQ-chat.service-1",
    ]);
    let record: serde_json::Value =
        serde_json::from_str(&fixture.read(bug["recorded"].as_str().unwrap())).unwrap();
    assert_eq!(record["criteria"], json!(["UX-12.a"]));
    assert_eq!(record["requirements"], json!(["REQ-chat.service-1"]));
}

#[cfg(unix)]
#[test]
fn cases_for_a_dotted_module_are_read_and_checked() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        "specs/chat.service/requirements.md",
        "### REQ-chat.service-1\nSend hybrid.\n",
    );
    init_dotted(&fixture);
    fixture.write(
        "specs/chat.service/chat.service.spec.md",
        "# Chat service\n",
    );
    fixture.write(
        ".chough/cases/chat.service.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"chat.service\"\n\n[[case]]\nid = \"TC-chat.service-sends-hybrid\"\ntitle = \"Sends hybrid\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"manual\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-chat.service-1\"]\nenvironments = [\"local\"]\nartifacts = [\"reviewer-note\"]\nsetup = [\"Fixture\"]\nsteps = [\"Look\"]\nexpected = [\"It is right\"]\n",
    );
    let listed = fixture.ok(&["cases", "list"]);
    assert_eq!(listed["cases"][0]["id"], "TC-chat.service-sends-hybrid");
    fixture.ok(&["cases", "check"]);
}

#[test]
fn a_mistaken_completed_entry_is_removed_or_corrected_in_one_call() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&[
        "checkpoint",
        "--completed",
        "Shiped the fix",
        "--completed",
        "Kept",
    ]);
    fixture.ok(&[
        "checkpoint",
        "--remove-completed",
        "Shiped the fix",
        "--completed",
        "Shipped the fix",
    ]);
    assert_eq!(
        fixture.session()["completed"],
        json!(["Kept", "Shipped the fix"])
    );
    let original = fixture.read(&fixture.session_file());
    let error = fixture.fails(&[
        "checkpoint",
        "--remove-completed",
        "Never recorded",
        "--decision",
        "Not saved either",
    ]);
    assert!(error.contains("Never recorded"), "{error}");
    assert_eq!(fixture.read(&fixture.session_file()), original);
    fixture.ok(&["checkpoint", "--remove-completed", "Kept"]);
    assert_eq!(fixture.session()["completed"], json!(["Shipped the fix"]));
}
