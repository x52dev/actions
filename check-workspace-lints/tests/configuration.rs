use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

use tempfile::TempDir;

fn fixture(name: &str) -> TempDir {
    let directory = tempfile::tempdir().unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(name);

    for file in [
        "Cargo.toml",
        "Cargo.lock",
        "src/lib.rs",
        "member/Cargo.toml",
        "member/src/lib.rs",
    ] {
        let destination = directory.path().join(file);

        fs::create_dir_all(destination.parent().unwrap()).unwrap();
        fs::copy(source.join(file), destination).unwrap();
    }

    directory
}

fn run(directory: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_check-workspace-lints"))
        .current_dir(directory)
        .arg(directory.join("Cargo.toml"))
        .output()
        .unwrap()
}

fn set_member_lints(directory: &Path, lints: &str) {
    let path = directory.join("member/Cargo.toml");
    let contents = fs::read_to_string(&path).unwrap();
    let package = contents.split("[lints").next().unwrap();

    fs::write(path, format!("{lints}\n\n{package}")).unwrap();
}

#[test]
fn accepts_inheritance_and_a_local_lint_set_in_one_workspace() {
    let directory = fixture("pass");
    let output = run(directory.path());

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn accepts_inline_local_lints_with_a_level_and_priority() {
    let directory = fixture("pass");

    set_member_lints(
        directory.path(),
        "lints = { rust = { unsafe_code = { level = 'allow', priority = 1 } } }",
    );

    let output = run(directory.path());

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn accepts_quoted_dotted_local_lint_keys() {
    let directory = fixture("pass");

    set_member_lints(directory.path(), "'lints'.'clippy'.'dbg_macro' = 'deny'");

    let output = run(directory.path());

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn accepts_dotted_workspace_inheritance() {
    let directory = fixture("pass");

    set_member_lints(directory.path(), "lints.workspace = true");

    let output = run(directory.path());

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn accepts_a_virtual_workspace() {
    let directory = fixture("pass");
    let path = directory.path().join("Cargo.toml");
    let contents = fs::read_to_string(&path).unwrap();
    let workspace = contents.split_once("[workspace]").unwrap().1;
    let workspace = workspace.split("[lints]").next().unwrap();

    fs::write(path, format!("[workspace]{workspace}")).unwrap();

    let output = run(directory.path());

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn accepts_local_lints_without_workspace_lints() {
    let directory = fixture("pass");
    let path = directory.path().join("Cargo.toml");
    let contents = fs::read_to_string(&path).unwrap();
    let package_and_workspace = contents.split("[workspace.lints.rust]").next().unwrap();

    fs::write(
        path,
        format!("{package_and_workspace}[lints.rust]\nunsafe_code = 'forbid'\n"),
    )
    .unwrap();

    let output = run(directory.path());

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn rejects_a_member_without_lints_and_reports_its_path() {
    let directory = fixture("fail");
    let output = run(directory.path());

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("member/Cargo.toml"));
}

#[test]
fn rejects_an_empty_local_lint_table() {
    let directory = fixture("pass");

    set_member_lints(directory.path(), "[lints.rust]");

    let output = run(directory.path());

    assert!(!output.status.success());
}

#[test]
fn rejects_an_empty_inline_lint_table() {
    let directory = fixture("pass");

    set_member_lints(directory.path(), "lints = {}");

    let output = run(directory.path());

    assert!(!output.status.success());
}

#[test]
fn rejects_invalid_member_toml() {
    let directory = fixture("pass");

    fs::write(directory.path().join("member/Cargo.toml"), "invalid TOML").unwrap();

    let output = run(directory.path());

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Failed to read Cargo metadata"));
}

#[test]
fn rejects_inheritance_with_local_overrides() {
    let directory = fixture("pass");

    set_member_lints(
        directory.path(),
        "[lints]\nworkspace = true\n\n[lints.rust]\nunsafe_code = 'allow'",
    );

    let output = run(directory.path());

    assert!(!output.status.success());
}

#[test]
fn rejects_a_missing_workspace_manifest() {
    let directory = fixture("pass");

    fs::remove_file(directory.path().join("Cargo.toml")).unwrap();

    let output = run(directory.path());

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Failed to read Cargo metadata"));
}
