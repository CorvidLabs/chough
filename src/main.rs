use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const SESSION: &str = ".chough/session.json";
const LIMIT: usize = 16_384;
const RECORD_LIMIT: u64 = 65_536;
const CONTEXT_LIMIT: usize = 131_072;
const SKILL_FILES: &[(&str, &str)] = &[
    (
        "SKILL.md",
        include_str!("../skills/chough-workflow/SKILL.md"),
    ),
    (
        "references/lifecycle.md",
        include_str!("../skills/chough-workflow/references/lifecycle.md"),
    ),
    (
        "references/bugs.md",
        include_str!("../skills/chough-workflow/references/bugs.md"),
    ),
    (
        "agents/openai.yaml",
        include_str!("../skills/chough-workflow/agents/openai.yaml"),
    ),
];

#[derive(Parser)]
#[command(
    version,
    about = "Focused context and revision-bound observations; native tools own lifecycle state"
)]
struct Cli {
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a portable skill and editable session without overwriting existing files.
    Init {
        #[arg(
            long,
            default_value = "Describe the current objective before beginning work."
        )]
        objective: String,
        #[arg(long, default_value = "")]
        intent_scope: String,
        #[arg(long = "module")]
        modules: Vec<String>,
    },
    /// Assemble bounded, deterministic session context from native sources.
    Context {
        #[arg(long)]
        json: bool,
    },
    /// Report observations and native SpecSync status without claiming approval.
    Status {
        #[arg(long)]
        environment: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Show session guidance together with the native next-action authority.
    Next {
        #[arg(long)]
        environment: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Record an observation; never executes a test or authenticates an actor.
    Evidence {
        #[command(subcommand)]
        command: EvidenceCommand,
    },
}

#[derive(Subcommand)]
enum EvidenceCommand {
    Add(EvidenceArgs),
}

#[derive(clap::Args)]
struct EvidenceArgs {
    id: String,
    #[arg(long = "criterion", required = true)]
    criteria: Vec<String>,
    #[arg(long = "requirement", required = true)]
    requirements: Vec<String>,
    #[arg(long = "contract", required = true)]
    contracts: Vec<String>,
    #[arg(long, value_enum)]
    kind: Kind,
    #[arg(long, value_enum)]
    outcome: Outcome,
    #[arg(long)]
    environment: String,
    #[arg(long)]
    summary: String,
    #[arg(long = "artifact")]
    artifacts: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
enum Kind {
    ProductTest,
    Structural,
    HumanObservation,
    DeliveryObservation,
}

#[derive(Clone, Debug, Deserialize, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
enum Outcome {
    Passed,
    Failed,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Session {
    schema_version: u32,
    objective: String,
    intent_scope: String,
    modules: Vec<String>,
    decisions: Vec<String>,
    open_questions: Vec<String>,
    next_action: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    revision: String,
    dirty: bool,
    fingerprint: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Evidence {
    schema_version: u32,
    id: String,
    criteria: Vec<String>,
    requirements: Vec<String>,
    contracts: Vec<String>,
    kind: Kind,
    outcome: Outcome,
    environment: String,
    summary: String,
    artifacts: Vec<String>,
    artifact_fingerprints: BTreeMap<String, String>,
    observed_at_unix_seconds: u64,
    snapshot: Snapshot,
    provenance: String,
}

#[derive(Serialize)]
struct Section {
    source: String,
    state: String,
    content: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.root.canonicalize().context("project root must exist")?;
    ensure!(root.is_dir(), "project root must be a directory");
    match cli.command {
        Commands::Init {
            objective,
            intent_scope,
            modules,
        } => init(&root, objective, intent_scope, modules)?,
        Commands::Context { json: as_json } => {
            let session = read_session(&root)?;
            let sections = context(&root, &session)?;
            if as_json {
                print_json(
                    &json!({"schema_version": 1, "session": session, "sections": sections}),
                )?;
            } else {
                println!(
                    "# Chough context\n\n## Session\n\n{}",
                    serde_json::to_string_pretty(&session)?
                );
                for section in sections {
                    println!(
                        "\n## {} [{}]\n\n{}",
                        section.source, section.state, section.content
                    );
                }
            }
        }
        Commands::Status {
            environment,
            json: as_json,
        }
        | Commands::Next {
            environment,
            json: as_json,
        } => {
            let report = status(&root, environment.as_deref())?;
            if as_json {
                print_json(&report)?;
            } else {
                println!("{}", serde_json::to_string_pretty(&report)?);
            }
        }
        Commands::Evidence {
            command: EvidenceCommand::Add(args),
        } => add_evidence(&root, args)?,
    }
    Ok(())
}

fn print_json(value: &Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn valid_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 100
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

// Reject even in-project symlinks so later retargeting cannot redirect a write.
fn local_path(root: &Path, relative: &str) -> Result<PathBuf> {
    let relative = Path::new(relative);
    ensure!(!relative.as_os_str().is_empty(), "empty source path");
    let mut path = root.to_path_buf();
    for component in relative.components() {
        match component {
            Component::Normal(part) => path.push(part),
            _ => bail!(
                "path must be relative without traversal: {}",
                relative.display()
            ),
        }
        if let Ok(metadata) = fs::symlink_metadata(&path) {
            ensure!(
                !metadata.file_type().is_symlink(),
                "symlink is not an allowed source: {}",
                path.display()
            );
        }
    }
    Ok(path)
}

fn write_new(root: &Path, relative: &str, content: &str) -> Result<bool> {
    ensure!(
        content.len() as u64 <= RECORD_LIMIT,
        "managed file exceeds 65536 bytes: {relative}"
    );
    let path = local_path(root, relative)?;
    if path.exists() {
        ensure!(path.is_file(), "expected a file: {}", path.display());
        return Ok(false);
    }
    fs::create_dir_all(path.parent().context("file has no parent")?)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(true)
}

fn init(root: &Path, objective: String, intent_scope: String, modules: Vec<String>) -> Result<()> {
    ensure!(!objective.trim().is_empty(), "objective must not be blank");
    ensure!(
        modules.iter().all(|module| valid_name(module)),
        "module names use letters, digits, hyphens, underscores"
    );
    let session = Session {
        schema_version: 1,
        objective,
        intent_scope,
        modules,
        decisions: vec![],
        open_questions: vec![],
        next_action: "Read scoped intent and native status, then choose the next action within the authorized scope.".into(),
    };
    validate_session(&session)?;
    let mut files = vec![(
        SESSION.into(),
        serde_json::to_string_pretty(&session)? + "\n",
    )];
    files.extend(SKILL_FILES.iter().map(|(name, content)| {
        (
            format!(".agents/skills/chough-workflow/{name}"),
            (*content).to_string(),
        )
    }));
    for (name, _) in &files {
        local_path(root, name)?;
    }
    let mut created = vec![];
    let mut preserved = vec![];
    for (name, content) in files {
        if write_new(root, &name, &content)? {
            created.push(name);
        } else {
            preserved.push(name);
        }
    }
    print_json(&json!({"schema_version": 1, "created": created, "preserved": preserved}))
}

fn read_session(root: &Path) -> Result<Session> {
    let path = local_path(root, SESSION)?;
    let session: Session =
        serde_json::from_str(&read_record(&path).context("run chough init first")?)
            .context("invalid session record")?;
    validate_session(&session)?;
    Ok(session)
}

fn validate_session(session: &Session) -> Result<()> {
    ensure!(
        session.schema_version == 1,
        "unsupported session schema_version"
    );
    ensure!(
        !session.objective.trim().is_empty(),
        "session objective must not be blank"
    );
    ensure!(
        session.modules.iter().all(|module| valid_name(module)),
        "invalid session module name"
    );
    ensure!(
        session.modules.len() <= 16,
        "select at most 16 modules for focused context"
    );
    ensure!(
        serde_json::to_vec(session)?.len() as u64 <= RECORD_LIMIT,
        "session exceeds 65536 bytes"
    );
    Ok(())
}

fn read_record(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut content = String::new();
    file.take(RECORD_LIMIT + 1).read_to_string(&mut content)?;
    ensure!(
        content.len() as u64 <= RECORD_LIMIT,
        "structured record exceeds 65536 bytes: {}",
        path.display()
    );
    Ok(content)
}

fn bounded(value: &[u8]) -> String {
    let mut result = String::from_utf8_lossy(&value[..value.len().min(LIMIT)]).into_owned();
    if value.len() > LIMIT {
        result.push_str("\n[truncated at 16384 bytes; read the source when relevant]");
    }
    result
}

fn file_section(root: &Path, name: &str) -> Result<Section> {
    let path = local_path(root, name)?;
    Ok(match File::open(&path) {
        Ok(file) => {
            let mut bytes = vec![];
            file.take((LIMIT + 1) as u64).read_to_end(&mut bytes)?;
            Section {
                source: name.into(),
                state: "present".into(),
                content: bounded(&bytes),
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Section {
            source: name.into(),
            state: "missing".into(),
            content: "Source is missing; do not infer its contents.".into(),
        },
        Err(error) => return Err(error).with_context(|| format!("reading {}", path.display())),
    })
}

fn native(root: &Path, program: &str, arguments: &[&str]) -> Section {
    match Command::new(program)
        .args(arguments)
        .current_dir(root)
        .output()
    {
        Ok(output) => Section {
            source: format!("{} {}", program, arguments.join(" ")),
            state: if output.status.success() {
                "observed"
            } else {
                "failed"
            }
            .into(),
            content: if output.status.success() {
                bounded(&output.stdout)
            } else {
                format!(
                    "exit={:?}\n{}\n{}",
                    output.status.code(),
                    bounded(&output.stdout),
                    bounded(&output.stderr)
                )
            },
        },
        Err(error) => Section {
            source: format!("{} {}", program, arguments.join(" ")),
            state: "unavailable".into(),
            content: format!("Unknown: {error}"),
        },
    }
}

fn hi_export(root: &Path, scope: &str) -> Section {
    if scope.is_empty() {
        native(root, "hi", &["export"])
    } else {
        native(root, "hi", &["export", "--", scope])
    }
}

fn context(root: &Path, session: &Session) -> Result<Vec<Section>> {
    let mut sections = vec![];
    let mut ancestors: Vec<_> = root.ancestors().collect();
    ancestors.reverse();
    for ancestor in ancestors {
        if ancestor.join("AGENTS.md").exists() {
            let mut section = file_section(ancestor, "AGENTS.md")?;
            section.source = ancestor.join("AGENTS.md").display().to_string();
            sections.push(section);
        }
    }
    sections.push(file_section(root, "INTENT.md")?);
    for name in ["hi/AGENTS.md", "specs/AGENTS.md"] {
        if local_path(root, name)?.is_file() {
            sections.push(file_section(root, name)?);
        }
    }
    sections.push(hi_export(root, &session.intent_scope));
    let modules: BTreeSet<_> = session.modules.iter().collect();
    ensure!(
        modules.len() <= 16,
        "select at most 16 modules for focused context"
    );
    for module in modules {
        let base = format!("specs/{module}");
        let instructions = format!("{base}/AGENTS.md");
        if local_path(root, &instructions)?.is_file() {
            sections.push(file_section(root, &instructions)?);
        }
        for name in [
            format!("{module}.spec.md"),
            "requirements.md".into(),
            "tasks.md".into(),
            "testing.md".into(),
        ] {
            sections.push(file_section(root, &format!("{base}/{name}"))?);
        }
        let lesson = format!("{base}/{module}.3md");
        if local_path(root, &lesson)?.is_file() {
            let mut section = file_section(root, &lesson)?;
            section.content = format!(
                "Native lesson source, verbatim (planes are not parsed by Chough).\n\n{}",
                section.content
            );
            sections.push(section);
        } else {
            sections.push(file_section(root, &format!("{base}/context.md"))?);
        }
    }
    let mut remaining = CONTEXT_LIMIT;
    for section in &mut sections {
        if remaining == 0 {
            section.state = "omitted-budget".into();
            section.content =
                "Source omitted: aggregate source content budget is 131072 bytes.".into();
        } else if section.content.len() > remaining {
            let mut boundary = remaining;
            while !section.content.is_char_boundary(boundary) {
                boundary -= 1;
            }
            section.content.truncate(boundary);
            section
                .content
                .push_str("\n[truncated: aggregate source content budget is 131072 bytes]");
            section.state = "truncated-budget".into();
            remaining = 0;
        } else {
            remaining -= section.content.len();
        }
    }
    Ok(sections)
}

fn git(root: &Path, arguments: &[&str]) -> Result<Output> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .context("git is required for evidence")?;
    ensure!(
        output.status.success(),
        "git {} failed: {}",
        arguments.join(" "),
        bounded(&output.stderr)
    );
    Ok(output)
}

fn snapshot(root: &Path) -> Result<Snapshot> {
    let actual_root = git(root, &["rev-parse", "--show-toplevel"])?;
    ensure!(
        Path::new(std::str::from_utf8(&actual_root.stdout)?.trim()).canonicalize()? == root,
        "--root must be the Git repository root for evidence"
    );
    let revision = String::from_utf8(git(root, &["rev-parse", "HEAD"])?.stdout)?
        .trim()
        .to_string();
    let list = git(
        root,
        &[
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ],
    )?;
    let mut files = BTreeSet::new();
    for bytes in list
        .stdout
        .split(|byte| *byte == 0)
        .filter(|bytes| !bytes.is_empty())
    {
        let name =
            std::str::from_utf8(bytes).context("non-UTF-8 repository paths are unsupported")?;
        if !name.starts_with(".chough/") && !name.starts_with("target/") {
            files.insert(name);
        }
    }
    let mut hash = Sha256::new();
    hash.update(revision.as_bytes());
    for name in files {
        hash.update((name.len() as u64).to_le_bytes());
        hash.update(name.as_bytes());
        let path = root.join(name);
        if let Some(parent) = Path::new(name)
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            local_path(root, parent.to_str().context("non-UTF-8 source parent")?)?;
        }
        match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                hash.update(b"symlink");
                hash.update(fs::read_link(&path)?.as_os_str().as_encoded_bytes());
            }
            Ok(metadata) if metadata.is_file() => {
                hash.update(b"file");
                hash.update([u8::from(metadata.permissions().readonly())]);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    hash.update((metadata.permissions().mode() & 0o111).to_le_bytes());
                }
                hash.update(metadata.len().to_le_bytes());
                let mut file = File::open(&path)?;
                let mut buffer = [0_u8; 8192];
                loop {
                    let size = file.read(&mut buffer)?;
                    if size == 0 {
                        break;
                    }
                    hash.update(&buffer[..size]);
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => hash.update(b"missing"),
            Ok(_) => bail!("unsupported Git input: {name}"),
            Err(error) => return Err(error.into()),
        }
    }
    let dirty_output = git(root, &["status", "--porcelain", "--untracked-files=all"])?;
    let dirty = String::from_utf8(dirty_output.stdout)?.lines().any(|line| {
        line.get(3..)
            .is_some_and(|name| !name.starts_with(".chough/") && !name.starts_with("target/"))
    });
    Ok(Snapshot {
        revision,
        dirty,
        fingerprint: format!("{:x}", hash.finalize()),
    })
}

fn find_id(value: &Value, id: &str) -> bool {
    match value {
        Value::Object(object) => {
            object.get("id").and_then(Value::as_str) == Some(id)
                || object
                    .iter()
                    .filter(|(key, _)| key.as_str() != "retired")
                    .any(|(_, value)| find_id(value, id))
        }
        Value::Array(values) => values.iter().any(|value| find_id(value, id)),
        _ => false,
    }
}

fn file_digest(root: &Path, relative: &str) -> Result<String> {
    let path = local_path(root, relative)?;
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 8192];
    loop {
        let size = file.read(&mut buffer)?;
        if size == 0 {
            break;
        }
        hash.update(&buffer[..size]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn add_evidence(root: &Path, args: EvidenceArgs) -> Result<()> {
    read_session(root)?;
    ensure!(valid_name(&args.id), "invalid evidence ID");
    ensure!(
        !args.environment.trim().is_empty() && !args.summary.trim().is_empty(),
        "environment and summary must not be blank"
    );
    let mut exports = BTreeMap::new();
    for criterion in &args.criteria {
        let family = criterion
            .split_once('-')
            .context("criterion must be a HI ID")?
            .0;
        ensure!(valid_name(family), "invalid criterion family");
        if !exports.contains_key(family) {
            let export = hi_export(root, family);
            ensure!(
                export.state == "observed",
                "cannot validate criteria: {}",
                export.content
            );
            let value: Value = serde_json::from_str(&export.content)
                .context("native HI family export is invalid or exceeds 16384 bytes")?;
            exports.insert(family.to_string(), value);
        }
        ensure!(
            find_id(&exports[family], criterion),
            "unknown or retired criterion: {criterion}"
        );
    }
    let mut requirements = String::new();
    for contract in &args.contracts {
        ensure!(
            contract.starts_with("specs/") && contract.ends_with(".spec.md"),
            "contract must be a native specs/*.spec.md path"
        );
        let path = local_path(root, contract)?;
        ensure!(path.is_file(), "missing contract: {contract}");
        let companion = Path::new(contract)
            .parent()
            .context("contract parent missing")?
            .join("requirements.md");
        let companion = local_path(
            root,
            companion.to_str().context("non-UTF-8 contract companion")?,
        )?;
        requirements.push_str(
            &read_record(&companion)
                .context("contract requires its native requirements.md companion")?,
        );
        requirements.push('\n');
    }
    for requirement in &args.requirements {
        ensure!(
            requirement.starts_with("REQ-")
                && requirements
                    .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                    .any(|part| part == requirement),
            "requirement not found in contract companions: {requirement}"
        );
    }
    let mut artifact_fingerprints = BTreeMap::new();
    for artifact in &args.artifacts {
        ensure!(
            local_path(root, artifact)?.is_file(),
            "missing artifact: {artifact}"
        );
        artifact_fingerprints.insert(artifact.clone(), file_digest(root, artifact)?);
    }
    ensure!(
        artifact_fingerprints.len() == args.artifacts.len(),
        "duplicate artifact paths"
    );
    let evidence = Evidence {
        schema_version: 1, id: args.id.clone(), criteria: args.criteria, requirements: args.requirements,
        contracts: args.contracts, kind: args.kind, outcome: args.outcome, environment: args.environment,
        summary: args.summary, artifacts: args.artifacts,
        artifact_fingerprints,
        observed_at_unix_seconds: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(), snapshot: snapshot(root)?,
        provenance: "unauthenticated observation; not proof of human approval, test execution, or deployment".into(),
    };
    let name = format!(".chough/evidence/{}.json", args.id);
    ensure!(
        !local_path(root, &name)?.exists(),
        "evidence ID already exists; record a new observation"
    );
    write_new(
        root,
        &name,
        &(serde_json::to_string_pretty(&evidence)? + "\n"),
    )?;
    print_json(&json!({"schema_version": 1, "recorded": name, "provenance": evidence.provenance}))
}

fn status(root: &Path, environment: Option<&str>) -> Result<Value> {
    let session = read_session(root)?;
    let current = snapshot(root);
    let mut observations = vec![];
    let directory = local_path(root, ".chough/evidence")?;
    if directory.exists() {
        let mut files: Vec<_> = fs::read_dir(directory)?.collect::<std::io::Result<_>>()?;
        files.sort_by_key(|file| file.file_name());
        for file in files {
            let name = file
                .file_name()
                .to_str()
                .context("invalid evidence filename")?
                .to_string();
            ensure!(
                name.ends_with(".json"),
                "unexpected file in evidence directory: {name}"
            );
            let path = local_path(root, &format!(".chough/evidence/{name}"))?;
            let evidence: Evidence = serde_json::from_str(&read_record(&path)?)
                .with_context(|| format!("invalid evidence record: {name}"))?;
            ensure!(
                evidence.schema_version == 1,
                "unsupported evidence schema_version: {name}"
            );
            ensure!(
                name == format!("{}.json", evidence.id)
                    && valid_name(&evidence.id)
                    && !evidence.criteria.is_empty()
                    && !evidence.requirements.is_empty()
                    && !evidence.contracts.is_empty()
                    && !evidence.environment.trim().is_empty()
                    && !evidence.summary.trim().is_empty()
                    && evidence.snapshot.fingerprint.len() == 64
                    && evidence
                        .snapshot
                        .fingerprint
                        .bytes()
                        .all(|byte| byte.is_ascii_hexdigit()),
                "invalid evidence fields: {name}"
            );
            ensure!(
                evidence.artifacts.len() == evidence.artifact_fingerprints.len(),
                "invalid artifact bindings: {name}"
            );
            let mut artifacts_current = true;
            for artifact in &evidence.artifacts {
                local_path(root, artifact)?;
                let expected = evidence
                    .artifact_fingerprints
                    .get(artifact)
                    .context("missing artifact binding")?;
                ensure!(
                    expected.len() == 64 && expected.bytes().all(|byte| byte.is_ascii_hexdigit()),
                    "invalid artifact digest: {name}"
                );
                if file_digest(root, artifact).as_ref().ok() != Some(expected) {
                    artifacts_current = false;
                }
            }
            let freshness = match &current {
                Err(_) => "unknown-revision",
                Ok(snapshot) if snapshot.fingerprint != evidence.snapshot.fingerprint => {
                    "stale-revision-or-inputs"
                }
                Ok(_) if !artifacts_current => "stale-artifact",
                Ok(_) if environment.is_none() => "environment-unchecked",
                Ok(_) if environment != Some(evidence.environment.as_str()) => "stale-environment",
                Ok(_) => "current-observation",
            };
            observations.push(json!({"record": evidence, "freshness": freshness}));
        }
    }
    let mut native_status = if local_path(root, ".specsync/config.toml")?.is_file()
        || local_path(root, ".specsync/config.json")?.is_file()
    {
        native(root, "specsync", &["change", "status", "--json"])
    } else {
        Section {
            source: "specsync change status --json".into(),
            state: "not-configured".into(),
            content: "No native SpecSync configuration found; lifecycle state is unknown.".into(),
        }
    };
    if native_status.state == "observed"
        && !serde_json::from_str::<Value>(&native_status.content)
            .is_ok_and(|value| value.is_array() || value.is_object())
    {
        native_status.state = "invalid-output".into();
        native_status.content = format!(
            "Lifecycle state is unknown: successful SpecSync output is not a complete JSON object or array.\nBounded raw output:\n{}",
            native_status.content
        );
    }
    Ok(json!({
        "schema_version": 1,
        "objective": session.objective,
        "session_next_action": session.next_action,
        "native_next_action_authority": "Read native_specsync output and follow its actual next action before changing lifecycle state.",
        "native_specsync": native_status,
        "snapshot": match &current { Ok(value) => serde_json::to_value(value)?, Err(error) => json!({"unknown": error.to_string()}) },
        "evidence_state": if observations.is_empty() { "missing" } else { "observations-present-not-acceptance" },
        "observations": observations,
        "environment": environment,
        "human_approval": "unknown; Chough does not authenticate human identity or approval",
        "delivery": "unknown; a recorded observation does not establish deployment or release",
    }))
}
