# x52 GitHub Actions

Reusable GitHub Actions for x52 projects.

## `setup-external-types`

The `setup-external-types` action installs the pinned Rust nightly toolchain,
`just`, and `cargo-check-external-types` for external-types checks:

```yaml
- uses: x52dev/actions/setup-external-types@master

- name: Check external types
  run: just check-external-types
```

The action owns the Rust toolchain version. Update it in
[`setup-external-types/action.yml`](setup-external-types/action.yml) when the
toolchain must change. Consumers do not need to repeat the version in their
workflows.
