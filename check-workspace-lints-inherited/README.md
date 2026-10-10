# Check Workspace Lints Inherited

This action checks that each local crate in a Rust workspace inherits lints from
the workspace manifest:

```toml
[lints]
workspace = true
```

The workspace root must define `[workspace.lints]`. The runner must have `cargo`
and `jq` installed.

```yaml
- uses: x52dev/actions/check-workspace-lints-inherited@main
```

Set `working-directory` when the workspace is in a subdirectory:

```yaml
- uses: x52dev/actions/check-workspace-lints-inherited@main
  with:
    working-directory: rust
```

The `result` output is `success` when the check passes and `failure` when it fails.
A failed check fails the action. Set `continue-on-error: true` to use the output
after a failure:

```yaml
- uses: x52dev/actions/check-workspace-lints-inherited@main
  id: lints
  continue-on-error: true

- name: Show lint check result
  env:
    RESULT: ${{ steps.lints.outputs.result }}
  run: echo "$RESULT"
```
