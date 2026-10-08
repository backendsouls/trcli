# Contract: `trcli link, tag, note`

**Spec**: [Foundation and Architecture](../spec.md) — User Story 2; FR-015 to FR-017

**Conventions**: [grammar, global options, shared verbs](./cli-conventions.md) · [output and exit codes](./output-and-exit-codes.md)

## Synopsis

```text
trcli link add <ref> <ref> [--relation <text>]
trcli link rm <ref> <ref> [--relation <text>]
trcli link list <ref>
trcli <noun> tag <ref> <tag>... [--remove]
trcli <noun> note <ref> <text>
trcli tag list [--kind <kind>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `link add` | Links two records of any kind; shown from both ends | 3 when a record is not found; 2 when both are the same record or the link exists |
| `link rm` | Removes a link | 3 when the link does not exist |
| `link list` | Links of a record in both directions, grouped by relation | — |
| `<noun> tag` | Adds tags to a record, or removes them with `--remove` | 2 on an invalid tag name |
| `<noun> note` | Adds a dated note to a record | 2 on empty text |
| `tag list` | Every tag with the number of records carrying it | — |

## Values

| Value | Rule |
|-------|------|
| `--relation` | 1–50 characters; default `related` |
| `<tag>` | 1–50 characters of `a-z 0-9 - _`; upper case is lowered |
| `<text>` | 1–20,000 characters |

## Example

```console
$ trcli link add rq-4m2p ref-7k3f --relation addresses
Linked rq-4m2p "Does X improve Y?" ⟷ ref-7k3f "Attention Is All You Need" (addresses)
```
