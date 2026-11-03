from pathlib import Path
import difflib,sys
p=Path('/Users/leif/Development/_CorvidLabs/_worktrees/chough-restate/tests/history.rs');s=p.read_text();old=s
start=s.index('    for (quote, close, mixed, cycle_mention) in [',s.index('fn real_git_post_prune_references_report_unavailable_until_exact_restoration'))
end=s.index('        let f = closed_cycles();',start)
s=s[:start]+'''    #[derive(Clone, Copy)]
    enum Reference {
        CheckpointFile,
        Directory(&'static str, &'static str, bool),
        CycleMention,
        FileSuffix(&'static str),
        FilePrefix(&'static str),
        FileProse(&'static str, &'static str),
    }
    for reference in [
        Reference::CheckpointFile,
        Reference::Directory("", "", false),
        Reference::Directory("'", "'", false),
        Reference::Directory("`", "`", false),
        Reference::Directory("\\\"", "\\\"", false),
        Reference::Directory("", ".", true),
        Reference::Directory("", ".", false),
        Reference::Directory("(", ")", false),
        Reference::Directory("(", ").", true),
        Reference::CycleMention,
        Reference::FileSuffix(".bak"),
        Reference::FileSuffix("/child"),
        Reference::FileSuffix("_copy"),
        Reference::FilePrefix("backup/"),
        Reference::FileProse("", "."),
        Reference::FileProse("\\\"", "\\\""),
        Reference::FileProse("(", ")."),
    ] {
'''+s[end:]
old_source='''        let reference_source = if quote.is_some() {
            artifact
        } else {
            ".chough/session.json"
        };'''
new_source='''        let reference_source = if matches!(reference, Reference::CheckpointFile) {
            ".chough/session.json"
        } else {
            artifact
        };
        let mixed = matches!(
            reference,
            Reference::Directory(_, _, true) | Reference::FileSuffix(_) | Reference::FilePrefix(_)
        );
        let whole_cycle = !matches!(reference, Reference::CheckpointFile | Reference::FileProse(_, _));'''
assert old_source in s;s=s.replace(old_source,new_source,1)
start=s.index('        if let Some(quote) = quote {',s.index('fn real_git_post_prune_references_report_unavailable_until_exact_restoration'))
end=s.index('        git_ok(\n            &f,\n            &["commit", "-m", "reference the retained baseline event"],',start)
s=s[:start]+'''        if matches!(reference, Reference::CheckpointFile) {
            f.ok(&["checkpoint", "--file", &target]);
            git_ok(&f, &["add", ".chough/session.json"]);
        } else {
            let reference_text = match reference {
                Reference::Directory(open, close, mixed) => {
                    let directory = format!("Retain {open}{cycle_directory}{close}\\n");
                    if mixed { format!("Retain {target}\\n{directory}") } else { directory }
                }
                Reference::CycleMention => {
                    // The candidate span cuts through a multibyte prefix.
                    format!("{}{}{}", "é".repeat(16), cycle, "é".repeat(100))
                }
                Reference::FileSuffix(suffix) => format!("Retain {target}\\nAnd {target}{suffix}\\n"),
                Reference::FilePrefix(prefix) => format!("Retain {target}\\nAnd {prefix}{target}\\n"),
                Reference::FileProse(open, close) => format!("Retain {open}{target}{close}\\n"),
                Reference::CheckpointFile => unreachable!(),
            };
            f.write(artifact, &reference_text);
            git_ok(&f, &["add", artifact]);
        }
'''+s[end:]
s=s.replace('''        let restore_path = if quote.is_some() {''','''        let restore_path = if whole_cycle {''',1)
Path('/tmp/chough-reference-right-boundary-test.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),s.splitlines(True),fromfile='a/tests/history.rs',tofile='b/tests/history.rs')))
if '--apply' in sys.argv:p.write_text(s)
print('/tmp/chough-reference-right-boundary-test.patch')
