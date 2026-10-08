# Contract: `trcli due (extended)`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 2 (remaining part); FR-012

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli due [--within <span>] [--source <kind>]...
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `due` | Defined in `specs/003-research-projects`. This spec adds sources: grant reporting deadlines, approvals nearing expiry, venue calls, instrument calibrations, material expiry | — |

## Values

| Value | Rule |
|-------|------|
| `--source` | adds `grant`, `ethics`, `venue`, `instrument`, `material` to the kinds defined in 003 |

## Example

```console
$ trcli due --within 30d --source grant --source ethics
2026-10-20  grant    grt-8k3d  Interim report
2026-10-31  ethics   eth-5c1z  Approval expires
```
