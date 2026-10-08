# Contract: `trcli audit, telemetry`

**Spec**: [Foundation and Architecture](../spec.md) — User Story 6; FR-046 to FR-054

**Conventions**: [grammar, global options, shared verbs](./cli-conventions.md) · [output and exit codes](./output-and-exit-codes.md)

## Synopsis

```text
trcli audit list [--record <ref>] [--kind <kind>] [--actor <text>] [--action <action>]
                 [--from <date>] [--to <date>] [--limit <n>]
trcli audit verify
trcli audit export [filters] --to <file> [--format <markdown|json|csv>]
trcli telemetry show [--experiment <ref>]
trcli telemetry on | off | status
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `audit list` | Entries matching every filter given, newest first | 2 when `--to` is before `--from` |
| `audit verify` | Checks that the trail has not been altered outside the tool | 6 `check_failed`, naming the first entry that does not match |
| `audit export` | Writes the matching entries as a report | 5 when `--to` exists and `--yes` is not given |
| `telemetry show` | Run counts, success rate, durations per step, feature usage | — |
| `telemetry on` / `off` | Switches local collection; nothing is ever sent | — |

## Values

| Value | Rule |
|-------|------|
| `--action` | `create`, `update`, `delete`, `import`, `export`, `run`, `status`, `confirm` |
| `--kind` | any record kind |
| note | `audit` has no `add`, `edit`, or `rm` |

## Example

```console
$ trcli audit list --record ref-7k3f --limit 2
SEQ   WHEN                 ACTOR   ACTION   WHAT
 18   2026-10-08 14:02     ana     update   ref-7k3f status: to_read → reading
 12   2026-10-08 13:40     ana     create   ref-7k3f "Attention Is All You Need"
```
