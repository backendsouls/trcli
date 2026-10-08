# Contract: `trcli project`

**Spec**: [Research Projects, Milestones, and Tasks](../spec.md) — User Stories 1 and 2; FR-001 to FR-018

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `prj`

## Synopsis

```text
trcli project add --title <text> --type <type> [--goal <text>] [--from <date>] [--to <date>]
                  [--institution <text>] [--programme <text>] [--degree <text>]
                  [--supervisor <staff-ref>] [--co-supervisor <staff-ref>]... [--committee <staff-ref>]...
                  [--funder <text>] [--grant-reference <text>] [--milestones <all|none|ask>]
trcli project use <ref> | --none
trcli project current
trcli project status <ref> <planned|active|on_hold|completed|abandoned> [--note <text>]
trcli project reopen <ref>
trcli project summary <ref>
trcli project assign <ref> <record-ref>... [--remove]
trcli project unassigned [--kind <kind>]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `project add` | Creates a project at status `planned`; proposes the type's milestones (see `--milestones`) | 2 on unknown type or `--to` before `--from` |
| `project use` | Sets the current project; later commands apply to it unless `--project <ref>` is given | 3 when not found |
| `project current` | Shows which project is current | 0 with a message when none |
| `project status` | Changes status; `completed` and `abandoned` require `--note` and make the project read-only | 2 without `--note` |
| `project assign` | Adds records to the project, or removes them with `--remove`; nothing is copied or deleted | 5 when removal would leave a record in no project and `--yes` is not given |
| `project unassigned` | Records that belong to no project | — |
| `project rm` | Lists contents; asks whether to keep the records or delete those in no other project | 5 `confirmation_required` |
| (any command, no current project, several projects) | Asks which project is meant | 2 `validation_failed` naming `--project` when it cannot ask |
| (any change in a completed or abandoned project) | Refused; offers `project reopen` | 5 |

## Values

| Value | Rule |
|-------|------|
| `--type` | `independent`, `undergraduate-research`, `capstone`, `masters`, `doctoral`, `postdoctoral`, `funded`, or a custom type |
| `--milestones` | `ask` (default on a terminal), `all`, `none` (default without a terminal) |
| global option added | `--project <ref>` and `--all-projects` on every list and add command |
| list filters | `--type`, `--status` |

## Example

```console
$ trcli project add --title "Doctorate" --type doctoral --from 2026-03-01 --to 2030-02-28 --milestones all
Added prj-1d4c "Doctorate" (Doctoral, planned)
Added 6 proposed milestones with suggested dates; confirm or change them with: trcli milestone list
```
