# x52 GitHub Actions

Reusable GitHub Actions for x52 projects.

## `check-workspace-lints-inherited`

The `check-workspace-lints-inherited` action checks that each local crate inherits
lints from the workspace manifest:

```yaml
- uses: x52dev/actions/check-workspace-lints-inherited@main
```

The workspace root must define `[workspace.lints]`, and each local crate must set
`[lints] workspace = true`. The runner must have `cargo` and `jq` installed.

Set `working-directory` when the workspace is in a subdirectory. See the
[action documentation](check-workspace-lints-inherited/README.md) for an example.

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
