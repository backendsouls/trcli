# Contract: `trcli audit, telemetry`

**Spec**: [Foundation and Architecture](../spec.md) — User Story 6; FR-046 to FR-054

**Conventions**: [grammar, global options, shared verbs](./cli-conventions.md) · [output and exit codes](./output-and-exit-codes.md)

## Synopsis

```text
trcli audit list [--record <ref>] [--kind <kind>] [--actor <text>] [--action <action>]
                 [--from <date>] [--to <date>] [--limit <n>]
trcli audit verify
trcli audit export [--record <ref>] [--kind <kind>] [--actor <text>] [--action <action>]
                   [--from <date>] [--until <date>] --to <file> [--format <markdown|json|csv>]
trcli telemetry show
trcli telemetry on | off | status
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `audit list` | Entries matching every filter given, newest first. `--until` is accepted as another name for `--to` | 2 when `--to` is before `--from` |
| `audit verify` | Checks that the trail has not been altered outside the tool | 6 `check_failed`, naming the first entry that does not match |
| `audit export` | Writes the matching entries as a report. Here `--to` names the file, so the last day is given with `--until`; an export is complete (no `--limit`) and is itself recorded as an `export` entry | 5 when the file exists and `--yes` is not given; 7 when it cannot be written |
| `telemetry show` | For each command used: how many times, how many succeeded, mean and longest duration. (`--experiment` and per-step figures arrive with `specs/004-experiments`.) | — |
| `telemetry status` | Whether local collection is on | — |
| `telemetry on` / `off` | Switches local collection; nothing is ever sent | — |

## Values

| Value | Rule |
|-------|------|
| `--action` | `create`, `update`, `delete`, `status`, `link`, `unlink`, `tag`, `untag`, `note`, `import`, `export`, `run`, `confirm`, `setting`, `upgrade` |
| `--from`, `--to`, `--until` | a day written `YYYY-MM-DD`, in UTC |
| `--limit` | 1 to 1000; default: the `output.page_size` setting |
| `--kind` | any record kind |
| note | `audit` has no `add`, `edit`, or `rm` |

## Example

```console
$ trcli audit list --record ref-7k3f --limit 2
SEQ   WHEN                 ACTOR   ACTION   WHAT
 18   2026-10-08 14:02     ana     update   ref-7k3f status: to_read → reading
 12   2026-10-08 13:40     ana     create   ref-7k3f "Attention Is All You Need"
```
