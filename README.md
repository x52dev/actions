# x52 GitHub Actions

Reusable GitHub Actions for x52 projects.

## `full-cargo-check-external-types`

The `full-cargo-check-external-types` action installs the pinned Rust nightly toolchain and
`cargo-check-external-types`, then checks all library crates in the workspace:

```yaml
- uses: x52dev/actions/full-cargo-check-external-types@master
```

Set `all-features: true` when the workspace check must enable every crate
feature.

The action owns the Rust toolchain version. Update it in
[`full-cargo-check-external-types/action.yml`](full-cargo-check-external-types/action.yml) when the
toolchain must change. Consumers do not need to repeat the version in their
workflows.
