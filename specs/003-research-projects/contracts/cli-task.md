# Contract: `trcli task`

**Spec**: [Research Projects, Milestones, and Tasks](../spec.md) — User Story 4; FR-028 to FR-039

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `tsk`

## Synopsis

```text
trcli task add <title> [--description <text>] [--priority <low|normal|high|urgent>] [--due <date>]
               [--effort <text>] [--responsible <staff-ref>] [--milestone <ref>] [--parent <ref>]
               [--after <ref>]... [--about <record-ref>]... [--repeat <interval>]
trcli task start <ref> | done <ref> | cancel <ref> | reopen <ref>
trcli task move <ref> --project <ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `task add` | Adds a task to the current project at status `to do` | 2 on a dependency loop or a task made its own sub-task; 3 when the milestone is in another project |
| `task done` | Records the completion date; a repeating task gets its next occurrence | 5 when sub-tasks are unfinished and `--yes` is not given |
| `task cancel` | Cancels; a repeating task stops repeating | — |
| `task move` | Moves to another project; its milestone is cleared and remaining dependencies are shown as cross-project | — |
| `task list` | Shared verb; blocked tasks are marked and name what blocks them | — |
| (warning) | A task due after its milestone's date is accepted with a warning | 0 |

## Values

| Value | Rule |
|-------|------|
| `<title>` | 1–500 characters |
| `--repeat` | `daily`, `weekly`, `monthly`, or `<n>d` / `<n>w` / `<n>m` |
| list filters | `--status`, `--priority`, `--milestone`, `--responsible`, `--due-before`, `--about <ref>`, `--blocked`; `--sort due\|priority` |

## Example

```console
$ trcli task add "Rerun baseline with new split" --due 2026-10-20 --milestone mil-7c3k --about exp-2b6r
Added tsk-5w0h "Rerun baseline with new split" (to do, due 2026-10-20)
```
