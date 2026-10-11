import json
from os.path import relpath
from pathlib import Path
import subprocess
import sys
import tomllib


def check(manifest_path, accept_lint_overrides):
    try:
        output = subprocess.check_output(
            [
                "cargo",
                "metadata",
                "--frozen",
                "--no-deps",
                "--format-version=1",
                "--manifest-path",
                str(manifest_path),
            ],
            cwd=manifest_path.parent,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise RuntimeError(f"Failed to read Cargo metadata: {error}") from error

    metadata = json.loads(output)
    passed = True

    for package in metadata["packages"]:
        if package["source"] is not None:
            continue

        with open(package["manifest_path"], "rb") as manifest:
            lints = tomllib.load(manifest).get("lints", {})

        if lints.get("workspace") is True:
            continue

        if accept_lint_overrides and any(
            isinstance(value, dict) and value
            for tool, value in lints.items()
            if tool != "workspace"
        ):
            continue

        path = relpath(package["manifest_path"], metadata["workspace_root"])
        requirement = "inherit workspace lints"

        if accept_lint_overrides:
            requirement += " or define local lints"

        print(f"Crate manifest must {requirement}: {path}", file=sys.stderr)
        passed = False

    return passed


def main():
    try:
        manifest_path = Path(sys.argv[1]).resolve()
        accept_lint_overrides = sys.argv[2]

        if accept_lint_overrides not in ("true", "false"):
            raise ValueError("accept-lint-overrides must be true or false")

        return 0 if check(manifest_path, accept_lint_overrides == "true") else 1
    except (OSError, ValueError, RuntimeError) as error:
        print(error, file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
