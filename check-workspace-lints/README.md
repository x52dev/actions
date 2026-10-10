# Check Workspace Lints

This action checks that each local crate in a Rust workspace inherits the
workspace lints:

```toml
[lints]
workspace = true
```

The `accept-lint-overrides` input defaults to `false`. Set it to `true` to also
accept crates that define at least one local lint:

```yaml
- uses: x52dev/actions/check-workspace-lints@main
  with:
    accept-lint-overrides: true
```

For example, a crate can define its own lint set when overrides are enabled:

```toml
[lints.rust]
unsafe_code = "forbid"
```

Empty or missing lint tables fail in both modes. The check accepts all TOML forms
that Cargo accepts, including inline tables, dotted keys, quoted keys, and
comments.

Cargo does not let a crate combine workspace lint inheritance with local lint
overrides. A crate that needs different lints must define its full local lint
set. When overrides are enabled, this action checks that a local configuration
exists; it does not compare local lints with workspace lints. The workspace root
needs `[workspace.lints]` only when a crate inherits it. Virtual workspaces are
supported.

The runner must have Rust 1.85 or newer with Cargo installed. Cargo builds the
Rust checker on first use and reuses the build on later calls. The checker's
dependencies are locked. The first build needs registry access unless those
dependencies are cached.

The checker uses `cargo metadata --frozen --no-deps` to list local crates and
validate the workspace. Cargo metadata errors fail the check.

```yaml
- uses: x52dev/actions/check-workspace-lints@main
```

Set `working-directory` when the workspace is in a subdirectory:

```yaml
- uses: x52dev/actions/check-workspace-lints@main
  with:
    working-directory: rust
```

The `result` output is `success` when the check passes and `failure` when it fails.
A failed check fails the action. Set `continue-on-error: true` to use the output
after a failure:

```yaml
- uses: x52dev/actions/check-workspace-lints@main
  id: lints
  continue-on-error: true

- name: Show lint check result
  env:
    RESULT: ${{ steps.lints.outputs.result }}
  run: echo "$RESULT"
```
