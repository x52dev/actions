# x52 GitHub Actions

Reusable GitHub Actions for x52 projects.

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
