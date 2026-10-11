from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


ACTION_DIRECTORY = Path(__file__).resolve().parent.parent


class ConfigurationTests(unittest.TestCase):
    def fixture(self, name="pass"):
        temporary = tempfile.TemporaryDirectory(prefix="workspace lints ")
        self.addCleanup(temporary.cleanup)
        directory = Path(temporary.name)

        shutil.copytree(ACTION_DIRECTORY / "tests" / name, directory, dirs_exist_ok=True)
        shutil.copyfile(ACTION_DIRECTORY / "check.py", directory / "check.py")

        return directory

    def run_check(self, directory, accept_lint_overrides="false", working_directory="."):
        return subprocess.run(
            [
                sys.executable,
                str(directory / "check.py"),
                str(directory / working_directory / "Cargo.toml"),
                accept_lint_overrides,
            ],
            cwd=directory,
            capture_output=True,
            text=True,
        )

    def set_member_lints(self, directory, lints):
        path = directory / "member/Cargo.toml"
        package = path.read_text().split("[lints")[0]

        path.write_text(f"{lints}\n\n{package}")

    def expect_success(self, output):
        self.assertEqual(output.returncode, 0, output.stdout + output.stderr)

    def expect_failure(self, output):
        self.assertEqual(output.returncode, 1, output.stdout + output.stderr)

    def test_workspace_inheritance_by_default(self):
        self.expect_success(self.run_check(self.fixture()))

    def test_workspace_inheritance_with_overrides(self):
        self.expect_success(self.run_check(self.fixture(), "true"))

    def test_reject_local_lints_by_default(self):
        directory = self.fixture()
        self.set_member_lints(directory, "[lints.rust]\nunsafe_code = 'allow'")

        output = self.run_check(directory)

        self.expect_failure(output)
        self.assertIn("must inherit workspace lints:", output.stderr)

    def test_reject_local_lints_with_overrides_disabled(self):
        directory = self.fixture()
        self.set_member_lints(directory, "[lints.rust]\nunsafe_code = 'allow'")

        self.expect_failure(self.run_check(directory, "false"))

    def test_accept_inheritance_and_local_lints_with_overrides(self):
        directory = self.fixture()
        self.set_member_lints(directory, "[lints.rust]\nunsafe_code = 'allow'")

        self.expect_success(self.run_check(directory, "true"))

    def test_inline_local_lints_with_level_and_priority(self):
        directory = self.fixture()
        self.set_member_lints(
            directory,
            "lints = { rust = { unsafe_code = { level = 'allow', priority = 1 } } }",
        )

        self.expect_success(self.run_check(directory, "true"))

    def test_quoted_dotted_local_lint_keys(self):
        directory = self.fixture()
        self.set_member_lints(directory, "'lints'.'clippy'.'dbg_macro' = 'deny'")

        self.expect_success(self.run_check(directory, "true"))

    def test_dotted_workspace_inheritance(self):
        directory = self.fixture()
        self.set_member_lints(directory, "lints.workspace = true")

        self.expect_success(self.run_check(directory))

    def test_virtual_workspace(self):
        directory = self.fixture()
        path = directory / "Cargo.toml"
        workspace = path.read_text().split("[workspace]")[1].split("[lints]")[0]
        path.write_text(f"[workspace]{workspace}")

        self.expect_success(self.run_check(directory))

    def test_local_lints_without_workspace_lints(self):
        directory = self.fixture()
        path = directory / "Cargo.toml"
        package_and_workspace = path.read_text().split("[workspace.lints.rust]")[0]
        path.write_text(f"{package_and_workspace}[lints.rust]\nunsafe_code = 'forbid'\n")
        self.set_member_lints(directory, "[lints.rust]\nunsafe_code = 'allow'")

        self.expect_success(self.run_check(directory, "true"))

    def test_missing_member_lints(self):
        output = self.run_check(self.fixture("fail"))

        self.expect_failure(output)
        self.assertRegex(output.stderr, r"member[/\\]Cargo\.toml")

    def test_missing_lints_with_overrides(self):
        self.expect_failure(self.run_check(self.fixture("fail"), "true"))

    def test_empty_local_lint_table_with_overrides(self):
        directory = self.fixture()
        self.set_member_lints(directory, "[lints.rust]")

        self.expect_failure(self.run_check(directory, "true"))

    def test_empty_inline_lint_table_with_overrides(self):
        directory = self.fixture()
        self.set_member_lints(directory, "lints = {}")

        self.expect_failure(self.run_check(directory, "true"))

    def test_invalid_member_toml(self):
        directory = self.fixture()
        (directory / "member/Cargo.toml").write_text("invalid TOML")

        output = self.run_check(directory)

        self.expect_failure(output)
        self.assertIn("Failed to read Cargo metadata", output.stderr)

    def test_inheritance_combined_with_local_overrides(self):
        directory = self.fixture()
        self.set_member_lints(
            directory,
            "[lints]\nworkspace = true\n\n[lints.rust]\nunsafe_code = 'allow'",
        )

        self.expect_failure(self.run_check(directory, "true"))

    def test_missing_workspace_manifest(self):
        directory = self.fixture()
        (directory / "Cargo.toml").unlink()

        output = self.run_check(directory)

        self.expect_failure(output)
        self.assertIn("Failed to read Cargo metadata", output.stderr)

    def test_invalid_input_value(self):
        output = self.run_check(self.fixture(), "yes")

        self.expect_failure(output)
        self.assertIn("accept-lint-overrides must be true or false", output.stderr)

    def test_working_directory(self):
        directory = self.fixture()
        shutil.copytree(ACTION_DIRECTORY / "tests/pass", directory / "rust")
        (directory / "Cargo.toml").unlink()

        self.expect_success(self.run_check(directory, "false", "rust"))


if __name__ == "__main__":
    unittest.main()
