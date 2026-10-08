# Contract: `trcli status`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 11; FR-051

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli status [--section <name>]...
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `status` | One view: due and overdue, runs waiting or failed, drafts by stage, open questions, reading backlog, expiring approvals, recent activity | — |
| `status --section` | The full list behind one section | 2 on unknown section |
| (empty workspace) | Suggests the first things to do | — |

## Values

| Value | Rule |
|-------|------|
| `--section` | `due`, `runs`, `drafts`, `questions`, `reading`, `approvals`, `activity` |

## Example

```console
$ trcli status
Due        2 overdue · 5 this week
Runs       1 waiting for you (run-6p0z, step "label")
Drafts     1 drafting · 1 submitted
Reading    14 to read · 2 reading
```
