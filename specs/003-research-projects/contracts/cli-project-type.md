# Contract: `trcli project-type`

**Spec**: [Research Projects, Milestones, and Tasks](../spec.md) — User Story 6; FR-045 to FR-048

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli project-type list | show <name>
trcli project-type add <name> --description <text> [--degree] [--detail <name>]...
trcli project-type milestones <name> --set "<title>@<position>"... | --restore
trcli project-type rm <name>
trcli project retype <project-ref> <type>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `project-type add` | Defines a custom type | 2 when the name is already used |
| `project-type milestones` | Sets the milestones a type proposes; applies to projects created afterwards | 2 on a position outside 0–100 |
| `project-type milestones --restore` | Restores a built-in type's original proposal | 2 on a custom type |
| `project-type rm` | Removes a custom type | 5 `blocked_by_dependents` listing the projects that use it; 2 on a built-in type |
| `project retype` | Changes a project's type; shows the effect, keeps existing milestones, offers the new type's | 5 `confirmation_required` |

## Values

| Value | Rule |
|-------|------|
| `<position>` | where in the project's duration the milestone typically falls, as a percentage 0–100 |

## Example

```console
$ trcli project-type milestones masters --set "Coursework completed@40" --set "Qualifying@55" --set "Dissertation submitted@92" --set "Defended@100"
Updated the milestones proposed for Master's (4). Existing projects are unchanged.
```
