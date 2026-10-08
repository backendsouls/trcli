# Contract: `trcli notebook`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 4; FR-020 to FR-023

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `nb`

## Synopsis

```text
trcli notebook add <text> [--date <date>] [--link <ref>]...
trcli notebook correct <entry-ref> <text>
trcli notebook list [--from <date>] [--to <date>] [--record <ref>] [--search <text>]
trcli notebook show <entry-ref>
trcli notebook export --from <date> --to <date> --to-file <file>
trcli notebook verify
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `notebook add` | Adds an entry with the date it refers to and the time it was written | 2 on empty text |
| `notebook correct` | Adds a correction that points to an earlier entry; both stay visible | 3 when the entry is not found |
| `notebook list` | Entries in time order, filtered | — |
| `notebook verify` | Checks that no entry was altered outside the tool | 6 `check_failed` |
| (no `edit`, no `rm`) | Entries cannot be changed or deleted | 2 `usage`, pointing to `notebook correct` |

## Values

| Value | Rule |
|-------|------|
| `<text>` | 1–20,000 characters |
| `--date` | the day the entry is about; defaults to today; may be in the past, not the future |

## Example

```console
$ trcli notebook add "Second batch contaminated; discarded samples 14-20." --link run-6p0z
Added nb-9a2k (2026-10-08 15:12), linked to run-6p0z
```
