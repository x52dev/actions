use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
};

use serde::Deserialize;

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<Package>,
    workspace_root: PathBuf,
}

#[derive(Deserialize)]
struct Package {
    manifest_path: PathBuf,
    source: Option<String>,
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(err) => {
            eprintln!("{err}");

            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<bool, Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let manifest_path = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("Cargo.toml"));
    let accept_lint_overrides = args
        .next()
        .unwrap_or_else(|| "false".into())
        .to_str()
        .ok_or("accept-lint-overrides must be true or false")?
        .parse::<bool>()
        .map_err(|_| "accept-lint-overrides must be true or false")?;

    check(&manifest_path, accept_lint_overrides)
}

fn check(manifest_path: &Path, accept_lint_overrides: bool) -> Result<bool, Box<dyn Error>> {
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--frozen",
            "--no-deps",
            "--format-version=1",
            "--manifest-path",
        ])
        .arg(manifest_path)
        .stderr(Stdio::inherit())
        .output()
        .map_err(|err| format!("Failed to run Cargo metadata: {err}"))?;

    if !output.status.success() {
        return Err("Failed to read Cargo metadata".into());
    }

    let metadata = serde_json::from_slice::<Metadata>(&output.stdout)?;
    let mut passed = true;

    for package in metadata
        .packages
        .iter()
        .filter(|package| package.source.is_none())
    {
        let contents = fs::read_to_string(&package.manifest_path)
            .map_err(|err| format!("Failed to read {}: {err}", package.manifest_path.display()))?;
        let manifest = toml::from_str::<toml::Value>(&contents)
            .map_err(|err| format!("Failed to parse {}: {err}", package.manifest_path.display()))?;

        let configured = manifest
            .get("lints")
            .and_then(toml::Value::as_table)
            .is_some_and(|lints| {
                lints.get("workspace").and_then(toml::Value::as_bool) == Some(true)
                    || accept_lint_overrides
                        && lints.iter().any(|(tool, value)| {
                            tool != "workspace"
                                && value.as_table().is_some_and(|table| !table.is_empty())
                        })
            });

        if !configured {
            let path = package
                .manifest_path
                .strip_prefix(&metadata.workspace_root)
                .unwrap_or(&package.manifest_path);

            let requirement = if accept_lint_overrides {
                "inherit workspace lints or define local lints"
            } else {
                "inherit workspace lints"
            };

            eprintln!("Crate manifest must {requirement}: {}", path.display());

            passed = false;
        }
    }

    Ok(passed)
}
