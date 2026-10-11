# x52 GitHub Actions

Reusable GitHub Actions for x52 projects.

## `check-workspace-lints`

The `check-workspace-lints` action checks that each local crate inherits workspace
lints:

```yaml
- uses: x52dev/actions/check-workspace-lints@main
```

Each crate must set `[lints] workspace = true`. Set `accept-lint-overrides: true`
to also accept nonempty local lint tables, such as `[lints.rust]` or
`[lints.clippy]`. This input defaults to `false`. Empty or missing lint tables
fail in both modes.

The runner must have Cargo and Python 3.11 or later installed. The action uses
Python's standard-library TOML parser. It has no build or package installation
step.

Set `working-directory` when the workspace is in a subdirectory. See the
[action documentation](check-workspace-lints/README.md) for examples.

The `result` output is `success` when the check passes and `failure` when it fails.
A failed check fails the action.

## `cargo-workspace-check-external-types`

The `cargo-workspace-check-external-types` action installs the pinned Rust nightly toolchain and
`cargo-check-external-types`, then checks all library crates in the workspace:

```yaml
- uses: x52dev/actions/cargo-workspace-check-external-types@master
```

Set `all-features: true` when the workspace check must enable every crate
feature.

The action owns the Rust toolchain version. Update it in
[`cargo-workspace-check-external-types/action.yml`](cargo-workspace-check-external-types/action.yml) when the
toolchain must change. Consumers do not need to repeat the version in their
workflows.
